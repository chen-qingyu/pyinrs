use std::{
    fmt::Display,
    ops::{Add, Index, Mul},
    str::FromStr,
    string::ParseError,
};

use crate::{detail, Int, List};

/// Str is immutable sequence of characters.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Str {
    data: String,
}

impl Str {
    /// Construct an empty string.
    pub fn new() -> Self {
        Self { data: String::new() }
    }

    /// Return the length of the string, in bytes.
    pub fn len(&self) -> i32 {
        self.data.len() as i32
    }

    /// Return `true` if the string has a length of 0, and `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Extract a string slice containing the entire string.
    pub fn as_str(&self) -> &str {
        self.data.as_str()
    }

    /// Return an iterator over the chars of the string.
    pub fn chars(&self) -> std::str::Chars {
        self.data.chars()
    }

    /// Extract the `i`th character of the string. Index can be negative, like Python's str: str[-1] gets the last element.
    pub fn char_at(&self, mut index: i32) -> Option<char> {
        if index < 0 {
            index += self.data.chars().count() as i32;
            if index < 0 {
                return None;
            }
        }
        self.data.chars().nth(index as usize)
    }

    /// Return the byte index of the first character of the string that matches the pattern.
    pub fn find(&self, pattern: &str) -> Option<usize> {
        self.data.find(pattern)
    }

    /// Return `true` if the given pattern matches a sub-string of the string.
    pub fn contains(&self, pattern: &str) -> bool {
        self.data.contains(pattern)
    }

    /// Count the total number of occurrences of the specified `pattern` in the string.
    pub fn count(&self, pattern: &str) -> usize {
        if pattern.is_empty() {
            return self.data.len() + 1;
        }

        let mut cnt = 0;
        let mut start = 0;
        while let Some(pos) = self.data[start..].find(pattern) {
            cnt += 1;
            start += pos + pattern.len();
        }

        cnt
    }

    /// Convert the string to a double-precision floating-point decimal number.
    pub fn to_decimal(&self) -> Option<f64> {
        self.data.parse().ok()
    }

    /// Convert the string to an `Int`.
    pub fn to_integer(&self) -> Option<Int> {
        self.data.parse().ok()
    }

    /// Return `true` if the string starts with the specified string, otherwise return `false`.
    pub fn starts_with(&self, pattern: &str) -> bool {
        self.data.starts_with(pattern)
    }

    /// Return `true` if the string ends with the specified string, otherwise return `false`.
    pub fn ends_with(&self, pattern: &str) -> bool {
        self.data.ends_with(pattern)
    }

    /// Return the lowercase equivalent of the string.
    pub fn lower(&self) -> Self {
        Self { data: self.data.to_lowercase() }
    }

    /// Return the uppercase equivalent of the string.
    pub fn upper(&self) -> Self {
        Self { data: self.data.to_uppercase() }
    }

    /// Replace all matches of a pattern with another string.
    pub fn replace(&self, old_str: &str, new_str: &str) -> Self {
        Self {
            data: self.data.replace(old_str, new_str),
        }
    }

    /// Return a string with leading and trailing whitespace removed.
    pub fn strip(&self) -> Self {
        Self {
            data: String::from(self.data.trim()),
        }
    }

    /// Split string with `separator`.
    pub fn split(&self, separator: &str) -> List<&str> {
        self.data.split(separator).collect::<List<&str>>()
    }

    /// Return a string which is the concatenation of the strings in `str_list`.
    pub fn join(&self, str_list: List<&str>) -> Self {
        Self {
            data: str_list.data.join(self.data.as_str()),
        }
    }
}

/*
Construct
*/

impl From<&str> for Str {
    fn from(value: &str) -> Self {
        detail::check_full(value.len(), i32::MAX as usize);
        Self { data: String::from(value) }
    }
}

impl From<String> for Str {
    fn from(value: String) -> Self {
        detail::check_full(value.len(), i32::MAX as usize);
        Self { data: value }
    }
}

impl FromStr for Str {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { data: s.to_string() })
    }
}

/*
Function
*/

impl Index<i32> for Str {
    type Output = u8;

    fn index(&self, index: i32) -> &Self::Output {
        detail::check_bounds(index, -self.len(), self.len());

        let index = detail::calc_index(index, self.data.len());
        &self.data.as_bytes()[index]
    }
}

impl Add for Str {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.data + &rhs.data).into()
    }
}

impl Mul<usize> for Str {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        self.data.repeat(rhs).into()
    }
}

/*
Display
*/

impl Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", self.data)
    }
}

/*
Iterator
*/

impl FromIterator<char> for Str {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let data = iter.into_iter().collect();
        Self { data }
    }
}
