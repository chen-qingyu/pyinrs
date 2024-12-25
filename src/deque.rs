use std::{
    collections::VecDeque,
    fmt::Display,
    ops::{Index, IndexMut, Shl, ShlAssign, Shr, ShrAssign},
};

use crate::detail;

/// Deque is generalization of stack and queue, supports memory efficient pushes and pops from either side.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Deque<T> {
    data: VecDeque<T>,
}

impl<T> Deque<T> {
    /// Construct an empty deque.
    pub fn new() -> Self {
        Self { data: VecDeque::new() }
    }

    /// Create an empty deque with space for at least `capacity` elements.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
        }
    }

    /// Return the number of elements in the deque.
    pub fn len(&self) -> i32 {
        self.data.len() as i32
    }

    /// Return `true` if the deque is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Provide a forward iterator.
    pub fn iter(&self) -> std::collections::vec_deque::Iter<T> {
        self.data.iter()
    }

    /// Provide a reference to the back element, or `None` if the deque is empty.
    pub fn back(&self) -> Option<&T> {
        self.data.back()
    }

    /// Provide a reference to the front element, or `None` if the deque is empty.
    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    /// Provide a mutable reference to the back element, or `None` if the deque is empty.
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.data.back_mut()
    }

    /// Provide a mutable reference to the front element, or `None` if the deque is empty.
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.data.front_mut()
    }

    /// Append the given `element` to the end of the deque.
    pub fn push_back(&mut self, element: T) -> &Self {
        detail::check_full(self.data.len(), i32::MAX as usize);

        self.data.push_back(element);
        self
    }

    /// Prepend the given `element` to the beginning of the deque.
    pub fn push_front(&mut self, element: T) -> &Self {
        detail::check_full(self.data.len(), i32::MAX as usize);

        self.data.push_front(element);
        self
    }

    /// Remove the last element and returns it, or `None` if the deque is empty.
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    /// Remove the first element and returns it, or `None` if the deque is empty.
    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    /// Remove all elements from the deque.
    pub fn clear(&mut self) {
        self.data.clear()
    }

    /// Reserve capacity for at least `additional` more elements to be inserted in the given deque.
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional)
    }
}

/*
Construct
*/

impl<T, const N: usize> From<[T; N]> for Deque<T> {
    fn from(value: [T; N]) -> Self {
        Self { data: VecDeque::from(value) }
    }
}

/*
Function
*/

impl<T> Index<i32> for Deque<T> {
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        detail::check_bounds(index, -self.len(), self.len());

        let index = detail::calc_index(index, self.data.len());
        &self.data[index]
    }
}

impl<T> IndexMut<i32> for Deque<T> {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        detail::check_bounds(index, -self.len(), self.len());

        let index = detail::calc_index(index, self.data.len());
        &mut self.data[index]
    }
}

#[auto_impl_ops::auto_ops]
impl<T> ShlAssign<usize> for Deque<T> {
    fn shl_assign(&mut self, mut rhs: usize) {
        if self.data.len() <= 1 || rhs == 0 {
            return;
        }

        rhs %= self.data.len();

        let mut tail = self.data.split_off(rhs);
        tail.append(&mut self.data);
        self.data = tail;
    }
}

#[auto_impl_ops::auto_ops]
impl<T> ShrAssign<usize> for Deque<T> {
    fn shr_assign(&mut self, rhs: usize) {
        if self.data.len() <= 1 || rhs == 0 {
            return;
        }

        self.shl_assign(self.data.len() - rhs % self.data.len()); // avoid to subtract with overflow
    }
}

impl<T> Extend<T> for Deque<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter)
    }
}

/*
Display
*/

impl<T: Display> Display for Deque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        detail::print(f, self.iter(), '<', '>')
    }
}

/*
Iterator
*/

impl<T> FromIterator<T> for Deque<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let data = iter.into_iter().collect();
        Self { data }
    }
}

impl<T> IntoIterator for Deque<T> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
