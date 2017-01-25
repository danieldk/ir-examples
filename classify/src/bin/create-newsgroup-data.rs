#[macro_use]
extern crate classify;
extern crate conllx;
extern crate fnv;
extern crate getopts;
extern crate walkdir;

use std::borrow::Cow;
use std::env::args;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::mem::size_of;
use std::process;

use conllx::Reader;
use fnv::FnvBuildHasher;
use getopts::{Matches, Options};
use walkdir::WalkDir;

use classify::*;

fn tf_from_str(tf_str: &str) -> fn(f64) -> f64 {
    match tf_str {
        "btf" => btf,
        "tf" => tf,
        "stf" => stf,
        _ => {
            stderr!("Unknown TF function: {}", tf_str);
            process::exit(1);
        }
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] NEWS_DIR LIBLINEAR_OUTPUT", program);
    stderr!("{}", opts.usage(&brief));
}

pub fn construct_vectors(dir: &str,
                         n_features: usize,
                         use_lemmas: bool,
                         filter: &StopwordFilter)
                         -> (Vec<SparseVector<usize, f64>>, Vec<usize>, Numberer<String>) {
    let extension = Some(OsStr::new("conll"));

    let mut numberer = Numberer::new(0);
    let mut vecs = Vec::new();
    let mut labels = Vec::new();

    for entry in WalkDir::new(dir) {
        let entry = or_exit(entry);
        if entry.file_type().is_file() && entry.path().extension() == extension {
            let group = entry.path()
                .components()
                .rev()
                .nth(1)
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap();

            let label = numberer.add(group.to_owned());
            labels.push(label);

            let mut vec_builder = HashingVectorBuilder::new(FnvBuildHasher::default(), n_features);

            let r = Reader::new(BufReader::new(or_exit(File::open(entry.path()))));
            for sentence in r {
                for token in or_exit(sentence) {
                    if filter.is_stopword(token.pos().unwrap_or("_"), token.lemma().unwrap_or("_")) {
                        continue;
                    }

                    vec_builder.count(FormFeature::new(Cow::Borrowed(token.form().unwrap_or("_"))));

                    if use_lemmas {
                        vec_builder.count(LemmaFeature::new(Cow::Borrowed(token.lemma().unwrap_or("_"))));
                    }
                }
            }

            vecs.push(vec_builder.build());
        }
    }

    (vecs, labels, numberer)
}

pub fn n_features(matches: &Matches) -> usize {
    let exponent = matches.opt_str("e").map(|v| or_exit(v.parse())).unwrap_or(18);
    let usize_bits = size_of::<usize>() * 8;
    if exponent > usize_bits - 1 {
        stderr!("Maximum exponent on this machine is {}", usize_bits - 1);
        std::process::exit(1);
    }

    2usize.pow(exponent as u32)
}


fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("e",
                "exponent",
                "use 2^exponent features (default: 18)",
                "EXPONENT");
    opts.optflag("l", "lemma", "add lemma features");
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("t",
                "tf",
                "Term frequency function: btf, tf, stf (default: stf)",
                "FUNCTION");
    let matches = or_exit(opts.parse(&args[1..]));

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let n_features = n_features(&matches);
    stderr!("Using kernel with {} features", n_features);

    let tf_str = matches.opt_str("t").unwrap_or("stf".to_owned());
    let tf = tf_from_str(&tf_str);

    if matches.free.len() != 2 {
        print_usage(&program, opts);
        process::exit(1);
    }

    let filter = PTBStopwordFilter;
    let use_lemmas = matches.opt_present("l");
    stderr!("Using lemmas: {}", use_lemmas);

    let (mut vecs, labels, _) =
        construct_vectors(&matches.free[0], n_features, use_lemmas, &filter);

    // Modified with two ideas from Khuan Yu: permit other TF measures,
    // normalize document vectors.

    tf_idfs(&mut vecs, tf, n_features);

    normalize(&mut vecs);

    let mut writer = LibSVMWriter::new(BufWriter::new(or_exit(File::create(&matches.free[1]))));

    for (label, vec) in labels.iter().zip(vecs.iter()) {
        or_exit(writer.write(*label, vec));
    }
}

fn normalize(vecs: &mut Vec<SparseVector<usize, f64>>) {
    for vec in vecs.iter_mut() {
        let l2norm = vec.iter().fold(0f64, |acc, (_, v)| acc + v * v).sqrt();

        for (_, val) in vec.iter_mut() {
            *val /= l2norm;
        }
    }
}

fn tf_idfs(vecs: &mut Vec<SparseVector<usize, f64>>, tf: fn(f64) -> f64, n_features: usize) {
    let mut dfs = vec![0f64; n_features];

    for vec in vecs.iter() {
        for (idx, val) in vec.iter() {
            if *val != 0f64 {
                dfs[*idx] += 1f64;
            }
        }
    }

    for df in dfs.iter_mut() {
        if *df != 0f64 {
            *df = (vecs.len() as f64 / *df).ln();
        }
    }

    for vec in vecs.iter_mut() {
        for (idx, val) in vec.iter_mut() {
            *val = tf(*val) * dfs[*idx as usize];
        }
    }
}

/// Binary term frequency.
pub fn btf(freq: f64) -> f64 {
    if freq > 0f64 { 1.0 } else { 0.0 }
}

/// Regular term frequency.
pub fn tf(freq: f64) -> f64 {
    freq
}

/// Sublinear term frequency
pub fn stf(freq: f64) -> f64 {
    if freq > 0f64 {
        (1_f64 + freq).ln()
    } else {
        0_f64
    }
}