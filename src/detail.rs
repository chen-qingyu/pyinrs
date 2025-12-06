#[inline]
pub fn check_bounds(pos: i32, begin: i32, end: i32) {
    if pos < begin || pos >= end {
        panic!("Error: Index out of range: {pos} not in {begin}..{end}.");
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
        panic!("Error: The container has reached the maximum size: {capacity}.");
    }
}

#[inline]
pub fn check_zero<T: PartialEq + Default>(number: T) {
    if number == T::default() {
        panic!("Error: Divide by zero.");
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

#[inline]
pub fn gcd<T: std::ops::Rem<Output = T> + Clone + Eq + Default>(mut a: T, mut b: T) -> T {
    // using Euclidean algorithm

    // a, b = b, a % b until b == 0
    while b != T::default() {
        let t = b.clone();
        b = a % b;
        a = t;
    }
    a // a is the GCD
}
