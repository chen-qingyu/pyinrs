#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct List<T> {
    data: Vec<T>,
}

impl<T> List<T>
where
    T: Eq + Clone,
{
    /// Constructs a new, empty `List<T>`.
    pub fn new() -> Self {
        List { data: Vec::new() }
    }

    /// Allocate a `List<T>` and fill it by cloning `elements`.
    pub fn from(elements: &[T]) -> Self {
        List {
            data: Vec::from(elements),
        }
    }

    /// Returns the number of elements in the list.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the list contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Return the first occurrence of the specified `element` or `None` in the list.
    pub fn find(&self, element: &T) -> Option<&T> {
        self.data.iter().find(|&x| x == element)
    }

    /// Returns `true` if the list contains `element`.
    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(element)
    }

    /// Counts the number of occurrence of the specified `element`.
    pub fn count(&self, element: &T) -> usize {
        self.data.iter().filter(|&x| x == element).count()
    }

    /// Inserts an element at position `index` within the list.
    pub fn insert(&mut self, index: usize, element: T) {
        self.data.insert(index, element)
    }

    /// Removes and returns the element at position `index` within the list.
    pub fn remove(&mut self, index: usize) -> T {
        self.data.remove(index)
    }

    /// Appends an element to the back of the list.
    pub fn push(&mut self, element: T) {
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
    pub fn uniquify(&mut self) {
        let mut buffer = Vec::with_capacity(self.size());
        for i in 0..self.size() {
            if !buffer.contains(&self.data[i]) {
                buffer.push(self.data[i].to_owned())
            }
        }
        self.data = buffer;
    }
}

impl<T> std::ops::Index<usize> for List<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> std::ops::IndexMut<usize> for List<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> std::ops::AddAssign for List<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.data.extend(rhs.data)
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> std::fmt::Display for List<T>
where
    T: std::fmt::Display,
{
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

impl<T> Default for List<T>
where
    T: Eq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
