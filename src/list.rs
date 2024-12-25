use std::{
    fmt::Display,
    ops::{Add, AddAssign, Index, IndexMut},
};

use crate::detail;

/// List is collection of homogeneous objects.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct List<T> {
    pub(crate) data: Vec<T>,
}

impl<T> List<T> {
    /// Construct an empty list.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Return the number of elements in the list.
    pub fn len(&self) -> i32 {
        self.data.len() as i32
    }

    /// Return `true` if the list contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Return an iterator over the list.
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    /// Return the first occurrence of the specified `element` or `None` in the list.
    pub fn find(&self, element: &T) -> Option<&T>
    where
        T: PartialEq,
    {
        self.data.iter().find(|&x| x == element)
    }

    /// Return `true` if the list contains `element`.
    pub fn contains(&self, element: &T) -> bool
    where
        T: PartialEq,
    {
        self.data.contains(element)
    }

    /// Count the number of occurrence of the specified `element`.
    pub fn count(&self, element: &T) -> usize
    where
        T: PartialEq,
    {
        self.data.iter().filter(|&x| x == element).count()
    }

    /// Return a list that eliminates duplicate elements and keep the original relative order of elements.
    pub fn uniquify(&self) -> Self
    where
        T: PartialEq + Clone,
    {
        let mut buffer = Vec::with_capacity(self.data.len());
        for i in 0..self.data.len() {
            if !buffer.contains(&self.data[i]) {
                buffer.push(self.data[i].clone());
            }
        }
        Self { data: buffer }
    }

    /// Insert the specified `element` at the specified `index` in the list.
    /// Index can be negative.
    pub fn insert(&mut self, index: i32, element: T) {
        detail::check_full(self.data.len(), i32::MAX as usize);
        detail::check_bounds(index, -self.len(), self.len() + 1);

        let index = detail::calc_index(index, self.data.len());
        self.data.insert(index, element)
    }

    /// Remove the `element` at the specified `index` in the list.
    /// Index can be negative.
    pub fn remove(&mut self, index: i32) -> T {
        detail::check_empty(self.data.len());
        detail::check_bounds(index, -self.len(), self.len());

        let index = detail::calc_index(index, self.data.len());
        self.data.remove(index)
    }

    /// Append an element to the back of the list.
    pub fn push(&mut self, element: T) {
        detail::check_full(self.data.len(), i32::MAX as usize);
        self.data.push(element)
    }

    /// Remove the last element from a list and returns it, or `None` if it is empty.
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Clear the list.
    pub fn clear(&mut self) {
        self.data.clear()
    }

    /// Reverse the order of elements in the list, in place.
    pub fn reverse(&mut self) {
        self.data.reverse()
    }
}

/*
Construct
*/

impl<T, const N: usize> From<[T; N]> for List<T> {
    fn from(value: [T; N]) -> Self {
        detail::check_full(N, i32::MAX as usize);
        Self { data: Vec::from(value) }
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(value: Vec<T>) -> Self {
        detail::check_full(value.len(), i32::MAX as usize);
        Self { data: value }
    }
}

/*
Function
*/

impl<T> Index<i32> for List<T> {
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        detail::check_bounds(index, -self.len(), self.len());

        let index = detail::calc_index(index, self.data.len());
        &self.data[index]
    }
}

impl<T> IndexMut<i32> for List<T> {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        detail::check_bounds(index, -self.len(), self.len());

        let index = detail::calc_index(index, self.data.len());
        &mut self.data[index]
    }
}

#[auto_impl_ops::auto_ops]
impl<T> AddAssign for List<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.data.extend(rhs.data)
    }
}

impl<T> Extend<T> for List<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter)
    }
}

/*
Display
*/

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        detail::print(f, self.iter(), '[', ']')
    }
}

/*
Iterator
*/

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let data = iter.into_iter().collect();
        Self { data }
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
