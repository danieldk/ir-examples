use quickcheck::{Arbitrary, Gen};
use rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Normal};

use std::collections::HashSet;
use std::iter::FromIterator;

use super::*;

pub trait Trie: TrieContains + TrieInsert + TriePrefixIter {}
impl<T> Trie for T where T: TrieContains + TrieInsert + TriePrefixIter {}

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

impl From<SmallAlphabet> for char {
    fn from(a: SmallAlphabet) -> Self {
        match a {
            SmallAlphabet::A => 'a',
            SmallAlphabet::B => 'b',
            SmallAlphabet::C => 'c',
            SmallAlphabet::D => 'd',
        }
    }
}

impl FromIterator<SmallAlphabet> for String {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = SmallAlphabet>
    {
        iter.into_iter().map(Into::<char>::into).collect()
    }
}

quickcheck! {
    fn array_contains_prop(data1: Vec<Vec<SmallAlphabet>>, data2: Vec<Vec<SmallAlphabet>>) -> bool {
        contains_test(Box::new(ArrayTrie::new()), data1, data2)
    }
}

quickcheck! {
    fn array_prefix_prop(data: Vec<Vec<SmallAlphabet>>) -> bool {
        prefix_test(Box::new(ArrayTrie::new()), data)
    }
}

quickcheck! {
    fn simple_prefix_prop(data: Vec<Vec<SmallAlphabet>>) -> bool {
        prefix_test(Box::new(SimpleTrie::new()), data)
    }
}

quickcheck! {
    fn simple_contains_prop(data1: Vec<Vec<SmallAlphabet>>, data2: Vec<Vec<SmallAlphabet>>) -> bool {
        contains_test(Box::new(SimpleTrie::new()), data1, data2)
    }
}

quickcheck! {
    fn ternary_prefix_prop(data: Vec<Vec<SmallAlphabet>>) -> bool {
        prefix_test(Box::new(TernaryTree::new(rand::weak_rng())), data)
    }
}

quickcheck! {
    fn ternary_contains_prop(data1: Vec<Vec<SmallAlphabet>>, data2: Vec<Vec<SmallAlphabet>>) -> bool {
        contains_test(Box::new(TernaryTree::new(rand::weak_rng())), data1, data2)
    }
}

quickcheck! {
    fn ternary_prefix_prop_u8(data: Vec<Vec<SmallAlphabet>>) -> bool {
        prefix_test(Box::new(TernaryTree::<u8>::new_with_prio(rand::weak_rng())), data)
    }
}

quickcheck! {
    fn ternary_contains_prop_u8(data1: Vec<Vec<SmallAlphabet>>, data2: Vec<Vec<SmallAlphabet>>) -> bool {
        contains_test(Box::new(TernaryTree::<u8>::new_with_prio(rand::weak_rng())), data1, data2)
    }
}


fn small_alphabet_to_string<I, B>(from: I) -> B
    where I: IntoIterator<Item = Vec<SmallAlphabet>>,
          B: FromIterator<String>
{
    from.into_iter()
        .filter(|w| !w.is_empty())
        .map(FromIterator::<SmallAlphabet>::from_iter)
        .collect()
}

fn contains_test(mut trie: Box<Trie>,
                 data1: Vec<Vec<SmallAlphabet>>,
                 data2: Vec<Vec<SmallAlphabet>>)
                 -> bool {
    let data1: HashSet<_> = small_alphabet_to_string(data1);
    let data2: HashSet<_> = small_alphabet_to_string(data2);
    let diff = &data2 - &data1;

    for word in &data1 {
        trie.insert(word);
    }

    for word in &data1 {
        if !trie.contains(word) {
            return false;
        }
    }

    for word in &diff {
        if trie.contains(word) {
            return false;
        }
    }

    true
}

fn prefix_test(mut trie: Box<Trie>, data: Vec<Vec<SmallAlphabet>>) -> bool {
    let data: Vec<_> = small_alphabet_to_string(data);

    if data.is_empty() {
        return true;
    }

    for word in &data {
        trie.insert(&word);
    }

    let prefix = random_prefix(&data);

    let found_prefixes: HashSet<_> = trie.prefix_iter(&prefix).collect();
    let correct_prefixes: HashSet<_> =
        data.iter().filter(|w| w.starts_with(&prefix)).cloned().collect();

    found_prefixes == correct_prefixes
}

fn random_prefix(data: &[String]) -> String {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0, data.len());

    let s: Vec<_> = data[idx].chars().collect();

    // Get a random and valid length, biased towards short prefixes.
    let mut len;
    loop {
        let normal = Normal::new(0., 2.);
        len = normal.ind_sample(&mut rng).abs().round() as usize + 1;

        if len <= s.len() {
            break;
        }
    }

    s.into_iter().take(len).collect()
}
