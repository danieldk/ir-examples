extern crate getopts;
extern crate ndarray;
extern crate ndarray_rand;
extern crate rand;

use std::env::args;
use std::f64;
use std::fmt::Display;
use std::process;

use getopts::Options;
use ndarray::{Array, Ix1};
use ndarray_rand::RandomExt;
use rand::{Rng, weak_rng};
use rand::distributions::Range;

/// Compute the Euclidean distance between two vectors.
fn euclidean_distance(v: &Array<f64, Ix1>, v2: &Array<f64, Ix1>) -> f64 {
    let diff = v - v2;
    (diff.dot(&diff)).sqrt()
}

/// Generate `n` vectors of dimensionality `d`.
fn generate_vectors<R>(d: usize, n: usize, rng: &mut R) -> Vec<Array<f64, Ix1>>
    where R: Rng
{
    let range: Range<f64> = Range::new(0., 1.);

    let mut vecs = Vec::new();
    for _ in 0..n {
        vecs.push(Array::random_using((d,), range, rng));
    }

    vecs
}

fn min_max_distance(vecs: &[Array<f64, Ix1>]) -> (f64, f64) {
    let mut max = f64::MIN;
    let mut min = f64::MAX;

    for i in 0..vecs.len() {
        for j in (i + 1)..vecs.len() {
            let dist = euclidean_distance(&vecs[i], &vecs[j]);
            if dist > max {
                max = dist;
            }
            if dist < min {
                min = dist;
            }
        }
    }

    (min, max)
}

pub fn or_exit<T, E: Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e: E| -> T {
        println!("Error: {}", e);
        process::exit(1)
    })
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] MAX_DIMS POINTS", program);
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

    let max_dims = or_exit(matches.free[0].parse());
    let points = or_exit(matches.free[1].parse());

    let mut rng = weak_rng();

    for d in 1..max_dims {
        let vecs = generate_vectors(d, points, &mut rng);
        let (min, max) = min_max_distance(&vecs);
        println!("d = {}, min = {1:.2}, max = {2:.2}, ratio = {3:.2}",
                 d,
                 min,
                 max,
                 (max - min) / min);
    }
}
