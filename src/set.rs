#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Set<T> {
    data: std::collections::BTreeSet<T>,
}

impl<T: Ord> Set<T> {
    /// Makes a new, empty `Set`.
    pub fn new() -> Self {
        Self {
            data: std::collections::BTreeSet::new(),
        }
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

    /// Adds a value to the set. Returns whether the value was newly inserted.
    pub fn add(&mut self, value: T) -> bool {
        self.data.insert(value)
    }

    /// Removes a value from the set and drops it. Returns whether such an element was present.
    pub fn remove(&mut self, value: &T) -> bool {
        self.data.remove(value)
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

impl<T: Ord, const N: usize> From<[T; N]> for Set<T> {
    fn from(value: [T; N]) -> Self {
        Self {
            data: std::collections::BTreeSet::from(value),
        }
    }
}

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

impl<T: Ord> std::cmp::PartialOrd for Set<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (
            self.data.is_subset(&other.data),
            self.data.is_superset(&other.data),
        ) {
            (true, true) => Some(std::cmp::Ordering::Equal),
            (true, false) => Some(std::cmp::Ordering::Less),
            (false, true) => Some(std::cmp::Ordering::Greater),
            _ => unreachable!(),
        }
    }
}

impl<T: Ord + Clone> std::ops::BitAnd<&Set<T>> for &Set<T> {
    type Output = Set<T>;

    fn bitand(self, rhs: &Set<T>) -> Self::Output {
        Set {
            data: &self.data & &rhs.data,
        }
    }
}

impl<T: Ord + Clone> std::ops::BitOr<&Set<T>> for &Set<T> {
    type Output = Set<T>;

    fn bitor(self, rhs: &Set<T>) -> Self::Output {
        Set {
            data: &self.data | &rhs.data,
        }
    }
}

impl<T: Ord + Clone> std::ops::BitXor<&Set<T>> for &Set<T> {
    type Output = Set<T>;

    fn bitxor(self, rhs: &Set<T>) -> Self::Output {
        Set {
            data: &self.data ^ &rhs.data,
        }
    }
}

impl<T: Ord + Clone> std::ops::Sub<&Set<T>> for &Set<T> {
    type Output = Set<T>;

    fn sub(self, rhs: &Set<T>) -> Self::Output {
        Set {
            data: &self.data - &rhs.data,
        }
    }
}

impl<T: Ord + Clone> std::ops::BitAndAssign for Set<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.data = &self.data & &rhs.data;
    }
}

impl<T: Ord + Clone> std::ops::BitOrAssign for Set<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.data = &self.data | &rhs.data;
    }
}

impl<T: Ord + Clone> std::ops::BitXorAssign for Set<T> {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.data = &self.data ^ &rhs.data;
    }
}

impl<T: Ord + Clone> std::ops::SubAssign for Set<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.data = &self.data - &rhs.data;
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Set<T> {
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

impl<T: Ord> Default for Set<T> {
    fn default() -> Self {
        Self::new()
    }
}
