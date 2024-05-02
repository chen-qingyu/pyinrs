#[inline]
pub fn check_bounds(pos: i32, begin: i32, end: i32) {
    if pos < begin || pos >= end {
        panic!("Error: Index out of range: {} not in {}..{}.", pos, begin, end);
    }
}

#[inline]
pub fn check_empty(size: usize) {
    if size == 0 {
        panic!("Error: The container is empty.");
    }
}

#[inline]
pub fn check_full(size: usize, capacity: usize) {
    if size >= capacity {
        panic!("Error: The container has reached the maximum size: {}.", capacity);
    }
}

#[inline]
pub fn calc_index(mut index: i32, size: usize) -> usize {
    if index < 0 {
        index += size as i32;
    }
    index as usize
}

#[inline]
pub fn print<I: Iterator>(f: &mut std::fmt::Formatter, iter: I, open: char, close: char) -> std::fmt::Result
where
    I::Item: std::fmt::Display,
{
    let mut it = iter.peekable();
    if it.peek().is_none() {
        return write!(f, "{open}{close}");
    }

    write!(f, "{open}")?;
    loop {
        write!(f, "{}", it.next().unwrap())?;
        if it.peek().is_none() {
            return write!(f, "{close}");
        }
        write!(f, ", ")?;
    }
}
