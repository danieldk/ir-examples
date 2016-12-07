#[macro_use]
extern crate trie;
extern crate getopts;
extern crate rand;

use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, stdin};
use std::process;
use std::collections::BTreeSet;

use getopts::Options;

use trie::*;

#[derive(Clone, Copy, Debug)]
enum TrieType {
    Array,
    Simple,
    Ternary,
}

impl TrieType {
    pub fn from_str(type_str: &str) -> Option<Self> {
        match type_str {
            "simple" => Some(TrieType::Simple),
            "array" => Some(TrieType::Array),
            "ternary" => Some(TrieType::Ternary),
            _ => None,
        }
    }
}

pub trait Trie: TriePrefixIter + TrieInsert {}
impl<T> Trie for T where T: TriePrefixIter + TrieInsert {}

fn load_dictionary(filename: &str, trie_type: TrieType) -> (Box<Trie>, Box<Trie>) {
    let f = or_exit(File::open(filename));
    let reader = BufReader::new(f);


    let (mut trie, mut reverse_trie): (Box<Trie>, Box<Trie>) = match trie_type {
        TrieType::Simple => (Box::new(SimpleTrie::new()), Box::new(SimpleTrie::new())),
        TrieType::Array => (Box::new(ArrayTrie::new()), Box::new(ArrayTrie::new())),
        TrieType::Ternary => {
            (Box::new(TernaryTree::new(rand::weak_rng())),
             Box::new(TernaryTree::new(rand::weak_rng())))
        }
    };

    for line in reader.lines() {
        let line = or_exit(line);
        trie.insert(&line);
        let line_reverse = reverse(line);
        reverse_trie.insert(&line_reverse)
    }

    (trie, reverse_trie)
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] WORDS", program);
    print!("{}", opts.usage(&brief));
}

fn reverse<S>(s: S) -> String
    where S: Into<String>
{
    s.into().chars().rev().collect()
}

fn process_query(query: &str, trie: &Box<Trie>, reverse_trie: &Box<Trie>) {
    let parts: Vec<_> = query.split("*").collect();

    if parts.len() != 2 {
        stderr!("Use exactly one wildcard");
        return;
    }

    let results: BTreeSet<String> = if parts[1].is_empty() {
        trie.prefix_iter(parts[0]).collect()
    } else if parts[0].is_empty() {
        let suffix = reverse(parts[1]);
        reverse_trie.prefix_iter(&suffix).map(reverse).collect()
    } else {
        let suffix = parts[1];
        let min_len = parts[0].len() + suffix.len();

        // Thanks to Kuan Yu for pointing out a bug in handling e.g. a*a and a performance
        // improvement. Note: the implementation can be made faster by iterating over the results
        // for the suffix (when the suffix is longer).
        trie.prefix_iter(parts[0])
            .filter(|word| word.len() >= min_len && word.ends_with(suffix))
            .collect()
    };

    for result in results {
        println!("{}", result);
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("t",
                "trie",
                "Type of trie: simple, array, ternary (default: ternary)",
                "TYPE");
    let matches = or_exit(opts.parse(&args[1..]));

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let trie_str = matches.opt_str("t").unwrap_or("ternary".to_owned());
    let trie_type = TrieType::from_str(&trie_str).unwrap_or_else(|| {
        stderr!("Unknown trie type: {}", trie_str);
        process::exit(1)
    });

    if matches.free.len() != 1 {
        print_usage(&program, opts);
        process::exit(1);
    }

    let (trie, reverse_trie) = load_dictionary(&matches.free[0], trie_type);

    let input = stdin();
    for line in input.lock().lines() {
        let line = or_exit(line);
        process_query(&line, &trie, &reverse_trie);
    }
}
