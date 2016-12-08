#![feature(test)]

extern crate binary_heap;
extern crate rand;
extern crate test;

use rand::{Rand, Rng};
use test::Bencher;

use binary_heap::*;

fn random_vec<T: Rand>(len: usize) -> Vec<T> {
    let mut rng = rand::thread_rng();
    rng.gen_iter().take(len).collect()
}

fn binary_heap_from_bench(b: &mut Bencher, len: usize) {
    let v: Vec<usize> = random_vec(len);

    b.iter(|| {
        let v = v.clone();
        BinaryHeap::from(v)
    })
}

#[bench]
fn binary_heap_from_50000(b: &mut Bencher) {
    binary_heap_from_bench(b, 50000)
}

#[bench]
fn binary_heap_from_100000(b: &mut Bencher) {
    binary_heap_from_bench(b, 100000)
}

#[bench]
fn binary_heap_from_200000(b: &mut Bencher) {
    binary_heap_from_bench(b, 200000)
}

#[bench]
fn binary_heap_from_400000(b: &mut Bencher) {
    binary_heap_from_bench(b, 400000)
}

#[bench]
fn binary_heap_from_800000(b: &mut Bencher) {
    binary_heap_from_bench(b, 800000)
}