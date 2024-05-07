use std::{
    collections::BTreeMap,
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::utility;

/// A Dict object maps keys to arbitrary values.
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Dict<K, V> {
    data: BTreeMap<K, V>,
}

impl<K: Ord, V> Dict<K, V> {
    /// Creates a new empty dictionary.
    pub fn new() -> Self {
        Self { data: BTreeMap::new() }
    }

    /// Returns the number of elements in the dictionary.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the dictionary contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Gets an iterator over the entries of the dictionary, sorted by key.
    pub fn iter(&self) -> std::collections::btree_map::Iter<K, V> {
        self.data.iter()
    }

    /// Return the key-value pair of the specified `key`, or `None` if the dictionary does not contain the key.
    pub fn find(&self, key: &K) -> Option<(&K, &V)> {
        self.data.iter().find(|p| p.0 == key)
    }

    /// Returns `true` if the dictionary contains a value for the specified key.
    pub fn contains(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    /// Return a reference of the value for key if key is in the dictionary, else default value.
    pub fn get<'a>(&'a self, key: &K, default: &'a V) -> &V {
        self.data.get(key).unwrap_or(default)
    }

    /// Return a new set of the dictionary's keys.
    pub fn keys(&self) -> crate::Set<K>
    where
        K: Clone,
    {
        self.data.keys().cloned().collect()
    }

    /// Return a new set of the dictionary's values.
    pub fn values(&self) -> crate::Set<V>
    where
        V: Ord + Clone,
    {
        self.data.values().cloned().collect()
    }

    /// Return a new set of the dictionary's items.
    pub fn items(&self) -> crate::Set<(K, V)>
    where
        K: Clone,
        V: Ord + Clone,
    {
        self.data.clone().into_iter().collect()
    }

    /// Adds a value to the dictionary. Returns whether the value was newly inserted.
    pub fn add(&mut self, key: K, value: V) -> bool {
        self.data.insert(key, value).is_none()
    }

    /// Removes a `key` from the dictionary. Returns `true` if such an `key` was present.
    pub fn remove(&mut self, key: &K) -> bool {
        self.data.remove(key).is_some()
    }

    /// Removes the first element from the dictionary and returns it, if any.
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.data.pop_first()
    }

    /// Clears the dictionary, removing all elements.
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

impl<K: Ord, V> Extend<(K, V)> for Dict<K, V> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        self.data.extend(iter)
    }
}

/*
Display
*/

struct Pair<K, V>(K, V); // workaround for the orphan rule

impl<K: Display, V: Display> Display for Pair<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

impl<K: Display, V: Display> Display for Dict<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        utility::print(f, self.data.iter().map(|p| Pair(p.0, p.1)), '{', '}')
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
