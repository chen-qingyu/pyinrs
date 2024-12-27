use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

use rand::{distributions::Uniform, Rng};

use crate::detail;

// Base radix of digits.
const BASE: i32 = 10;

// Number of decimal digits per chunk.
const DIGITS_PER_CHUNK: usize = BASE.ilog10() as usize;

/// Int provides support for big integer arithmetic.
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Int {
    // Sign of integer, 1 is positive, -1 is negative, and 0 is zero.
    sign: i32,

    // List of digits, represent absolute value of the integer.
    // Base 10, little endian.
    // Example: `12345000`
    // ```
    // chunk: 0 0 0 5 4 3 2 1
    // index: 0 1 2 3 4 5 6 7
    // ```
    chunks: Vec<i32>,
}

impl Int {
    // Remove leading zeros elegantly and correct sign.
    fn trim(&mut self) {
        while !self.chunks.is_empty() && self.chunks.last().unwrap() == &0 {
            self.chunks.pop();
        }

        if self.chunks.is_empty() {
            self.sign = 0;
        }
    }

    // Test whether the characters represent an integer.
    fn is_integer(chars: &str, len: usize) -> bool {
        let have_sign = chars.as_bytes()[0] == b'+' || chars.as_bytes()[0] == b'-';
        if len == 0 || (len == 1 && have_sign) {
            return false;
        }

        for i in usize::from(have_sign)..len {
            if !chars.as_bytes()[i].is_ascii_digit() {
                return false;
            }
        }

        true
    }

    // Increment the absolute value by 1 quickly.
    // Require self != 0
    fn abs_inc(&mut self) {
        // add a leading zero for carry
        self.chunks.push(0);

        let mut i = 0;
        while self.chunks[i] == BASE - 1 {
            i += 1;
        }
        self.chunks[i] += 1;
        while i != 0 {
            i -= 1;
            self.chunks[i] = 0;
        }

        self.trim();

        // keep sign unchanged
    }

    // Decrement the absolute value by 1 quickly.
    // Require self != 0
    fn abs_dec(&mut self) {
        let mut i = 0;
        while self.chunks[i] == 0 {
            i += 1;
        }
        self.chunks[i] -= 1;
        while i != 0 {
            i -= 1;
            self.chunks[i] = BASE - 1;
        }

        self.trim();
    }

    // Compare absolute value.
    fn abs_cmp(&self, rhs: &Vec<i32>) -> Ordering {
        if self.chunks.len() != rhs.len() {
            return if self.chunks.len() > rhs.len() { Ordering::Greater } else { Ordering::Less };
        }

        for (&this, &that) in self.chunks.iter().rev().zip(rhs.iter().rev()) {
            if this != that {
                return if this > that { Ordering::Greater } else { Ordering::Less };
            }
        }

        return Ordering::Equal;
    }

    /// Construct a new zero integer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Count the number of digits in the integer (based 10).
    pub fn digits(&self) -> u32 {
        if self.chunks.is_empty() {
            return 0;
        }

        return (self.chunks.len() as u32 - 1) * DIGITS_PER_CHUNK as u32 + self.chunks.last().unwrap().ilog10() + 1;
    }

    /// Determine whether the integer is zero quickly.
    pub fn is_zero(&self) -> bool {
        self.sign == 0
    }

    /// Determine whether the integer is positive quickly.
    pub fn is_positive(&self) -> bool {
        self.sign == 1
    }

    /// Determine whether the integer is negative quickly.
    pub fn is_negative(&self) -> bool {
        self.sign == -1
    }

    /// Determine whether the integer is even quickly.
    pub fn is_even(&self) -> bool {
        if self.is_zero() {
            true
        } else {
            self.chunks[0] & 1 == 0
        }
    }

    /// Determine whether the integer is odd quickly.
    pub fn is_odd(&self) -> bool {
        if self.is_zero() {
            false
        } else {
            self.chunks[0] & 1 == 1
        }
    }

    /// Determine whether the integer is prime number.
    pub fn is_prime(&self) -> bool {
        if self <= &1.into() {
            return false; // prime >= 2
        }

        let mut i = Self::from(2);
        while &i * &i <= *self {
            if (self % &i).is_zero() {
                return false;
            }
            i.abs_inc();
        }
        true
    }

    /// Increment the value by 1 quickly.
    pub fn inc(&mut self) -> &Self {
        if self.sign == 1 {
            self.abs_inc();
        } else if self.sign == -1 {
            self.abs_dec();
        } else {
            self.sign = 1;
            self.chunks.push(1);
        }
        self
    }

    /// Decrement the value by 1 quickly.
    pub fn dec(&mut self) -> &Self {
        if self.sign == 1 {
            self.abs_dec();
        } else if self.sign == -1 {
            self.abs_inc();
        } else {
            self.sign = -1;
            self.chunks.push(1);
        }
        self
    }

    /// Return the absolute value of self.
    pub fn abs(&self) -> Self {
        if self.sign == -1 {
            -self
        } else {
            self.clone()
        }
    }

    /// Return the factorial of self.
    pub fn factorial(&self) -> Self {
        if self.sign == -1 {
            panic!("Error: Negative integer have no factorial.");
        }

        let mut result = Self::from(1); // 0! == 1
        let mut i = self.clone();

        // fast judgement, fast decrement
        while i.is_positive() {
            result *= &i;
            i.abs_dec();
        }

        result
    }

    /// Calculate the next prime that greater than self.
    pub fn next_prime(&self) -> Self {
        if *self < 2.into() {
            return 2.into();
        }

        let mut prime = self.clone(); // >= 2

        // if prime is even, let it odd and < self, because prime > 2 is odd and while prime += 2
        if prime.is_even() {
            prime.abs_dec();
        }

        // prime >= 1
        loop {
            // faster than prime += 2
            prime.abs_inc();
            prime.abs_inc();

            if prime.is_prime() {
                break;
            }
        }

        prime
    }

    /// Convert the integer to some integer of type T.
    pub fn to_integer<T: AddAssign + MulAssign + std::convert::From<i32>>(&self) -> T {
        let mut result: T = 0.into();

        for i in (0..self.chunks.len()).rev() {
            result *= BASE.into();
            result += self.chunks[i].into();
        }

        result *= self.sign.into();
        result
    }

    /// Return the square root of `integer`.
    pub fn sqrt(integer: &Self) -> Self {
        if integer.sign == -1 {
            panic!("Error: Cannot compute square root of a negative integer.");
        }

        if integer.is_zero() {
            return Self::new();
        } else if integer < &Self::from(4) {
            return Self::from(1);
        } else if integer < &Self::from(9) {
            return Self::from(2);
        } else if integer < &Self::from(16) {
            return Self::from(3); // can not be omitted
        }

        // using Newton's method

        // as far as possible to reduce the number of iterations
        // cur_sqrt = 10^(digits/2 - 1) in O(1)
        let mut digits = vec![0; integer.chunks.len() / 2 - 1]; // integer.digits() >= 2
        digits.push(1);
        let mut cur_sqrt = Self { chunks: digits, sign: 1 };

        let mut pre_sqrt = Self::from(-1);

        while cur_sqrt != pre_sqrt {
            pre_sqrt = cur_sqrt.clone();
            cur_sqrt = (&cur_sqrt + integer / &cur_sqrt) / Self::from(2);
        }

        cur_sqrt
    }

    /// Return `base**exp`.
    pub fn pow(base: &Self, exp: &Self) -> Self {
        // check if base.abs() is 1
        // if base.abs() is 1, only when base is negative and exp is odd return -1, otherwise return 1
        if base.digits() == 1 && base.chunks[0] == 1 {
            return if base.sign == -1 && exp.is_odd() { (-1).into() } else { 1.into() };
        }

        // then, check if exp is negative
        if exp.is_negative() {
            if base.is_zero() {
                panic!("Error: Math domain error.");
            }

            return Self::new();
        }

        // fast power algorithm

        let mut num = base.clone();
        let mut n = exp.clone();
        let mut result = Self::from(1); // base**0 == 1

        while !n.is_zero() {
            if n.is_odd() {
                result *= &num;
            }
            num *= num.clone();
            n /= Self::from(2); // integer divide
        }
        result
    }

    /// Return `(base**exp) % module` faster.
    pub fn pow_mod(base: &Self, exp: &Self, module: &Self) -> Self {
        // check if base.abs() is 1
        // if base.abs() is 1, only when base is negative and exp is odd return -1, otherwise return 1
        if base.digits() == 1 && base.chunks[0] == 1 {
            return if base.sign == -1 && exp.is_odd() { (-1).into() } else { 1.into() };
        }

        if exp.is_negative() {
            if base.is_zero() {
                panic!("Error: Math domain error.");
            }

            return Self::new();
        }

        // fast power algorithm

        let mut num = base.clone();
        let mut n = exp.clone();
        let mut result = Self::from(1); // base**0 == 1

        while !n.is_zero() {
            if n.is_odd() {
                result = (&result * &num) % module;
            }
            num = (&num * &num) % module;
            n /= Self::from(2); // integer divide
        }
        result
    }

    /// Return the logarithm of `integer` based on `base`.
    pub fn log(integer: &Self, base: &Self) -> Self {
        if integer.sign <= 0 || base < &2.into() {
            panic!("Error: Math domain error.");
        }

        if base == &10.into() {
            return (integer.digits() as i32 - 1).into();
        }

        let mut result = Self::new();
        let mut value = integer / base;

        while !value.is_zero() {
            result.inc();
            value /= base;
        }

        result
    }

    /// Calculate the greatest common divisor of two integers.
    pub fn gcd(a: &Self, b: &Self) -> Self {
        detail::gcd(a.abs(), b.abs())
    }

    /// Calculate the least common multiple of two integers.
    pub fn lcm(a: &Self, b: &Self) -> Self {
        if a.is_zero() || b.is_zero() {
            return Self::new();
        }

        (a * b).abs() / Self::gcd(a, b) // LCM = |a * b| / GCD
    }

    /// Return a non-negative random integer with a specific number of `digits`.
    pub fn random(digits: usize) -> Self {
        let mut rng = rand::thread_rng();

        let mut digits_vec = vec![0; digits]; // may be 0
        let sign = if digits_vec.is_empty() { 0 } else { 1 };

        let digit = Uniform::from(0..=9);
        for d in digits_vec.iter_mut() {
            *d = rng.sample(digit);
        }

        // reset most significant digit if is 0
        if digits_vec.last() == Some(&0) {
            *digits_vec.last_mut().unwrap() = rng.sample(Uniform::from(1..=9));
        }

        Self { chunks: digits_vec, sign }
    }
}

/*
Construct
*/

impl From<&str> for Int {
    fn from(value: &str) -> Self {
        if !Self::is_integer(value, value.len()) {
            panic!("Error: Wrong integer literal.");
        }

        let value = value.as_bytes();

        let sign = if value[0] == b'-' { -1 } else { 1 };

        // skip symbol
        let digits = if value[0] == b'-' || value[0] == b'+' { &value[1..] } else { &value };

        let chunks_len = (digits.len() as f64 / DIGITS_PER_CHUNK as f64).ceil() as usize;
        let mut chunks = vec![0; chunks_len];

        // every DIGITS_PER_CHUNK digits into a chunk (align right)
        let mut chunk = 0;
        let mut idx = chunks_len;
        for i in 0..digits.len() {
            chunk = chunk * 10 + (digits[i] - b'0') as i32;
            // I think maybe it's not the fastest, but it's the most elegant
            if (i + 1) % DIGITS_PER_CHUNK == digits.len() % DIGITS_PER_CHUNK {
                idx -= 1;
                chunks[idx] = chunk;
                chunk = 0;
            }
        }

        let mut r = Self { sign, chunks };
        r.trim();
        r
    }
}

impl From<i32> for Int {
    fn from(mut value: i32) -> Self {
        if value == 0 {
            return Self::new();
        }

        // value != 0
        let mut obj = Self::new();
        obj.sign = if value > 0 { 1 } else { -1 };
        value = value.abs();
        while value > 0 {
            obj.chunks.push(value % BASE);
            value /= BASE;
        }
        obj
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseIntError;

impl FromStr for Int {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if !Self::is_integer(s, s.len()) {
            return Err(ParseIntError);
        }

        Ok(Self::from(s))
    }
}

/*
Function
*/

impl PartialOrd for Int {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Int {
    fn cmp(&self, that: &Self) -> Ordering {
        if self.sign != that.sign {
            return match self.sign - that.sign {
                (1..) => Ordering::Greater,
                0 => Ordering::Equal,
                (..=-1) => Ordering::Less,
            };
        }

        // the sign of two integers is the same

        if self.sign >= 0 {
            self.abs_cmp(&that.chunks)
        } else {
            self.abs_cmp(&that.chunks).reverse()
        }
    }
}

impl Neg for &Int {
    type Output = Int;

    fn neg(self) -> Self::Output {
        self.clone().neg()
    }
}

impl Neg for Int {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            chunks: self.chunks,
            sign: -self.sign,
        }
    }
}

#[auto_impl_ops::auto_ops]
impl AddAssign<&Int> for Int {
    fn add_assign(&mut self, rhs: &Self) {
        // if one of the operands is zero, just return another one
        if self.sign == 0 || rhs.sign == 0 {
            return if self.sign == 0 {
                *self = rhs.clone();
            };
        }

        // if the operands are of opposite signs, perform subtraction
        if self.sign != rhs.sign {
            return *self -= &-rhs;
        }

        // now, the sign of two integers is the same and not zero

        // normalize
        let a = &mut self.chunks;
        let b = &rhs.chunks;
        a.resize(a.len().max(rhs.chunks.len()) + 1, 0); // a.len is max+1

        // calculate
        for i in 0..b.len() {
            let t = a[i] + b[i];
            a[i] = t % BASE;
            a[i + 1] += t / BASE;
        }
        for i in b.len()..a.len() {
            if a[i] >= BASE {
                a[i + 1] += 1;
                a[i] = 0;
            }
        }

        self.trim();
    }
}

#[auto_impl_ops::auto_ops]
impl SubAssign<&Int> for Int {
    fn sub_assign(&mut self, rhs: &Self) {
        // if one of the operands is zero
        if self.sign == 0 || rhs.sign == 0 {
            return if self.sign == 0 {
                *self = -rhs;
            };
        }

        // if the operands are of opposite signs, perform addition
        if self.sign != rhs.sign {
            return *self += &-rhs;
        }

        // now, the sign of two integers is the same and not zero

        // normalize
        let mut rhs = rhs.chunks.clone();
        if self.abs_cmp(&rhs) == Ordering::Less {
            self.sign = -self.sign;
            std::mem::swap(&mut self.chunks, &mut rhs);
        }
        let a = &mut self.chunks;
        let b = &rhs;
        a.push(0);

        // calculate
        for i in 0..b.len() {
            let t = a[i] - b[i];
            a[i] = i32::rem_euclid(t, BASE);
            a[i + 1] += i32::div_euclid(t, BASE);
        }
        for i in b.len()..a.len() {
            if a[i] < 0 {
                a[i + 1] -= 1;
                a[i] = BASE - 1;
            }
        }

        self.trim();
    }
}

#[auto_impl_ops::auto_ops]
impl MulAssign<&Int> for Int {
    fn mul_assign(&mut self, rhs: &Self) {
        // if one of the operands is zero, just return zero
        if self.sign == 0 || rhs.sign == 0 {
            *self = 0.into();
        }

        // now, the sign of two integers is not zero

        // normalize
        let a = &self.chunks;
        let b = &rhs.chunks;
        let mut result = Self {
            sign: if self.sign == rhs.sign { 1 } else { -1 },
            chunks: vec![0; a.len() + b.len()],
        };
        let c = &mut result.chunks;

        // calculate
        for i in 0..a.len() {
            for j in 0..b.len() {
                c[i + j] += a[i] * b[j];
                c[i + j + 1] += c[i + j] / BASE;
                c[i + j] %= BASE;
            }
        }

        result.trim();
        *self = result;
    }
}

#[auto_impl_ops::auto_ops]
impl DivAssign<&Int> for Int {
    fn div_assign(&mut self, rhs: &Self) {
        // if rhs is zero, panic
        detail::check_zero(rhs.sign);

        // if self.abs() < rhs.abs(), just return 0
        if self.chunks.len() < rhs.chunks.len() {
            return *self = Self::new();
        }

        // the sign of two integers is not zero

        // prepare variables
        let size = self.chunks.len() - rhs.chunks.len() + 1;

        // tmp = rhs * 10^(size), not size-1, since the for loop will pop at first, so tmp is rhs * 10^(size-1) at first
        let mut chunks = [0].repeat(size);
        chunks.extend(rhs.chunks.clone());
        let mut tmp = Self { chunks, sign: 1 }; // intermediate variable for rhs * 10^i, positive

        let mut result = Self::new();
        result.sign = if self.sign == rhs.sign { 1 } else { -1 }; // the sign is depends on the sign of operands
        result.chunks.resize(size, 0);

        self.sign = 1;

        // calculation
        for i in (0..size).rev() {
            // tmp = rhs * 10^i
            tmp.chunks.remove(0); // faster than use VecDeque::pop_front()

            // <= 9 loops
            while *self >= tmp {
                result.chunks[i] += 1;
                *self -= &tmp;
            }
        }

        result.trim();
        *self = result;
    }
}

#[auto_impl_ops::auto_ops]
impl RemAssign<&Int> for Int {
    fn rem_assign(&mut self, rhs: &Self) {
        // if rhs is zero, panic
        detail::check_zero(rhs.sign);

        // if self.abs() < rhs.abs(), just return self
        if self.chunks.len() < rhs.chunks.len() {
            return;
        }

        // the sign of two integers is not zero

        // prepare variables
        let size = self.chunks.len() - rhs.chunks.len() + 1;

        self.sign = 1;

        // tmp = rhs * 10^(size), not size-1, since the for loop will pop at first, so tmp is rhs * 10^(size-1) at first
        let mut chunks = [0].repeat(size);
        chunks.extend(rhs.chunks.clone());
        let mut tmp = Self { chunks, sign: 1 }; // intermediate variable for rhs * 10^i, positive

        // calculation
        for _ in 0..size {
            // tmp = rhs * 10^i
            tmp.chunks.remove(0); // faster than use VecDeque::pop_front()

            // <= 9 loops
            while *self >= tmp {
                *self -= &tmp;
            }
        }

        self.trim();
    }
}

/*
Display
*/

impl Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.sign == 0 {
            return write!(f, "0");
        }

        if self.sign == -1 {
            write!(f, "-")?;
        }

        write!(f, "{}", self.chunks.last().unwrap())?;
        for i in (0..self.chunks.len() - 1).rev() {
            write!(f, "{:0W$}", self.chunks[i], W = DIGITS_PER_CHUNK)?;
        }

        Ok(())
    }
}
