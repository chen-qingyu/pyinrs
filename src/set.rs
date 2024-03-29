use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::Display,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign},
};

use crate::{Deque, List};

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Set<T> {
    data: BTreeSet<T>,
}

impl<T: Ord> Set<T> {
    /// Makes a new, empty `Set`.
    pub fn new() -> Self {
        Self { data: BTreeSet::new() }
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Gets an iterator that visits the elements in the `Set` in ascending order.
    pub fn iter(&self) -> std::collections::btree_set::Iter<T> {
        self.data.iter()
    }

    /// Returns `true` if the set contains an element equal to the value.
    pub fn contains(&self, value: &T) -> bool {
        self.data.contains(value)
    }

    /// Returns `true` if `self` has no elements in common with `other`.
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.data.is_disjoint(&other.data)
    }

    /// Adds a value to the set.
    pub fn add(&mut self, value: T) -> &Self {
        self.data.insert(value);
        self
    }

    /// Removes a value from the set and drops it.
    pub fn remove(&mut self, value: &T) -> &Self {
        self.data.remove(value);
        self
    }

    /// Removes the first element from the set and returns it, if any.
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop_first()
    }

    /// Clears the set, removing all elements.
    pub fn clear(&mut self) {
        self.data.clear()
    }
}

/*
Construct
*/

impl<T: Ord, const N: usize> From<[T; N]> for Set<T> {
    fn from(value: [T; N]) -> Self {
        Self { data: BTreeSet::from(value) }
    }
}

impl<T: Ord> From<Deque<T>> for Set<T> {
    fn from(value: Deque<T>) -> Self {
        value.into_iter().collect()
    }
}

impl<T: Ord> From<List<T>> for Set<T> {
    fn from(value: List<T>) -> Self {
        value.into_iter().collect()
    }
}

/*
Function
*/

impl<T: Ord> PartialOrd for Set<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.data.is_subset(&other.data), self.data.is_superset(&other.data)) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            _ => unreachable!(),
        }
    }
}

impl<T: Ord + Clone> BitAnd for Set<T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Set { data: &self.data & &rhs.data }
    }
}

impl<T: Ord + Clone> BitOr for Set<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Set { data: &self.data | &rhs.data }
    }
}

impl<T: Ord + Clone> BitXor for Set<T> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Set { data: &self.data ^ &rhs.data }
    }
}

impl<T: Ord + Clone> Sub for Set<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Set { data: &self.data - &rhs.data }
    }
}

impl<T: Ord + Clone> BitAndAssign for Set<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.data = &self.data & &rhs.data;
    }
}

impl<T: Ord + Clone> BitOrAssign for Set<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.data = &self.data | &rhs.data;
    }
}

impl<T: Ord + Clone> BitXorAssign for Set<T> {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.data = &self.data ^ &rhs.data;
    }
}

impl<T: Ord + Clone> SubAssign for Set<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.data = &self.data - &rhs.data;
    }
}

/*
Display
*/

impl<T: Display> Display for Set<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.data.is_empty() {
            return write!(f, "{{}}");
        }

        let mut it = self.data.iter().peekable();
        write!(f, "{{")?;
        loop {
            write!(f, "{}", it.next().unwrap())?;
            if it.peek().is_none() {
                return write!(f, "}}");
            }
            write!(f, ", ")?;
        }
    }
}

/*
Iterator
*/

impl<T: Ord> FromIterator<T> for Set<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let data = iter.into_iter().collect();
        Self { data }
    }
}

impl<T> IntoIterator for Set<T> {
    type Item = T;
    type IntoIter = std::collections::btree_set::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

/*
Transform
*/

impl<T> From<BTreeSet<T>> for Set<T> {
    fn from(value: BTreeSet<T>) -> Self {
        Self { data: value }
    }
}

impl<T> From<Set<T>> for BTreeSet<T> {
    fn from(value: Set<T>) -> Self {
        value.data
    }
}
