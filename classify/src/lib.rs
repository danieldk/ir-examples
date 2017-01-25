extern crate itertools;

extern crate num_traits;

#[macro_use]
mod macros;

mod features;
pub use features::{FeatureCounter, FormFeature, HashingVectorBuilder, LemmaFeature,
    SignedHashingVectorBuilder, SparseVectorBuilder};

mod filters;
pub use filters::{PTBStopwordFilter, StopwordFilter};

mod numberer;
pub use numberer::Numberer;

mod sparse_vector;
pub use sparse_vector::{Idx, SparseVector};

mod svm;
pub use svm::LibSVMWriter;

mod util;
pub use util::or_exit;

#[cfg(test)]
mod tests;