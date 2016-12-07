extern crate num;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate rand;

#[macro_use]
mod macros;

mod array;
pub use array::ArrayTrie;

mod simple;
pub use simple::SimpleTrie;

mod ternary;
pub use ternary::TernaryTree;

mod trie;
pub use trie::{TrieContains, TriePrefixIter, TrieInsert};

mod util;
pub use util::or_exit;

#[cfg(test)]

mod test;
