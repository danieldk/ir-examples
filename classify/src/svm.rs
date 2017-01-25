use std::fmt::Display;
use std::io;
use std::io::{BufWriter, Write};

use itertools::Itertools;
use num_traits::{Num, One};

use super::{Idx, SparseVector};

pub struct LibSVMWriter<W>
    where W: Write
{
    writer: BufWriter<W>,
}

impl<W> LibSVMWriter<W>
    where W: Write
{
    pub fn new(writer: BufWriter<W>) -> Self {
        LibSVMWriter { writer: writer }
    }

    pub fn write<I, V>(&mut self, label: usize, vec: &SparseVector<I, V>) -> io::Result<()>
        where I: Copy + Display + Idx,
              V: Display + Num
    {
        let fv_str = vec.iter().map(|(f, v)| format!("{}:{}", *f + One::one(), v)).join(" ");

        write!(self.writer, "{} {}\n", label, fv_str)
    }
}