use std::cmp;
use std::cmp::Ordering;
use std::iter::Peekable;
use std::mem;
use std::str::Chars;

use num::{Bounded, One, Unsigned};
use rand::Rng;
use rand::distributions::range::SampleRange;

use trie::*;

pub trait Priority: Unsigned + Bounded + Copy + Ord + SampleRange {}
impl<T> Priority for T where T: Unsigned + Bounded + Copy + Ord + SampleRange {}

/// A randomized ternary search trie. This trie stores words with shared
/// prefixes. Randomization is used to ensure that the tree is (typically)
/// balanced. See: Randomized Ternary Search Tries, Nicolai Diethelm
///
/// <https://arxiv.org/abs/1606.04042>
pub struct TernaryTree<P = u32>
    where P: Priority
{
    root: BoxedNode<P>,
    rng: Box<Rng>,
}

impl<P> TernaryTree<P>
    where P: Priority
{
    /// Construct a trie. This constructor has a priority type parameter,
    /// This allows the user to specify the type of the priority. E.g. for
    /// smaller trees a narrow unsigned could suffice and saves memory.
    pub fn new_with_prio<R>(rng: R) -> Self
        where R: Rng + 'static
    {
        TernaryTree {
            root: BoxedNode::default(),
            rng: Box::new(rng),
        }
    }
}

impl TernaryTree<u32> {
    /// Construct a trie. The random number generator will be used to
    /// generate word priorities.
    pub fn new<R>(rng: R) -> Self
        where R: Rng + 'static
    {
        TernaryTree {
            root: BoxedNode::default(),
            rng: Box::new(rng),
        }
    }
}

impl<P> TrieContains for TernaryTree<P>
    where P: Priority
{
    fn contains(&self, word: &str) -> bool {
        assert!(!word.is_empty(),
                "Cannot search empty string in ternary trie");

        match self.root.prefix_node(word.chars().peekable()) {
            Some(node) => node.str_prio != Bounded::min_value(),
            None => false,
        }
    }
}

impl<P> TriePrefixIter for TernaryTree<P>
    where P: Priority
{
    fn prefix_iter<'a>(&'a self, prefix: &str) -> Box<Iterator<Item = String> + 'a> {
        if prefix.is_empty() {
            return Box::new(Iter::new(self.root.as_ref()));
        }

        // Get the tree node that represents the prefix.
        let node = self.root.prefix_node(prefix.chars().peekable());

        Box::new(Iter::with_prefix(node, prefix.to_owned()))
    }
}

impl<P> TrieInsert for TernaryTree<P>
    where P: Priority
{
    fn insert(&mut self, s: &str) {
        assert!(s.len() > 0, "Empty key");

        let mut root = BoxedNode::default();
        mem::swap(&mut root, &mut self.root);
        self.root = root.insert(s.chars().peekable(), &mut self.rng);
    }
}

#[derive(Debug)]
struct TreeNode<P> {
    ch: char,

    // Node priority. This should always be larger than the priorities of the
    // left and right child.
    prio: P,

    // String priority: 0 if this node does not represent a word, non-0 otherwise.
    str_prio: P,

    left: BoxedNode<P>,
    mid: BoxedNode<P>,
    right: BoxedNode<P>,
}

impl<P> TreeNode<P>
    where P: Priority
{
    fn new(ch: char) -> Self {
        TreeNode {
            ch: ch,
            prio: Bounded::min_value(),
            str_prio: Bounded::min_value(),
            left: BoxedNode::default(),
            mid: BoxedNode::default(),
            right: BoxedNode::default(),
        }
    }
}

/// A boxed node: the motivation is twofold:
///
/// - The size of a recursive value type cannot be computed.
/// - This representation allows us to model absent nodes (that we can
///   still insert on).
#[derive(Debug)]
struct BoxedNode<P>(Option<Box<TreeNode<P>>>);

impl<P> BoxedNode<P> {
    /// Construct a boxed node from a tree node.
    fn new(node: TreeNode<P>) -> Self {
        BoxedNode(Some(Box::new(node)))
    }

    /// Get the boxed node as a reference.
    fn as_ref(&self) -> Option<&TreeNode<P>> {
        self.0.as_ref().map(|b| b.as_ref())
    }
}

impl<P> BoxedNode<P>
    where P: Priority
{
    /// Insert characters into the tree starting at this boxed node. This
    /// method will panic if it is passed an iterator without characters.
    fn insert<R>(self, mut chars: Peekable<Chars>, rng: &mut R) -> Self
        where R: Rng
    {
        let ch = *chars.peek().unwrap();

        /// Unwrap the treenode, creating a new node if it was a None node.
        let mut node = match self.0 {
            Some(node) => *node,
            None => TreeNode::new(ch),
        };

        /// Insert into the left/middle/right node, as appropriate. Parents/childs
        /// are rotated if the child has a higher priority than the parent.
        match ch.cmp(&node.ch) {
            Ordering::Less => {
                node.left = node.left.insert(chars, rng);
                if node.left.as_ref().unwrap().prio > node.prio {
                    node = rotate_with_left(node);
                }
            }
            Ordering::Greater => {
                node.right = node.right.insert(chars, rng);
                if node.right.as_ref().unwrap().prio > node.prio {
                    node = rotate_with_right(node);
                }
            }
            Ordering::Equal => {
                // Consume the next character.
                chars.next();

                // If there is another character in the iterator, insert
                // recursively in the mid child. Otherwise, the node is
                // an accepting node -> generate a non-zero priority for
                // the string.
                if chars.peek().is_some() {
                    node.mid = node.mid.insert(chars, rng);
                } else if node.str_prio == Bounded::min_value() {
                    node.str_prio = rng.gen_range::<P>(Bounded::min_value(), Bounded::max_value()) +
                                    One::one();
                }

                // If the middle child exists and has a higher priority
                // than the string priority of the current node, update
                // the node priority to that of the mid child.
                node.prio = match node.mid.0 {
                    Some(ref mid) => cmp::max(node.str_prio, mid.prio),
                    None => node.str_prio,
                }
            }
        }

        BoxedNode::new(node)
    }

    /// Returns the node that represents the given prefix. Note that we
    /// return the accepting node and not its mid chid. Otherwise, a
    /// caller could not check if the prefix is also a word.
    fn prefix_node(&self, mut chars: Peekable<Chars>) -> Option<&TreeNode<P>> {
        match self.as_ref() {
            Some(node) => {
                chars.peek().cloned().and_then(|ch| match ch.cmp(&node.ch) {
                    Ordering::Less => node.left.prefix_node(chars),
                    Ordering::Greater => node.right.prefix_node(chars),
                    Ordering::Equal => {
                        chars.next();
                        if chars.peek().is_some() {
                            node.mid.prefix_node(chars)
                        } else {
                            Some(node)
                        }
                    }
                })
            }
            None => None,
        }

    }
}

impl<P> Default for BoxedNode<P> {
    fn default() -> Self {
        BoxedNode(None)
    }
}

/// Iterator items.
enum IterItem<'a, P>
    where P: 'a
{
    /// Pair of a node and the 'generated' string to reach the node.
    Node(Option<&'a TreeNode<P>>, String),

    /// A value (word accepted by the trie).
    Value(String),
}

struct Iter<'a, P: 'a> {
    work: Vec<IterItem<'a, P>>,
}

impl<'a, P> Iter<'a, P>
    where P: Priority
{
    /// Create a new iterator starting at the given node.
    fn new(root: Option<&'a TreeNode<P>>) -> Self {
        Iter { work: vec![IterItem::Node(root, String::new())] }
    }

    /// Create a new iterator starting at the given node, with a prefix.
    fn with_prefix(root: Option<&'a TreeNode<P>>, prefix: String) -> Self {
        if prefix.is_empty() {
            return Iter { work: vec![IterItem::Node(root, prefix)] };
        }

        let mut items = Vec::new();
        if let Some(root) = root {
            items.push(IterItem::Node(root.mid.as_ref(), prefix.clone()));

            // If the prefix is non-empty, we also have to check whether the
            // given node/prefix is a word. If so, we add this as a result
            // item.
            if root.str_prio != Bounded::min_value() {
                items.push(IterItem::Value(prefix));
            }
        }

        Iter { work: items }
    }
}

impl<'a, P> Iterator for Iter<'a, P>
    where P: Priority
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = try_ok!(self.work.pop());

            match item {
                IterItem::Value(val) => return Some(val),
                IterItem::Node(node, prefix) => {
                    // Note 'work' is a stack, so we have to add work that we want
                    // to do last first and vice versa.

                    let node = ok_or_continue!(node);

                    // Add reachable nodes as work.
                    self.work
                        .push(IterItem::Node(node.right.as_ref(), prefix.clone()));

                    let mut new_prefix = prefix.clone();
                    new_prefix.push(node.ch);

                    self.work
                        .push(IterItem::Node(node.mid.as_ref(), new_prefix.clone()));

                    if node.str_prio != Bounded::min_value() {
                        self.work.push(IterItem::Value(new_prefix.clone()));
                    }

                    self.work
                        .push(IterItem::Node(node.left.as_ref(), prefix.clone()));
                }
            }
        }
    }
}

/// Rotate node with its left child.
fn rotate_with_left<P>(mut node: TreeNode<P>) -> TreeNode<P> {
    let mut y = *node.left.0.unwrap();
    node.left = y.right;
    y.right = BoxedNode::new(node);
    y
}

/// Rotate node with its right child.
fn rotate_with_right<P>(mut node: TreeNode<P>) -> TreeNode<P> {
    let mut y = *node.right.0.unwrap();
    node.right = y.left;
    y.left = BoxedNode::new(node);
    y
}
