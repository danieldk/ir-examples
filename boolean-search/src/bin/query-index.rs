#[macro_use]
extern crate boolean_search;
extern crate getopts;

use std::borrow::Cow;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
use std::process;

use getopts::Options;

use boolean_search::{DocIdentifiers, InvertedIndex, InvertedIndexFromText, MemoryIndex, or_exit};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] TITLE_FILE INDEX_FILE", program);
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

    if matches.free.len() != 2 {
        print_usage(&program, opts);
        process::exit(1);
    }

    // Read the titles file.
    let title_file = or_exit(File::open(&matches.free[0]));
    let doc_ids = or_exit(DocIdentifiers::from_buf_read(BufReader::new(title_file)));

    // Read the inverted index.
    let index_file = or_exit(File::open(&matches.free[1]));
    let index: MemoryIndex<u64> = or_exit(MemoryIndex::from_text(BufReader::new(index_file)));

    let input = stdin();
    for line in input.lock().lines() {
        let line = or_exit(line);

        // Store the terms in a vector.
        let terms: Vec<_> = line.split_whitespace().collect();

        // Get the posting lists for each term for terms that are in
        // the inverted index.
        let mut posting_lists: Vec<_> = terms.iter()
            .filter_map(|t| index.posting(t))
            .collect();

        // If the number of postings lists is smaller than the number of
        // terms, one of the terms was not found. This means that there
        // are no documents satisfying the query. Read the next line.
        if posting_lists.len() != terms.len() || posting_lists.is_empty() {
            continue;
        }

        // Sort the postings lists from smallest to largest.
        posting_lists.sort_by(|p1, p2| p1.len().cmp(&p2.len()));

        // Get the first postings list.
        let (first, rest) = posting_lists.split_first().unwrap();

        // Apply intersection to the postings lists.
        let result = rest.iter().fold(Cow::Borrowed(first), |acc, p| Cow::Owned(acc.intersect(p)));

        // Print the document ids and titles.
        for doc in result.iter() {
            match doc_ids.get(*doc as usize) {
                Some(title) => println!("{}: {}", doc, title),
                None => println!("{}: title unknown", doc),
            }
        }
    }
}
