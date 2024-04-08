use std::{
    collections::VecDeque,
    fmt::Display,
    ops::{Shl, ShlAssign, Shr, ShrAssign},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Deque<T> {
    data: VecDeque<T>,
}

impl<T> Deque<T> {
    /// Creates an empty `Deque`.
    pub fn new() -> Self {
        Deque { data: VecDeque::new() }
    }

    /// Creates an empty deque with space for at least `capacity` elements.
    pub fn with_capacity(capacity: usize) -> Self {
        Deque {
            data: VecDeque::with_capacity(capacity),
        }
    }

    /// Returns the length of the deque.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the deque is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Provides a forward iterator.
    pub fn iter(&self) -> std::collections::vec_deque::Iter<T> {
        self.data.iter()
    }

    /// Provides a reference to the back element, or `None` if the deque is empty.
    pub fn back(&self) -> Option<&T> {
        self.data.back()
    }

    /// Provides a reference to the front element, or `None` if the deque is empty.
    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    /// Provides a mutable reference to the back element, or `None` if the deque is empty.
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.data.back_mut()
    }

    /// Provides a mutable reference to the front element, or `None` if the deque is empty.
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.data.front_mut()
    }

    /// Appends an element to the back of the deque.
    pub fn push_back(&mut self, element: T) -> &Self {
        self.data.push_back(element);
        self
    }

    /// Adds an element first in the deque.
    pub fn push_front(&mut self, element: T) -> &Self {
        self.data.push_front(element);
        self
    }

    /// Removes the last element and returns it, or `None` if the deque is empty.
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    /// Removes the first element and returns it, or `None` if the deque is empty.
    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    /// Removes all elements from the deque.
    pub fn clear(&mut self) {
        self.data.clear()
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the given deque.
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

impl<T> ShrAssign<usize> for Deque<T> {
    fn shr_assign(&mut self, mut rhs: usize) {
        if self.data.len() <= 1 || rhs == 0 {
            return;
        }

        rhs %= self.data.len();

        let mut tail = self.data.split_off(self.data.len() - rhs);
        tail.append(&mut self.data);
        self.data = tail;
    }
}

impl<T> Shl<usize> for Deque<T> {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self::Output {
        self <<= rhs;
        self
    }
}

impl<T> Shr<usize> for Deque<T> {
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self::Output {
        self >>= rhs;
        self
    }
}

/*
Display
*/

impl<T: Display> Display for Deque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.data.is_empty() {
            return write!(f, "<>");
        }

        let mut it = self.data.iter().peekable();
        write!(f, "<")?;
        loop {
            write!(f, "{}", it.next().unwrap())?;
            if it.peek().is_none() {
                return write!(f, ">");
            }
            write!(f, ", ")?;
        }
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

/*
Transform
*/

impl<T> From<VecDeque<T>> for Deque<T> {
    fn from(value: VecDeque<T>) -> Self {
        Self { data: value }
    }
}

impl<T> From<Deque<T>> for VecDeque<T> {
    fn from(value: Deque<T>) -> Self {
        value.data
    }
}
