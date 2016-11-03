#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
extern crate rand;

use std::cmp::min;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct SuffixArray<T: Ord> {
    data: Vec<T>,
    sarr: Vec<usize>,
}

enum Bound {
    LowerBound,
    UpperBound,
}

impl<T: Ord> SuffixArray<T> {
    pub fn new<V>(data: V) -> SuffixArray<T>
        where V: Into<Vec<T>>
    {
        let data = data.into();

        // Construct an array with the same length as the data array
        // with indices 0..n-1.
        let mut sarr: Vec<_> = (0..data.len()).collect();

        // Sort the array, thereby constructing the suffix array. Note
        // that we compare the array slices starting at the given indices.
        //
        // Note that this sorts in n^2 log(n) time. For real applications,
        // use a fast algorithm like SA-IS.
        sarr.sort_by(|&i1, &i2| data[i1..].cmp(&data[i2..]));

        SuffixArray {
            data: data,
            sarr: sarr,
        }
    }

    fn bound(&self, bound: Bound, needle: &[T]) -> usize {
        let result = self.sarr.binary_search_by(|&idx| {
            let upper = min(self.data.len(), idx + needle.len());

            // Note: the binary_search_by documentation states: "if no match
            // is found then Err is returned, containing the index where a
            // matching element could be inserted"
            //
            // This means that we can get the lower bound by pretending that
            // all matching elements are greater and the upper bound by
            // pretending that all matching elements are less.
            match self.data[idx..upper].cmp(needle) {
                Ordering::Equal => {
                    match bound {
                        Bound::LowerBound => Ordering::Greater,
                        Bound::UpperBound => Ordering::Less,
                    }
                }
                ordering => ordering,
            }
        });

        match result {
            Ok(_) => unreachable!(),
            Err(lower) => lower,
        }
    }

    pub fn contains(&self, needle: &[T]) -> bool {
        self.sarr
            .binary_search_by(|&idx| {
                // We don't want to compare 'needle' with the complete suffix
                // starting at 'idx', but only the prefix of the length of
                // 'needle'. However, 'needle' can be larger than the suffix
                // at 'index'. So, we have to avoid slicing beyond the end.
                let upper = min(self.data.len(), idx + needle.len());

                // Compare the prefix of the suffix and the needle.
                self.data[idx..upper].cmp(needle)
            })
            .is_ok()
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn find(&self, needle: &[T]) -> &[usize] {
        let lower = self.lower_bound(needle);
        let upper = self.upper_bound(needle);

        return &self.sarr[lower..upper];
    }

    fn lower_bound(&self, needle: &[T]) -> usize {
        self.bound(Bound::LowerBound, needle)
    }

    fn upper_bound(&self, needle: &[T]) -> usize {
        self.bound(Bound::UpperBound, needle)
    }

    pub fn positions(&self) -> &[usize] {
        &self.sarr
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};
    use rand;
    use rand::Rng;
    use rand::distributions::{IndependentSample, Normal};
    use std::str;

    use super::*;

    #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
    enum SmallAlphabet {
        A,
        B,
        C,
        D,
    }

    impl Arbitrary for SmallAlphabet {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            match g.gen_range(0, 4) {
                0 => SmallAlphabet::A,
                1 => SmallAlphabet::B,
                2 => SmallAlphabet::C,
                3 => SmallAlphabet::D,
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_contains() {
        let example = "bananabread";
        let sarr = SuffixArray::new(example);

        // Check all substrings
        for i in 0..example.len() + 1 {
            for j in i + 1..example.len() + 1 {
                assert!(sarr.contains(example[i..j].as_bytes()));
            }
        }

        // Things that should not be there
        assert!(!sarr.contains("x".as_bytes()));
        assert!(!sarr.contains("breads".as_bytes()));
        assert!(!sarr.contains("bn".as_bytes()));
    }

    #[test]
    fn test_find() {
        let sarr = SuffixArray::new("bananabread");
        let positions = sarr.find("a".as_bytes());
        assert_eq!(positions, &[5, 9, 3, 1]);

        let positions = sarr.find("na".as_bytes());
        assert_eq!(positions, &[4, 2]);

        let positions = sarr.find("foobar".as_bytes());
        assert_eq!(positions, &[]);

        let positions = sarr.find("".as_bytes());
        assert_eq!(positions, &[5, 9, 3, 1, 0, 6, 10, 8, 4, 2, 7]);
    }

    quickcheck! {
        fn contains_prop(data: Vec<SmallAlphabet>) -> bool {
            if data.is_empty() {
                return true
            }

            let sarr = SuffixArray::new(data.clone());

            // Note: we could check all subsequences as well.
            let seq = random_subsequence(&data);

            sarr.contains(seq)
        }
    }

    quickcheck! {
        // The order of suffixes in the suffix array should be
        // as if we extract all the suffixes and sort them.
        fn sorted_prop(data: Vec<SmallAlphabet>) -> bool {
            let sarr = SuffixArray::new(data.clone());

            let sarr_suffixes: Vec<_> = sarr.positions()
                .iter()
                .map(|&i| &sarr.data()[i..])
                .collect();

            sarr_suffixes == sorted_suffixes(&data)
        }
    }

    quickcheck! {
        fn find_prop(data: Vec<SmallAlphabet>) -> bool {
            if data.is_empty() {
                return true
            }

            let sarr = SuffixArray::new(data.clone());

            // Note: we could check all subsequences as well.
            let seq = random_subsequence(&data);

            let suffixes = sorted_suffixes_index(&data);

            // Find positions using linear search.
            let match_indices : Vec<usize> = suffixes
                .iter()
                .filter_map(|&(idx, suffix)| {
                    if suffix.starts_with(seq) {
                        Some(idx)
                    } else {
                        None
                    }
                })
                .collect();

            match_indices == sarr.find(&seq)
        }
    }

    fn sorted_suffixes<T>(data: &[T]) -> Vec<&[T]>
        where T: Ord
    {
        let mut suffixes: Vec<_> = (0..data.len())
            .map(|i| &data[i..])
            .collect();
        suffixes.sort();
        suffixes
    }

    fn sorted_suffixes_index<T>(data: &[T]) -> Vec<(usize, &[T])>
        where T: Ord
    {
        let mut suffixes: Vec<_> = (0..data.len())
            .map(|i| (i, &data[i..]))
            .collect();
        suffixes.sort_by(|&(_, s1), &(_, s2)| s1.cmp(s2));
        suffixes
    }

    fn random_subsequence<T>(data: &[T]) -> &[T] {
        // Get a random starting index.
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0, data.len());
        let max_len = data.len() - idx;

        // Get a random and valid length, biased towards short sequences.
        let mut len;
        loop {
            let normal = Normal::new(0., 2.);
            len = normal.ind_sample(&mut rng).abs().round() as usize + 1;

            if len <= max_len {
                break;
            }
        }

        &data[idx..idx + len]
    }
}
