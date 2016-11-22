extern crate itertools;

#[macro_use]
mod macros;

mod docid;
pub use docid::DocIdentifiers;

mod index;
pub use index::{InvertedIndex, InvertedIndexFromText, InvertedIndexToText, InvertedIndexMut,
                Posting, TextReadError};

mod memory;
pub use memory::MemoryIndex;

mod util;
pub use util::{is_sorted_uniq, or_exit};
