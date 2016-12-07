use std::str::Chars;

use trie::*;

struct Edge {
    ch: char,
    to: Box<ArrayTrie>,
}

/// A trie that stores edges using dynamic arrays.
pub struct ArrayTrie {
    accepting: bool,
    edges: Vec<Edge>,
}

impl ArrayTrie {
    /// Construct a new trie.
    pub fn new() -> Self {
        ArrayTrie {
            accepting: false,
            edges: Vec::new(),
        }
    }

    fn insert_chars(&mut self, mut chars: Chars) {
        match chars.next() {
            Some(ch) => {
                let mut edge = match self.edges.binary_search_by_key(&ch, |e| e.ch) {
                    Ok(idx) => &mut self.edges[idx],
                    Err(idx) => {
                        self.edges.insert(idx,
                                          Edge {
                                              ch: ch,
                                              to: Box::new(ArrayTrie::new()),
                                          });
                        &mut self.edges[idx]
                    }
                };

                edge.to.insert_chars(chars);
            }
            None => self.accepting = true,
        }
    }

    fn prefix_node(&self, mut chars: Chars) -> Option<&ArrayTrie> {
        match chars.next() {
            Some(ch) => {
                match self.edges.binary_search_by_key(&ch, |e| e.ch) {
                    Ok(idx) => self.edges[idx].to.prefix_node(chars),
                    Err(_) => None,
                }
            }
            None => Some(self),
        }
    }
}

impl TrieContains for ArrayTrie {
    fn contains(&self, word: &str) -> bool {
        match self.prefix_node(word.chars()) {
            Some(trie) => trie.accepting,
            None => false,
        }
    }
}

impl TriePrefixIter for ArrayTrie {
    fn prefix_iter<'a>(&'a self, prefix: &str) -> Box<Iterator<Item = String> + 'a> {
        match self.prefix_node(prefix.chars()) {
            Some(node) => Box::new(Iter::new(node, prefix.to_owned())),
            None => Box::new(Iter::empty()),
        }
    }
}

impl TrieInsert for ArrayTrie {
    fn insert(&mut self, s: &str) {
        assert!(s.len() > 0, "Empty key");
        self.insert_chars(s.chars())
    }
}

/// Iterator work item.
struct StringNodePair<'a> {
    s: String,
    node: &'a ArrayTrie,
}

struct Iter<'a> {
    work: Vec<StringNodePair<'a>>,
}

impl<'a> Iter<'a> {
    fn empty() -> Self {
        Iter { work: Vec::new() }
    }

    fn new(node: &'a ArrayTrie, prefix: String) -> Self {
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
            for &Edge { ref ch, ref to } in &pair.node.edges {
                let mut new_str = pair.s.clone();
                new_str.push(*ch);
                self.work.push(StringNodePair {
                    node: &to,
                    s: new_str,
                })
            }

            if pair.node.accepting {
                return Some(pair.s);
            }
        }
    }
}
