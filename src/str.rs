use crate::utility;
use crate::List;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Str {
    data: String,
}

impl Str {
    /// Creates a new empty `Str`.
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    /// Returns the size of this `Str`, in bytes.
    pub fn size(&self) -> i32 {
        self.data.len() as i32
    }

    /// Returns `true` if this `Str` has a size of 0, and `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the byte index of the first character of this string that matches the pattern.
    pub fn find(&self, pattern: &str) -> Option<usize> {
        self.data.find(pattern)
    }

    /// Returns `true` if the given pattern matches a sub-slice of this string.
    pub fn contains(&self, pattern: &str) -> bool {
        self.data.contains(pattern)
    }

    /// Count the total number of occurrences of the specified element in the string.
    pub fn count(&self, element: char) -> usize {
        self.data.chars().filter(|&x| x == element).count()
    }

    /// Convert the string to a double-precision floating-point decimal number.
    pub fn to_decimal(&self) -> Option<f64> {
        self.data.parse().ok()
    }

    /// Return `true` if the string begins with the specified string, otherwise return `false`.
    pub fn begin_with(&self, pattern: &str) -> bool {
        self.data.starts_with(pattern)
    }

    /// Return `true` if the string ends with the specified string, otherwise return `false`.
    pub fn end_with(&self, pattern: &str) -> bool {
        self.data.ends_with(pattern)
    }

    /// Returns the lowercase equivalent of this string, as a new `Str`.
    pub fn lower(&self) -> Self {
        Self {
            data: self.data.to_lowercase(),
        }
    }

    /// Returns the uppercase equivalent of this string, as a new `Str`.
    pub fn upper(&self) -> Self {
        Self {
            data: self.data.to_uppercase(),
        }
    }

    /// Replaces all matches of a pattern with another string.
    pub fn replace(&self, old_str: &str, new_str: &str) -> Self {
        Self {
            data: self.data.replace(old_str, new_str),
        }
    }

    /// Returns a string with leading and trailing whitespace removed.
    pub fn strip(&self) -> Self {
        Self {
            data: String::from(self.data.trim()),
        }
    }

    /// Split string with separator.
    pub fn split(&self, separator: &str) -> List<&str> {
        List::from(self.data.split(separator).collect::<Vec<&str>>())
    }

    /// Return a string which is the concatenation of the strings in str_list.
    pub fn join(&self, str_list: List<&str>) -> Self {
        let v: Vec<&str> = str_list.into();
        Self {
            data: v.join(self.data.as_str()),
        }
    }
}

impl From<&str> for Str {
    fn from(value: &str) -> Self {
        utility::check_full(value.len() as i32, i32::MAX);
        Self {
            data: String::from(value),
        }
    }
}
impl From<Str> for String {
    fn from(value: Str) -> Self {
        value.data
    }
}

impl std::ops::Index<i32> for Str {
    type Output = str;
    fn index(&self, index: i32) -> &Self::Output {
        utility::check_bounds(index, -self.size(), self.size());

        let index = utility::calc_index(index, self.data.len());
        &self.data[index..=index]
    }
}

impl std::fmt::Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.data)
    }
}

impl Default for Str {
    fn default() -> Self {
        Self::new()
    }
}
