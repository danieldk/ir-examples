use std::collections::hash_map;
use std::collections::hash_map::HashMap;
use std::fmt;
use std::io;
use std::io::{BufRead, Write};
use std::str::FromStr;

use itertools::Itertools;

use super::*;
use super::index::{posting_from_ref, posting_from_vec};

/// In-memory inverted index.
pub struct MemoryIndex<N> {
    terms: HashMap<String, Vec<N>>,
}

impl<N> MemoryIndex<N>
    where N: Ord
{
    /// Construct an empty in-memory inverted index.
    pub fn new() -> MemoryIndex<N> {
        MemoryIndex { terms: HashMap::new() }
    }
}

impl<N> InvertedIndexFromText<N> for MemoryIndex<N>
    where N: FromStr + Ord
{
    /// Read an inverted index from a buffered reader. The expected format
    /// is:
    ///
    /// * Each line contains a term and postings list.
    /// * The term and postings list are separated by the tab character.
    /// * The postings list consists of unsigned integers separated by the space character.
    fn from_text<R>(reader: R) -> Result<Self, TextReadError>
        where R: BufRead
    {
        let mut index = MemoryIndex::new();

        for line in reader.lines() {
            let line = try!(line);
            let mut iter = line.split_whitespace();

            let term = try!(iter.next().ok_or(TextReadError::NoTerm));

            let mut docs: Vec<N> = Vec::new();
            for doc_str in iter {
                docs.push(try!(doc_str.parse().map_err(|_| TextReadError::Parse)));
            }

            if !is_sorted_uniq(&docs) {
                return Err(TextReadError::NotSortedOrUnique(line.to_owned()));
            }

            docs.shrink_to_fit();
            index.add_postings_list(term, docs);
        }

        Ok(index)
    }
}

impl<N> InvertedIndexToText<N> for MemoryIndex<N>
    where N: fmt::Display
{
    fn to_text<W>(&self, writer: &mut W) -> io::Result<()>
        where W: Write
    {
        for (term, posting) in &self.terms {
            try!(write!(writer, "{}\t", term));
            let docs_str = posting.iter().map(ToString::to_string).join(" ");
            try!(writer.write(docs_str.as_bytes()));
            try!(writer.write(&['\n' as u8]));
        }

        Ok(())
    }
}

impl<N> IntoIterator for MemoryIndex<N>
    where N: 'static + Clone + Ord
{
    type Item = (String, Posting<'static, N>);

    type IntoIter = IntoIter<N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { term_docs_iter: self.terms.into_iter() }
    }
}

impl<'a, N> IntoIterator for &'a MemoryIndex<N>
    where N: Clone + Ord
{
    type Item = (&'a str, Posting<'a, N>);

    type IntoIter = Iter<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { term_docs_iter: self.terms.iter() }
    }
}

pub struct Iter<'a, N: 'a> {
    term_docs_iter: hash_map::Iter<'a, String, Vec<N>>,
}

impl<'a, N> Iterator for Iter<'a, N>
    where N: Clone + Ord
{
    type Item = (&'a str, Posting<'a, N>);

    fn next(&mut self) -> Option<Self::Item> {
        self.term_docs_iter.next().map(|(k, v)| (k.as_ref(), posting_from_ref(&v)))
    }
}

pub struct IntoIter<N> {
    term_docs_iter: hash_map::IntoIter<String, Vec<N>>,
}

impl<N> Iterator for IntoIter<N>
    where N: 'static + Clone + Ord
{
    type Item = (String, Posting<'static, N>);

    fn next(&mut self) -> Option<Self::Item> {
        self.term_docs_iter.next().map(|(k, v)| (k, posting_from_vec(v)))
    }
}

impl<N: Ord> InvertedIndexMut<N> for MemoryIndex<N> {
    fn add_term(&mut self, term: &str, doc: N) {
        let mut posting = self.terms.entry(term.to_owned()).or_insert(Vec::new());

        if let Err(idx) = posting.binary_search(&doc) {
            posting.insert(idx, doc)
        }
    }


    fn add_postings_list<D>(&mut self, term: &str, docs: D)
        where D: Into<Vec<N>>
    {
        self.terms.insert(term.to_owned(), docs.into());
    }
}

impl<N> InvertedIndex<N> for MemoryIndex<N>
    where N: Clone + Ord
{
    fn iter<'a>(&'a self) -> Box<Iterator<Item = (&str, Posting<N>)> + 'a> {
        Box::new(Iter { term_docs_iter: self.terms.iter() })

    }

    fn posting(&self, term: &str) -> Option<Posting<N>> {
        self.terms.get(term).map(|docs| posting_from_ref(docs))
    }

    fn len(&self) -> usize {
        self.terms.len()
    }
}
