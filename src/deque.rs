use std::collections::LinkedList;

use crate::List;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Deque<T> {
    data: LinkedList<T>,
}

impl<T> Deque<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Deque {
            data: LinkedList::new(),
        }
    }

    pub fn iter(&self) -> std::collections::linked_list::Iter<T> {
        self.data.iter()
    }

    pub fn back(&self) -> Option<&T> {
        self.data.back()
    }

    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.data.back_mut()
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.data.front_mut()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn push_back(&mut self, element: T) {
        self.data.push_back(element)
    }

    pub fn push_front(&mut self, element: T) {
        self.data.push_front(element)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn reverse(&mut self) -> &Self {
        self.data = self.data.clone().into_iter().rev().collect();
        self
    }

    pub fn to_list(&self) -> List<T> {
        List::from(self.data.clone().into_iter().collect::<Vec<T>>())
    }
}

impl<T, const N: usize> From<[T; N]> for Deque<T> {
    fn from(value: [T; N]) -> Self {
        Deque {
            data: LinkedList::from(value),
        }
    }
}

impl<T> std::ops::ShlAssign<usize> for Deque<T>
where
    T: Clone,
{
    fn shl_assign(&mut self, mut rhs: usize) {
        if self.data.len() <= 1 || rhs == 0 {
            return;
        }

        rhs %= self.data.len();

        let mut tail = self.data.split_off(rhs);
        tail.extend(self.data.clone());
        self.data = tail;
    }
}

impl<T> std::ops::ShrAssign<usize> for Deque<T>
where
    T: Clone,
{
    fn shr_assign(&mut self, mut rhs: usize) {
        if self.data.len() <= 1 || rhs == 0 {
            return;
        }

        rhs %= self.data.len();

        let mut tail = self.data.split_off(self.data.len() - rhs);
        tail.extend(self.data.clone());
        self.data = tail;
    }
}

impl<T> std::fmt::Display for Deque<T>
where
    T: std::fmt::Display,
{
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
