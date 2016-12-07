/// The trait `TrieContains` provides membership checks.
pub trait TrieContains {
    /// Returns `true` when a word is in the trie, or `false` otherwise.
    fn contains(&self, word: &str) -> bool;
}

/// The trait `TrieContains` supports iteration over all words in the trie
/// starting with the given prefix.
pub trait TriePrefixIter {
    /// Iterate over the words starting with the given `prefix`.
    fn prefix_iter<'a>(&'a self, prefix: &str) -> Box<Iterator<Item = String> + 'a>;
}

/// The trait `TrieContains` provides a method to insert words in the trie.
pub trait TrieInsert {
    /// Iterate over the words starting with the given `prefix`.
    fn insert(&mut self, word: &str);
}
