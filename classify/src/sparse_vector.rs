use std::collections::BTreeMap;
use std::collections::btree_map;
use std::mem;
use std::ops::{Deref, DerefMut};

use num_traits::{Num, Unsigned, Zero};

pub trait Idx: Unsigned + Ord {}
impl<T> Idx for T where T: Unsigned + Ord {}

/// Sparse vector implementation.
///
/// This is a simple wrapper around a BTreeMap that automatically removes
/// components that are set to 0 from the sparse representation.
pub struct SparseVector<I, V>
    where I: Idx
{
    data: BTreeMap<I, V>,
}

impl<I, V> SparseVector<I, V>
    where I: Idx,
          V: Num
{
    pub fn new() -> Self {
        SparseVector { data: BTreeMap::new() }
    }

    pub fn get(&mut self, idx: I) -> MutableValue<I, V> {
        MutableValue { entry: ValueWrapper::from(self.data.entry(idx)) }
    }

    pub fn iter(&self) -> Iter<I, V> {
        Iter { inner: self.data.iter() }
    }

    pub fn iter_mut(&mut self) -> IterMut<I, V> {
        IterMut { inner: self.data.iter_mut() }
    }
}

enum ValueWrapper<'a, I, V>
    where I: 'a + Idx,
          V: 'a
{
    Vacant(btree_map::VacantEntry<'a, I, V>, V),
    Occupied(btree_map::OccupiedEntry<'a, I, V>),
    Absent,
}


impl<'a, I, V> ValueWrapper<'a, I, V>
    where I: Idx,
          V: Zero
{
    fn from(entry: btree_map::Entry<'a, I, V>) -> Self {
        match entry {
            btree_map::Entry::Vacant(entry) => ValueWrapper::Vacant(entry, Zero::zero()),
            btree_map::Entry::Occupied(entry) => ValueWrapper::Occupied(entry),
        }
    }
}

pub struct MutableValue<'a, I, V>
    where I: 'a + Idx,
          V: 'a + Num
{
    entry: ValueWrapper<'a, I, V>,
}

impl<'a, I, V> Deref for MutableValue<'a, I, V>
    where I: Idx,
          V: Num
{
    type Target = V;

    fn deref(&self) -> &Self::Target {
        match self.entry {
            ValueWrapper::Occupied(ref occupied) => occupied.get(),
            ValueWrapper::Vacant(_, ref value) => value,
            ValueWrapper::Absent => unreachable!(),
        }
    }
}

impl<'a, I, V> DerefMut for MutableValue<'a, I, V>
    where I: Idx,
          V: Num
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self.entry {
            ValueWrapper::Occupied(ref mut occupied) => occupied.get_mut(),
            ValueWrapper::Vacant(_, ref mut value) => value,
            ValueWrapper::Absent => unreachable!(),
        }
    }
}


impl<'a, I, V> Drop for MutableValue<'a, I, V>
    where I: Idx,
          V: Num
{
    fn drop(&mut self) {
        let mut entry = ValueWrapper::Absent;
        mem::swap(&mut entry, &mut self.entry);

        match entry {
            ValueWrapper::Occupied(occupied) => {
                if *occupied.get() == Zero::zero() {
                    occupied.remove();
                }
            }
            ValueWrapper::Vacant(vacant, value) => {
                if value != Zero::zero() {
                    vacant.insert(value);
                }
            }
            ValueWrapper::Absent => unreachable!(),
        }
    }
}

pub struct Iter<'a, I, V>
    where I: 'a + Idx,
          V: 'a
{
    inner: btree_map::Iter<'a, I, V>,
}

impl<'a, I, V> Iterator for Iter<'a, I, V>
    where I: Idx
{
    type Item = (&'a I, &'a V);

    fn next(&mut self) -> Option<(&'a I, &'a V)> {
        self.inner.next()
    }
}

pub struct IterMut<'a, I, V>
    where I: 'a + Idx,
          V: 'a
{
    inner: btree_map::IterMut<'a, I, V>,
}

impl<'a, I, V> Iterator for IterMut<'a, I, V>
    where I: Idx
{
    type Item = (&'a I, &'a mut V);

    fn next(&mut self) -> Option<(&'a I, &'a mut V)> {
        self.inner.next()
    }
}