use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Dict<K, V> {
    data: BTreeMap<K, V>,
}

impl<K: Ord, V> Dict<K, V> {
    /// Makes a new, empty `Dict`.
    pub fn new() -> Self {
        Self { data: BTreeMap::new() }
    }

    /// Returns the number of elements in the dict.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the dict contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Gets an iterator over the entries of the dict, sorted by key.
    pub fn iter(&self) -> std::collections::btree_map::Iter<K, V> {
        self.data.iter()
    }

    /// Returns `true` if the dict contains a value for the specified key.
    pub fn contains(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    /// Return a reference of the value for key if key is in the dick, else default value.
    pub fn get<'a>(&'a self, key: &K, default: &'a V) -> &V {
        self.data.get(key).unwrap_or(default)
    }

    /// Return a new set of the dict's keys.
    pub fn keys(&self) -> crate::Set<K>
    where
        K: Clone,
    {
        self.data.keys().cloned().collect()
    }

    /// Return a new set of the dict's values.
    pub fn values(&self) -> crate::Set<V>
    where
        V: Ord + Clone,
    {
        self.data.values().cloned().collect()
    }

    /// Return a new set of the dict's items.
    pub fn items(&self) -> crate::Set<(K, V)>
    where
        K: Clone,
        V: Ord + Clone,
    {
        self.data.clone().into_iter().collect()
    }

    /// Adds a value to the dict. Returns whether the value was newly inserted.
    pub fn add(&mut self, key: K, value: V) -> bool {
        self.data.insert(key, value).is_none()
    }

    /// Removes a key from the dict, returning the value at the key if the key was previously in the dict.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    /// Removes the first element from the dict and returns it, if any.
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.data.pop_first()
    }

    /// Clears the dict, removing all elements.
    pub fn clear(&mut self) {
        self.data.clear()
    }
}

/*
Construct
*/

impl<K: Ord, V, const N: usize> From<[(K, V); N]> for Dict<K, V> {
    fn from(value: [(K, V); N]) -> Self {
        Self { data: BTreeMap::from(value) }
    }
}

/*
Function
*/

impl<K: Ord + Clone, V: PartialEq> PartialOrd for Dict<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.keys().partial_cmp(&other.keys())
    }
}

impl<K: Ord, V> Index<&K> for Dict<K, V> {
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        if !self.contains(key) {
            panic!("Error: Key is not found in the dict.");
        }

        &self.data[key]
    }
}

impl<K: Ord, V> IndexMut<&K> for Dict<K, V> {
    fn index_mut(&mut self, key: &K) -> &mut Self::Output {
        if !self.contains(key) {
            panic!("Error: Key is not found in the dict.");
        }

        &mut *self.data.get_mut(key).unwrap()
    }
}

/*
Display
*/

impl<K: Display, V: Display> Display for Dict<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.data.is_empty() {
            return write!(f, "{{}}");
        }

        let mut it = self.data.iter().peekable();
        write!(f, "{{")?;
        loop {
            let item = it.next().unwrap();
            write!(f, "{}: {}", item.0, item.1)?;
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

impl<K: Ord, V> FromIterator<(K, V)> for Dict<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let data = iter.into_iter().collect();
        Self { data }
    }
}

impl<K, V> IntoIterator for Dict<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::btree_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

/*
Transform
*/

impl<K, V> From<BTreeMap<K, V>> for Dict<K, V> {
    fn from(value: BTreeMap<K, V>) -> Self {
        Self { data: value }
    }
}

impl<K, V> From<Dict<K, V>> for BTreeMap<K, V> {
    fn from(value: Dict<K, V>) -> Self {
        value.data
    }
}
