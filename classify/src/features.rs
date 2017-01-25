use std::borrow::Cow;
use std::hash::{BuildHasher, Hash, Hasher};

use num_traits::{Num, One, Signed};

use super::{Idx, SparseVector};

const U64_MSB: u64 = 1 << 63;
const U64_MSB_MASK: u64 = !(1 << 63);

/// Trait for data structures that count occurences of
/// features in training instances. Implementations of
/// a counter will typically increment the value of a
/// feature when it occurs more than once.
pub trait FeatureCounter<V> {
    fn count(&mut self, value: V);
}

/// Trait for data structures that return (sparse) feature
/// vectors.
pub trait SparseVectorBuilder<I, V>
    where I: Idx,
          V: Num
{
    fn build(self) -> SparseVector<I, V>;
}

/// Feature vector builder that uses a hash kernel.
pub struct HashingVectorBuilder<V, H>
    where V: Num,
          H: BuildHasher
{
    vec: SparseVector<usize, V>,
    len: usize,
    hash: H,
}

impl<V, H> HashingVectorBuilder<V, H>
    where V: Num,
          H: BuildHasher
{
    /// Construct a hashing builder using the given hash and with the
    /// provided size. Ideally, the size is a power of two, such that
    /// every feature becomes equally likely.
    pub fn new(hash: H, len: usize) -> Self {
        HashingVectorBuilder {
            vec: SparseVector::new(),
            len: len,
            hash: hash,
        }
    }
}

impl<V, H> SparseVectorBuilder<usize, V> for HashingVectorBuilder<V, H>
    where V: Num,
          H: BuildHasher
{
    fn build(self) -> SparseVector<usize, V> {
        self.vec
    }
}

impl<V, H, HV> FeatureCounter<HV> for HashingVectorBuilder<V, H>
    where V: Copy + Num,
          H: BuildHasher,
          HV: Hash
{
    fn count(&mut self, value: HV) {
        let mut hasher = self.hash.build_hasher();

        value.hash(&mut hasher);

        let idx = hasher.finish() as usize % self.len;
        let v = *self.vec.get(idx) + One::one();
        *self.vec.get(idx) = v;
    }
}

/// Feature vector builder that uses feature hashing.
///
/// In contrast to `HashingVectorBuilder`, this implementation
/// uses the sign of the hash to determine wether the feature
/// value is positive or negative.
pub struct SignedHashingVectorBuilder<V, H>
    where V: Num,
          H: BuildHasher
{
    vec: SparseVector<usize, V>,
    len: usize,
    hash: H,
}

impl<V, H> SignedHashingVectorBuilder<V, H>
    where V: Signed,
          H: BuildHasher
{
    /// Construct a hashing builder using the given hash and with the
    /// provided size. Ideally, the size is a power of two, such that
    /// every feature becomes equally likely.
    pub fn new(hash: H, len: usize) -> Self {
        SignedHashingVectorBuilder {
            vec: SparseVector::new(),
            len: len,
            hash: hash,
        }
    }
}

impl<V, H> SparseVectorBuilder<usize, V> for SignedHashingVectorBuilder<V, H>
    where V: Signed,
          H: BuildHasher
{
    fn build(self) -> SparseVector<usize, V> {
        self.vec
    }
}

impl<V, H, HV> FeatureCounter<HV> for SignedHashingVectorBuilder<V, H>
    where V: Copy + Signed,
          H: BuildHasher,
          HV: Hash
{
    fn count(&mut self, value: HV) {
        let mut hasher = self.hash.build_hasher();

        value.hash(&mut hasher);

        let hash = hasher.finish();
        let idx = (hash & U64_MSB_MASK) as usize % self.len;

        let v = if hash & U64_MSB == U64_MSB {
            *self.vec.get(idx) - One::one()
        } else {
            *self.vec.get(idx) + One::one()
        };
        *self.vec.get(idx) = v;
    }
}

/// Word form feature.
pub struct FormFeature<'a> {
    form: Cow<'a, str>,
}

impl<'a> FormFeature<'a> {
    pub fn new(form: Cow<'a, str>) -> Self {
        FormFeature { form: form }
    }
}

impl<'a> Hash for FormFeature<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        "form".hash(state);
        self.form.hash(state);
    }
}

/// Lemma feature.
pub struct LemmaFeature<'a> {
    lemma: Cow<'a, str>,
}

impl<'a> LemmaFeature<'a> {
    pub fn new(lemma: Cow<'a, str>) -> Self {
        LemmaFeature { lemma: lemma }
    }
}

impl<'a> Hash for LemmaFeature<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        "lemma".hash(state);
        self.lemma.hash(state);
    }
}