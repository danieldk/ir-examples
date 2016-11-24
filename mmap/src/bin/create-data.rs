extern crate getopts;
extern crate mmap;

use std::env::args;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process;

use getopts::Options;

use mmap::*;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE N", program);
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

    // Parse n as an unsigned 64-bit integer.
    let n: u64 = or_exit(matches.free[1].parse());

    let f = or_exit(File::create(&matches.free[0]));
    let mut w = BufWriter::new(f);

    for i in 0..n {
        // Cast unsigned to slice of bytes. Note: the underlying data is
        // not modified.
        let bytes: [u8; 8] = unsafe { std::mem::transmute(i) };

        // Write the data to disk.
        or_exit(w.write(&bytes));
    }
}
