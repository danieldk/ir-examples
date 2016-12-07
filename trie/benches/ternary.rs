#![feature(test)]

extern crate rand;

extern crate test;

extern crate trie;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use rand::{Rng, weak_rng};
use test::Bencher;

use trie::{TernaryTree, TrieContains, TrieInsert};

fn sowpods_sample<R>(rng: &mut R, sample_size: usize) -> io::Result<Vec<String>>
    where R: Rng
{
    // let f = File::open("sowpods.txt")?;
    let f = File::open("sowpods.txt")?;
    let r = BufReader::new(f);

    let sample = rand::sample(rng, r.lines().map(Result::unwrap), sample_size);

    Ok(sample)
}

fn ternary_lookup_bench(b: &mut Bencher, dict_len: usize, lookup_len: usize) {
    let mut rng = weak_rng();
    let sample = sowpods_sample(&mut rng, dict_len).unwrap();

    let mut trie = TernaryTree::new(weak_rng());
    for word in &sample {
        trie.insert(word);
    }

    let lookup_sample = rand::sample(&mut rng, sample, lookup_len);

    b.iter(|| {
        for word in &lookup_sample {
            trie.contains(&word);
        }
    })
}

#[bench]
fn ternary_lookup_20000(b: &mut Bencher) {
    ternary_lookup_bench(b, 20000, 10000)
}

#[bench]
fn ternary_lookup_40000(b: &mut Bencher) {
    ternary_lookup_bench(b, 40000, 10000)
}

#[bench]
fn ternary_lookup_80000(b: &mut Bencher) {
    ternary_lookup_bench(b, 80000, 10000)
}

#[bench]
fn ternary_lookup_160000(b: &mut Bencher) {
    ternary_lookup_bench(b, 160000, 10000)
}