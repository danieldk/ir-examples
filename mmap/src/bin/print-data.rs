extern crate getopts;
extern crate memmap;
extern crate mmap;

use std::env::args;
use std::process;

use getopts::Options;
use memmap::{Mmap, Protection};

use mmap::*;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE INDEX", program);
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

    // Create a memory mapping for the given file.
    let mmap_file = or_exit(Mmap::open_path(&matches.free[0], Protection::Read));

    // Get the memory as a slice of bytes.
    let u8_data: &[u8] = unsafe { mmap_file.as_slice() };

    // Cast the slice of bytes to a slice of unsigned 64-bit integers.
    let u64_data = unsafe { bytes_to_typed::<u64>(u8_data) };

    // Parse the index given as an argument as a usize.
    let idx: usize = or_exit(matches.free[1].parse());

    // Print the value at the given index.
    println!("Value at index {}: {}", idx, u64_data[idx]);
}
