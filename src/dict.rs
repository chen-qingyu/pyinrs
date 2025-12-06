use std::{
    collections::BTreeMap,
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::detail;

/// Dict maps keys to arbitrary values.
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Dict<K, V> {
    data: BTreeMap<K, V>,
}

impl<K: Ord, V> Dict<K, V> {
    /// Construct an empty dictionary.
    pub fn new() -> Self {
        Self { data: BTreeMap::new() }
    }

    /// Return the number of elements in the dictionary.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Return `true` if the dictionary contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Return an iterator over the entries of the dictionary, sorted by key.
    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, K, V> {
        self.data.iter()
    }

    /// Return the key-value pair of the specified `key`, or `None` if the dictionary does not contain the key.
    pub fn find(&self, key: &K) -> Option<(&K, &V)> {
        self.data.iter().find(|p| p.0 == key)
    }

    /// Return `true` if the dictionary contains a value for the specified key.
    pub fn contains(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    /// Return a reference of the value for `key` if `key` is in the dictionary, else `defaults` value.
    pub fn get<'a>(&'a self, key: &K, defaults: &'a V) -> &'a V {
        self.data.get(key).unwrap_or(defaults)
    }

    /// Get an iterator over the keys of the dictionary, in sorted order.
    pub fn keys(&self) -> std::collections::btree_map::Keys<'_, K, V> {
        self.data.keys()
    }

    /// Get an iterator over the values of the dictionary, in order by key.
    pub fn values(&self) -> std::collections::btree_map::Values<'_, K, V> {
        self.data.values()
    }

    /// Add the specified `key` and `value` to the dictionary. Return `true` if the `key` and `value` was newly inserted.
    pub fn add(&mut self, key: K, value: V) -> bool {
        self.data.insert(key, value).is_none()
    }

    /// Remove `key` from the dictionary. Return `true` if such an `key` was present.
    pub fn remove(&mut self, key: &K) -> bool {
        self.data.remove(key).is_some()
    }

    /// Remove the first element from the dictionary and returns it, if any.
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.data.pop_first()
    }

    /// Clear the dictionary, removing all elements.
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
        detail::print(f, self.data.iter().map(|p| Pair(p.0, p.1)), '{', '}')
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
