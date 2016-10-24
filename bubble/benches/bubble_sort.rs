#![feature(test)]

extern crate bubble;
extern crate rand;
extern crate test;

use rand::{Rand, Rng};
use test::Bencher;

fn random_vec<T: Rand>(len: usize) -> Vec<T> {
    let mut rng = rand::thread_rng();
    rng.gen_iter().take(len).collect()
}

fn bubble_sort_bench(b: &mut Bencher, len: usize) {
    let unsorted: Vec<usize> = random_vec(len);

    b.iter(|| {
        let mut arr = unsorted.clone();
        bubble::bubble_sort(&mut arr);
    })
}

#[bench]
fn bubble_sort_100(b: &mut Bencher) {
    bubble_sort_bench(b, 100)
}

#[bench]
fn bubble_sort_200(b: &mut Bencher) {
    bubble_sort_bench(b, 200)
}

#[bench]
fn bubble_sort_400(b: &mut Bencher) {
    bubble_sort_bench(b, 400)
}

#[bench]
fn bubble_sort_800(b: &mut Bencher) {
    bubble_sort_bench(b, 800)
}

#[bench]
fn bubble_sort_1600(b: &mut Bencher) {
    bubble_sort_bench(b, 1600)
}

#[bench]
fn bubble_sort_3200(b: &mut Bencher) {
    bubble_sort_bench(b, 3200)
}
