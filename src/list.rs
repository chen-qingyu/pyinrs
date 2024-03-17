use crate::utility;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct List<T> {
    data: Vec<T>,
}

impl<T> List<T> {
    /// Constructs a new, empty `List<T>`.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> i32 {
        self.data.len() as i32
    }

    /// Returns true if the list contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns an iterator over the list.
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

    /// Returns `true` if the list contains `element`.
    pub fn contains(&self, element: &T) -> bool
    where
        T: PartialEq,
    {
        self.data.contains(element)
    }

    /// Counts the number of occurrence of the specified `element`.
    pub fn count(&self, element: &T) -> i32
    where
        T: PartialEq,
    {
        self.data.iter().filter(|&x| x == element).count() as i32
    }

    /// Inserts an element at position `index (-len() <= index <= len())` within the list.
    pub fn insert(&mut self, index: i32, element: T) {
        utility::check_full(self.data.len(), i32::MAX as usize);
        utility::check_bounds(index, -self.len(), self.len() + 1);

        let index = utility::calc_index(index, self.data.len());
        self.data.insert(index, element)
    }

    /// Removes and returns the element at position `index (-len() <= index < len())` within the list.
    pub fn remove(&mut self, index: i32) -> T {
        utility::check_empty(self.data.len());
        utility::check_bounds(index, -self.len(), self.len());

        let index = utility::calc_index(index, self.data.len());
        self.data.remove(index)
    }

    /// Appends an element to the back of the list.
    pub fn push(&mut self, element: T) {
        utility::check_full(self.data.len(), i32::MAX as usize);
        self.data.push(element)
    }

    /// Removes the last element from a list and returns it, or `None` if it is empty.
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Clears the list.
    pub fn clear(&mut self) {
        self.data.clear()
    }

    /// Reverses the order of elements in the list, in place.
    pub fn reverse(&mut self) {
        self.data.reverse()
    }

    /// Eliminate duplicate elements and keep the original relative order of elements.
    pub fn uniquify(&mut self)
    where
        T: PartialEq + Clone,
    {
        let mut buffer = Vec::with_capacity(self.data.len());
        for i in 0..self.data.len() {
            if !buffer.contains(&self.data[i]) {
                buffer.push(self.data[i].clone());
            }
        }
        self.data = buffer;
    }
}

/*
Construct
*/

impl<T, const N: usize> From<[T; N]> for List<T> {
    fn from(value: [T; N]) -> Self {
        utility::check_full(N, i32::MAX as usize);
        Self { data: Vec::from(value) }
    }
}

impl<T> From<crate::Deque<T>> for List<T> {
    fn from(value: crate::Deque<T>) -> Self {
        value.into_iter().collect()
    }
}

impl<T> From<crate::Set<T>> for List<T> {
    fn from(value: crate::Set<T>) -> Self {
        value.into_iter().collect()
    }
}

/*
Function
*/

impl<T> std::ops::Index<i32> for List<T> {
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        utility::check_bounds(index, -self.len(), self.len());

        let index = utility::calc_index(index, self.data.len());
        &self.data[index]
    }
}

impl<T> std::ops::IndexMut<i32> for List<T> {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        utility::check_bounds(index, -self.len(), self.len());

        let index = utility::calc_index(index, self.data.len());
        &mut self.data[index]
    }
}

impl<T> std::ops::AddAssign for List<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.data.extend(rhs.data)
    }
}

/*
Display
*/

impl<T: std::fmt::Display> std::fmt::Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.data.len() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", self.data[i])?;
        }
        write!(f, "]")
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

/*
Transform
*/

impl<T> From<Vec<T>> for List<T> {
    fn from(value: Vec<T>) -> Self {
        utility::check_full(value.len(), i32::MAX as usize);
        Self { data: value }
    }
}

impl<T> From<List<T>> for Vec<T> {
    fn from(value: List<T>) -> Self {
        value.data
    }
}
