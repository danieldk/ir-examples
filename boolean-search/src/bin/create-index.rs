#[macro_use]
extern crate boolean_search;
extern crate stdinout;
extern crate conllx;
extern crate getopts;

use std::env::args;
use std::io::BufWriter;
use std::process;

use conllx::Features;
use stdinout::*;
use getopts::Options;

use boolean_search::{InvertedIndexMut, InvertedIndexToText, MemoryIndex, or_exit};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] EXPR [INPUT_FILE] [OUTPUT_FILE]",
                        program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = or_exit(opts.parse(&args[1..]));

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.len() > 2 {
        print_usage(&program, opts);
        process::exit(1);
    }

    let input = Input::from(matches.free.get(0).map(String::as_str));
    let reader = conllx::Reader::new(or_exit(input.buf_read()));

    let output = Output::from(matches.free.get(1).map(String::as_str));
    let mut writer = BufWriter::new(or_exit(output.write()));

    let mut index = MemoryIndex::new();

    for sentence in reader {
        let sentence = or_exit(sentence);

        // Get the document identifier. We can safely assume that all the
        // tokens in a sentence belong to the same document.
        let token = ok_or_continue!(sentence.as_tokens().get(0));
        let doc_str = ok_or_continue!(token.features().map(Features::as_str));
        let doc: usize = or_exit(doc_str.parse());

        // Get the lemmas and add to the inverted index.
        for token in &sentence {
            let lemma = ok_or_continue!(token.lemma());
            index.add_term(lemma, doc);
        }
    }

    // Write out the inverted index.
    or_exit(index.to_text(&mut writer));
}
