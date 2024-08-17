use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::Display,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign},
};

use crate::utility;

/// Set is collection of distinct objects.
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Set<T> {
    data: BTreeSet<T>,
}

impl<T: Ord> Set<T> {
    /// Construct an empty set.
    pub fn new() -> Self {
        Self { data: BTreeSet::new() }
    }

    /// Return the number of elements in the set.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Return `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Return an iterator that visits the elements in the `Set` in ascending order.
    pub fn iter(&self) -> std::collections::btree_set::Iter<T> {
        self.data.iter()
    }

    /// Return the element of the specified `value`, or `None` if the set does not contain the element.
    pub fn find(&self, value: &T) -> Option<&T> {
        self.data.iter().find(|&e| e == value)
    }

    /// Return `true` if the set contains an element equal to the value.
    pub fn contains(&self, value: &T) -> bool {
        self.data.contains(value)
    }

    /// Get the smallest item of the set, or `None` if the set is empty.
    pub fn min(&self) -> Option<&T> {
        self.data.first()
    }

    /// Get the largest item of the set, or `None` if the set is empty.
    pub fn max(&self) -> Option<&T> {
        self.data.last()
    }

    /// Return `true` if `self` has no elements in common with `other`.
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.data.is_disjoint(&other.data)
    }

    /// Add a value to the set. Return whether the value was newly inserted.
    pub fn add(&mut self, value: T) -> bool {
        self.data.insert(value)
    }

    /// Remove a value from the set. Return whether such an element was present.
    pub fn remove(&mut self, value: &T) -> bool {
        self.data.remove(value)
    }

    /// Remove the first element from the set and returns it, if any.
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop_first()
    }

    /// Clear the set, removing all elements.
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

/*
Function
*/

impl<T: Ord> PartialOrd for Set<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.data.is_subset(&other.data), self.data.is_superset(&other.data)) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            (false, false) => None,
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

impl<T: Ord> Extend<T> for Set<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter)
    }
}

/*
Display
*/

impl<T: Display> Display for Set<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        utility::print(f, self.data.iter(), '{', '}')
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
