#![feature(test)]

extern crate boxed;
extern crate rand;
extern crate test;

use rand::Rng;

use test::Bencher;

const ARRAY_SIZE: usize = 1000000;

#[bench]
fn unboxed_sum(b: &mut Bencher) {
    let arr: Vec<usize> = (0..ARRAY_SIZE).collect();
    b.iter(|| boxed::sum_slice(&arr))
}

#[bench]
fn unboxed_sum_shuffled(b: &mut Bencher) {
    let mut arr: Vec<usize> = (0..ARRAY_SIZE).collect();
    let mut rng = rand::thread_rng();
    rng.shuffle(&mut arr);

    b.iter(|| boxed::sum_slice(&arr))
}

#[bench]
fn boxed_sum(b: &mut Bencher) {
    let arr: Vec<Box<usize>> = (0..ARRAY_SIZE).map(Box::new).collect();
    b.iter(|| boxed::sum_slice_boxed(&arr))
}

#[bench]
fn boxed_sum_shuffled(b: &mut Bencher) {
    let mut arr: Vec<Box<usize>> = (0..ARRAY_SIZE).map(Box::new).collect();
    let mut rng = rand::thread_rng();
    rng.shuffle(&mut arr);

    b.iter(|| boxed::sum_slice_boxed(&arr))
}
