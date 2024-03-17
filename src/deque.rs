#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Deque<T> {
    data: std::collections::LinkedList<T>,
}

impl<T> Deque<T> {
    /// Creates an empty `Deque`.
    pub fn new() -> Self {
        Deque {
            data: std::collections::LinkedList::new(),
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
    pub fn iter(&self) -> std::collections::linked_list::Iter<T> {
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
    pub fn push_back(&mut self, element: T) {
        self.data.push_back(element)
    }

    /// Adds an element first in the deque.
    pub fn push_front(&mut self, element: T) {
        self.data.push_front(element)
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
}

/*
Construct
*/

impl<T, const N: usize> From<[T; N]> for Deque<T> {
    fn from(value: [T; N]) -> Self {
        Self {
            data: std::collections::LinkedList::from(value),
        }
    }
}

impl<T> From<crate::List<T>> for Deque<T> {
    fn from(value: crate::List<T>) -> Self {
        value.into_iter().collect()
    }
}

impl<T> From<crate::Set<T>> for Deque<T> {
    fn from(value: crate::Set<T>) -> Self {
        value.into_iter().collect()
    }
}

/*
Function
*/

impl<T> std::ops::ShlAssign<usize> for Deque<T> {
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

impl<T> std::ops::ShrAssign<usize> for Deque<T> {
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

/*
Display
*/

impl<T: std::fmt::Display> std::fmt::Display for Deque<T> {
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
    type IntoIter = std::collections::linked_list::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

/*
Transform
*/

impl<T> From<std::collections::LinkedList<T>> for Deque<T> {
    fn from(value: std::collections::LinkedList<T>) -> Self {
        Self { data: value }
    }
}

impl<T> From<Deque<T>> for std::collections::LinkedList<T> {
    fn from(value: Deque<T>) -> Self {
        value.data
    }
}
