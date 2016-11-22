use std::borrow::Cow;
use std::fmt;
use std::io;
use std::io::{BufRead, Write};
use std::slice;
use std::vec;

/// An InvertedIndexMut is an inverted index that can be mutated.
pub trait InvertedIndexMut<N: Ord> {
    /// Add a term-docid pair to the inverted index.
    fn add_term(&mut self, term: &str, doc: N);

    /// Add a postings lists for a term. If the term is already in the inverted
    /// index, its posting list is replaced.
    fn add_postings_list<D>(&mut self, term: &str, docs: D) where D: Into<Vec<N>>;
}

/// An inverted index stores a term <-> postings list mapping.
pub trait InvertedIndex<N: Clone + Ord> {
    /// Iterate over all term, postings list pairs in the inverted index.
    fn iter<'a>(&'a self) -> Box<Iterator<Item = (&str, Posting<N>)> + 'a>;

    /// Get the number of terms in the index.
    fn len(&self) -> usize;

    /// Retrieve the postings list for a term.
    fn posting(&self, term: &str) -> Option<Posting<N>>;
}

#[derive(Debug)]
pub enum TextReadError {
    Io(io::Error),
    NoTerm,
    NotSortedOrUnique(String),
    Parse,
}

impl From<io::Error> for TextReadError {
    fn from(err: io::Error) -> TextReadError {
        TextReadError::Io(err)
    }
}

impl fmt::Display for TextReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TextReadError::Io(ref err) => write!(f, "{}", err),
            &TextReadError::NoTerm => write!(f, "No term found"),
            &TextReadError::NotSortedOrUnique(ref line) => {
                write!(f, "Postings list not sorted or unique: {}", line)
            }
            &TextReadError::Parse => write!(f, "Could not parse document identifier"),
        }
    }
}

pub trait InvertedIndexFromText<N>
    where Self: Sized
{
    /// Read an inverted index from a buffered reader. The expected format
    /// is:
    ///
    /// * Each line contains a term and postings list.
    /// * The term and postings list are separated by the tab character.
    /// * The postings list consists of unsigned integers separated by the space character.
    fn from_text<R>(reader: R) -> Result<Self, TextReadError> where R: BufRead;
}

pub trait InvertedIndexToText<N> {
    fn to_text<W>(&self, writer: &mut W) -> io::Result<()> where W: Write;
}

/// A Posting is a sorted list of unique document identifiers.
#[derive(Clone, Debug)]
pub struct Posting<'a, N>
    where N: Ord + Clone + 'a
{
    docs: Cow<'a, [N]>,
}

impl<'a, N> Posting<'a, N>
    where N: Clone + Ord
{
    /// Intersect the postings list with another postings list. If n is the
    /// length of the smaller list and m is the length of the larger list,
    /// intersection is in: O(n + n log m) iff n < m / log m and O(n + m)
    /// otherwise.
    pub fn intersect(&self, other: &Posting<N>) -> Posting<'static, N> {
        let (smaller, larger) = min_max_posting(self, other);

        let larger_f = larger.len() as f64;

        if (smaller.len() as f64) < (larger_f / larger_f.log(2.)) {
            self.intersect_binsearch(other)
        } else {
            self.intersect_linear(other)
        }
    }

    fn intersect_binsearch(&self, other: &Posting<N>) -> Posting<'static, N> {
        let mut inter = Vec::new();

        let (smaller, larger) = min_max_posting(self, other);

        let mut offset = 0;
        for doc in smaller.docs.as_ref() {
            offset = match larger.docs[offset..].binary_search(doc) {
                Ok(idx) => {
                    inter.push(doc.clone());
                    idx
                }
                Err(idx) => idx,
            }
        }

        posting_from_vec(inter)
    }

    fn intersect_linear(&self, other: &Posting<N>) -> Posting<'static, N> {
        let mut inter = Vec::new();

        let mut p1i = 0;
        let mut p2i = 0;

        while p1i != self.docs.len() && p2i != other.docs.len() {
            let doc1 = &self.docs.as_ref()[p1i];
            let doc2 = &other.docs.as_ref()[p2i];

            if doc1 == doc2 {
                inter.push(doc1.clone());
                p1i += 1;
                p2i += 1;
            } else if doc1 < doc2 {
                p1i += 1;
            } else {
                p2i += 1;
            }
        }

        posting_from_vec(inter)
    }

    /// Get an iterator over the document IDs in the posting list.
    pub fn iter(&self) -> slice::Iter<N> {
        return self.docs.iter();
    }

    /// Get the size of the postings list.
    pub fn len(&self) -> usize {
        return self.docs.len();
    }
}

impl<'a, N> IntoIterator for Posting<'a, N>
    where N: Clone + Ord
{
    type Item = N;

    type IntoIter = vec::IntoIter<N>;

    fn into_iter(self) -> Self::IntoIter {
        self.docs.into_owned().into_iter()
    }
}

impl<'a, N> IntoIterator for &'a Posting<'a, N>
    where N: Clone + Ord
{
    type Item = &'a N;

    type IntoIter = slice::Iter<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.docs.into_iter()
    }
}

fn min_max_posting<'a, N>(a: &'a Posting<'a, N>,
                          b: &'a Posting<'a, N>)
                          -> (&'a Posting<'a, N>, &'a Posting<'a, N>)
    where N: Clone + Ord
{
    if a.docs.len() < b.docs.len() {
        (a, b)
    } else {
        (b, a)
    }
}

pub fn posting_from_ref<'a, N>(s: &'a [N]) -> Posting<'a, N>
    where N: Clone + Ord
{
    Posting { docs: Cow::Borrowed(s) }
}

pub fn posting_from_vec<N>(v: Vec<N>) -> Posting<'static, N>
    where N: Clone + Ord
{
    Posting { docs: Cow::Owned(v) }
}
