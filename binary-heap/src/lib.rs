#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use std::mem;
use std::ops::{Deref, DerefMut};

/// A binary (max) heap.
///
/// A binary heap stores elements such that the largest element can always
/// be retrieved in O(1) time.
///
/// The binary heap uses an array as a backing store, making it
/// memory-efficient.
///
/// Note that this is a smaller binary heap implementation for the IR course.
/// Rust has its own (more extensive) implementation:
///
/// <https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html>
pub struct BinaryHeap<T>
    where T: Ord
{
    data: Vec<T>,
}

impl<T> BinaryHeap<T>
    where T: Ord
{
    /// Create an empty heap.
    pub fn new() -> Self {
        BinaryHeap { data: Vec::new() }
    }

    /// Insert a value into the heap. The average-case complexity is *O(1)*,
    /// the worst-case complexity *O(log n)*.
    pub fn insert(&mut self, val: T) {
        let idx = self.data.len();
        self.data.push(val);
        self.swim(idx);
    }

    /// Perform an in-place conversion of the heap to a sorted `Vec`. This
    /// operation is in *O(n log n)* time.
    pub fn into_sorted_vec(mut self) -> Vec<T> {
        for heap_end in (1..self.data.len()).rev() {
            self.data.swap(0, heap_end);
            self.sink_range(0, heap_end);
        }

        self.data
    }

    /// Returns `true` if the heap is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get the largest element in the heap. Returns `None` when the heap
    /// is empty.
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    /// Rust's BinaryHeap has a variant of `peek` that returns the root as
    /// a value that can be mutated. When the mutable value binding gets out
    /// of scope, the heap property is automatically restored through `Drop`.
    ///
    /// This is kinda cool, since it combines a couple of Rusts strenghts
    /// (deterministic destructors and forbidding simultaneous mutable and
    /// immutable borrowing). So, I tried to implement this myself :).
    pub fn peek_mut(&mut self) -> Option<PeekMut<T>> {
        if self.data.is_empty() {
            None
        } else {
            Some(PeekMut { heap: self })
        }
    }

    /// Retrieve and remove the largest element.
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop().map(|mut v| {
            if !self.data.is_empty() {
                mem::swap(&mut v, &mut self.data[0]);
                self.sink(0);
            }

            v
        })
    }

    /// The sink operation restores the heap property when a value is
    /// smaller than one of its children.
    fn sink(&mut self, idx: usize) {
        let len = self.data.len();
        self.sink_range(idx, len);
    }

    /// The sink operation restores the heap property when a value is
    /// smaller than one of its children.
    ///
    /// This variant allows us to provide an end index (exclusive) to
    /// apply sink when only a part of the data array constitutes the
    /// heap.
    fn sink_range(&mut self, mut idx: usize, end: usize) {
        let mut child = (idx * 2) + 1;

        while child < end {
            let right_child = child + 1;

            if right_child < end && self.data[right_child] > self.data[child] {
                child = right_child;
            }

            if self.data[idx] >= self.data[child] {
                break;
            }

            self.data.swap(idx, child);

            idx = child;
            child = (idx * 2) + 1;
        }
    }

    /// The sink operation restores the heap property when a value is
    /// larger than its parent.
    fn swim(&mut self, mut idx: usize) {
        while idx != 0 {
            let parent = (idx - 1) / 2;

            if self.data[idx] <= self.data[parent] {
                break;
            }

            self.data.swap(idx, parent);

            idx = parent;
        }
    }
}

impl<T> From<Vec<T>> for BinaryHeap<T>
    where T: Ord
{
    fn from(vec: Vec<T>) -> BinaryHeap<T> {
        let mut heap = BinaryHeap { data: vec };

        for idx in 1..(heap.data.len()) {
            heap.swim(idx)
        }

        heap
    }
}

/// A mutable reference to the largest value of a `BinaryHeap`.
///
/// If the value is changed such that the heap property is violated, it is
/// automatically restored when the binding goes out of scope.
pub struct PeekMut<'a, T>
    where T: 'a + Ord
{
    heap: &'a mut BinaryHeap<T>,
}

impl<'a, T> Deref for PeekMut<'a, T>
    where T: Ord
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.heap.data[0]
    }
}

impl<'a, T> DerefMut for PeekMut<'a, T>
    where T: Ord
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.heap.data[0]
    }
}

impl<'a, T> Drop for PeekMut<'a, T>
    where T: Ord
{
    fn drop(&mut self) {
        self.heap.sink(0);
    }
}

/// Sort a `Vec`.
///
/// This first converts the `Vec` to a binary heap. Then the binary heap is
/// converted to a sorted `Vec`. Both conversions are performed in-place in
/// the vector.
pub fn heapsort<T>(arr: Vec<T>) -> Vec<T>
    where T: Ord
{
    let heap = BinaryHeap::from(arr);
    heap.into_sorted_vec()
}

#[cfg(test)]
mod tests;
