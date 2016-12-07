use std::collections::HashMap;
use std::str::Chars;

use trie::*;

/// A trie that uses a HashMap for edges. This representation
/// is quite memory-inefficient.
pub struct SimpleTrie {
    accepting: bool,
    edges: HashMap<char, SimpleTrie>,
}

impl SimpleTrie {
    pub fn new() -> Self {
        SimpleTrie {
            accepting: false,
            edges: HashMap::new(),
        }
    }

    fn insert_chars(&mut self, mut chars: Chars) {
        match chars.next() {
            Some(ch) => {
                let entry = self.edges.entry(ch).or_insert(SimpleTrie::new());
                entry.insert_chars(chars);
            }
            None => self.accepting = true,
        }
    }

    fn prefix_node(&self, mut chars: Chars) -> Option<&SimpleTrie> {
        match chars.next() {
            Some(ch) => {
                match self.edges.get(&ch) {
                    Some(node) => node.prefix_node(chars),
                    None => None,
                }
            }
            None => Some(self),
        }
    }
}

impl TrieContains for SimpleTrie {
    fn contains(&self, word: &str) -> bool {
        match self.prefix_node(word.chars()) {
            Some(trie) => trie.accepting,
            None => false, 
        }
    }
}

impl TriePrefixIter for SimpleTrie {
    fn prefix_iter<'a>(&'a self, prefix: &str) -> Box<Iterator<Item = String> + 'a> {
        match self.prefix_node(prefix.chars()) {
            Some(node) => Box::new(Iter::new(node, prefix.to_owned())),
            None => Box::new(Iter::empty()),
        }
    }
}

impl TrieInsert for SimpleTrie {
    fn insert(&mut self, s: &str) {
        assert!(s.len() > 0, "Empty key");
        self.insert_chars(s.chars());
    }
}

/// Work items for iteration.
struct StringNodePair<'a> {
    s: String,
    node: &'a SimpleTrie,
}

/// An iterator over the trie.
struct Iter<'a> {
    work: Vec<StringNodePair<'a>>,
}

impl<'a> Iter<'a> {
    /// The empty iterator.
    fn empty() -> Self {
        Iter { work: Vec::new() }
    }

    /// An iterator that starts at a particular node with a particular
    /// prefix.
    fn new(node: &'a SimpleTrie, prefix: String) -> Self {
        Iter {
            work: vec![StringNodePair {
                           s: prefix,
                           node: node,
                       }],
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let pair = try_ok!(self.work.pop());

            // Add reachable nodes as work.
            for (ch, to_node) in pair.node.edges.iter() {
                let mut new_str = pair.s.clone();
                new_str.push(*ch);
                self.work.push(StringNodePair {
                    node: to_node,
                    s: new_str,
                })
            }

            if pair.node.accepting {
                return Some(pair.s);
            }
        }
    }
}
