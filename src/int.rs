use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

use rand::{distr::Uniform, Rng};

use crate::detail;

// Base radix of digits.
const BASE: i64 = 10_i64.pow(i64::MAX.ilog10()); // 1'000'000'000'000'000'000

// Number of decimal digits per chunk.
const DIGITS_PER_CHUNK: usize = BASE.ilog10() as usize; // 18

/// Int provides support for big integer arithmetic.
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Int {
    // Sign of integer, 1 is positive, -1 is negative, and 0 is zero.
    sign: i8,

    // List of chunks, represent absolute value of the integer, little endian.
    // Example: `12345678901234567890`
    // ```
    // chunk: 345678901234567890 000000000000000012
    // index: 0                  1
    // ```
    chunks: Vec<i64>,
}

impl Int {
    // Remove leading zeros and correct sign.
    fn trim(&mut self) {
        while !self.chunks.is_empty() && self.chunks.last().unwrap() == &0 {
            self.chunks.pop();
        }

        if self.chunks.is_empty() {
            self.sign = 0;
        }
    }

    // Test whether the characters represent an integer.
    fn is_integer(chars: &[u8], len: usize) -> bool {
        let have_sign = chars[0] == b'+' || chars[0] == b'-';
        if len == 0 || (len == 1 && have_sign) {
            return false;
        }

        for c in chars.iter().skip(usize::from(have_sign)) {
            if !c.is_ascii_digit() {
                return false;
            }
        }

        true
    }

    // Increase the absolute value by 1 quickly.
    fn abs_inc(&mut self) {
        assert!(self.sign != 0);

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

        self.trim(); // sign unchanged
    }

    // Decrease the absolute value by 1 quickly.
    fn abs_dec(&mut self) {
        assert!(self.sign != 0);

        let mut i = 0;
        while self.chunks[i] == 0 {
            i += 1;
        }
        self.chunks[i] -= 1;
        while i != 0 {
            i -= 1;
            self.chunks[i] = BASE - 1;
        }

        self.trim(); // sign may change to zero
    }

    // Compare absolute value.
    fn abs_cmp(&self, rhs: &[i64]) -> Ordering {
        if self.chunks.len() != rhs.len() {
            return if self.chunks.len() > rhs.len() { Ordering::Greater } else { Ordering::Less };
        }

        for (&this, &that) in self.chunks.iter().rev().zip(rhs.iter().rev()) {
            if this != that {
                return if this > that { Ordering::Greater } else { Ordering::Less };
            }
        }

        Ordering::Equal
    }

    // Multiply with small int. O(N)
    fn small_mul(&mut self, n: i64) {
        assert!(self.is_positive());
        assert!(n > 0 && n < BASE);

        let mut carry = 0;
        for chunk in self.chunks.iter_mut() {
            let tmp = *chunk as i128 * n as i128 + carry as i128;
            *chunk = (tmp % BASE as i128) as i64; // t%b < b
            carry = (tmp / BASE as i128) as i64; // t/b <= ((b-1)*(b-1) + (b-1))/b = b - 1 < b
        }
        self.chunks.push(carry);

        self.trim();
    }

    // Divide with small int. O(N)
    // Retrun the remainder.
    fn small_div(&mut self, n: i64) -> i64 {
        assert!(self.is_positive());
        assert!(n > 0 && n < BASE);

        let mut r = 0;
        for chunk in self.chunks.iter_mut().rev() {
            r = r * BASE as i128 + *chunk as i128;
            *chunk = (r / n as i128) as i64; // r/n <= ((n-1)*b+(b-1))/n = (n*b - 1)/n < b
            r %= n as i128; // r%n < r%b < b
        }

        self.trim();
        r as i64
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

        (self.chunks.len() as u32 - 1) * DIGITS_PER_CHUNK as u32 + self.chunks.last().unwrap().ilog10() + 1
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

        if self == &2.into() || self == &3.into() {
            return true;
        }

        if self.is_even() {
            return false;
        }

        let s = Int::sqrt(self);
        let mut n = Int::from(3);
        while n <= s {
            if (self % &n).is_zero() {
                return false;
            }
            n += Int::from(2);
        }

        true
    }

    /// Increase the value by 1 quickly.
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

    /// Decrease the value by 1 quickly.
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
        Self {
            sign: self.sign.abs(),
            chunks: self.chunks.clone(),
        }
    }

    /// Return the quotient and remainder simultaneously.
    /// `self == (self / rhs) * rhs + self % rhs`
    pub fn divmod(&self, rhs: &Self) -> (Self, Self) {
        // if rhs is zero, panic
        detail::check_zero(rhs.sign);

        // if this.abs < rhs.abs, just return {0, this}
        if self.digits() < rhs.digits() {
            return (0.into(), self.clone());
        }

        // now, the sign of two integers is not zero

        // if rhs < base, then use small_div in O(N)
        if rhs.chunks.len() == 1 {
            let mut a = self.abs();
            let r = a.small_div(rhs.chunks[0]); // this.abs divmod rhs.abs
            return (if self.sign == rhs.sign { a } else { -a }, Int::from(self.sign as i64 * r));
        }

        // dividend, divisor, temporary quotient, accumulated quotient
        let (mut a, mut b, mut t, mut q) = (self.abs(), rhs.abs(), Int::from(1), Int::new());

        // double ~ left shift, O(log(2^N))) * O(N) = O(N^2)
        while a.abs_cmp(&b.chunks).is_ge() {
            b.small_mul(2);
            t.small_mul(2);
        }

        // halve ~ right shift, O(log(2^N))) * O(N) = O(N^2)
        while t.is_positive() {
            if a.abs_cmp(&b.chunks).is_ge() {
                a -= &b;
                q += &t;
            }
            b.small_div(2);
            t.small_div(2);
        }

        // now q is the quotient.abs, a is the remainder.abs
        (if self.sign == rhs.sign { q } else { -q }, if self.sign == 1 { a } else { -a })
    }

    /// Return the factorial of self.
    pub fn factorial(&self) -> Self {
        if self.sign == -1 {
            panic!("Error: Require this >= 0 for factorial().");
        }

        if self.chunks.len() > 1 {
            panic!("Error: This integer is too large to calculate for factorial().");
        }

        let mut result = Self::from(1); // 0! == 1
        for i in 1..=self.to_number::<i64>() {
            result.small_mul(i);
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
            prime += Int::from(2);

            if prime.is_prime() {
                break;
            }
        }

        prime
    }

    /// Attempt to convert this integer to a number of the specified type `T`.
    pub fn to_number<T: Add<Output = T> + Mul<Output = T> + std::convert::From<i64>>(&self) -> T {
        let mut result: T = 0.into();

        for i in (0..self.chunks.len()).rev() {
            result = result * BASE.into() + self.chunks[i].into();
        }

        result * (self.sign as i64).into()
    }

    /// Return the square root of integer `n`.
    pub fn sqrt(n: &Self) -> Self {
        if n.sign == -1 {
            panic!("Error: Require n >= 0 for sqrt(n).");
        }

        // binary search
        let (mut lo, mut hi, mut res) = (Int::from(0), n.clone(), Int::default());
        while lo <= hi {
            let mid = &lo + (&hi - &lo) / Int::from(2);

            // if mid^2 <= n, update the result and search in upper half
            if &mid * &mid <= *n {
                res = mid.clone();
                lo = mid + Int::from(1);
            }
            // else mid^2 > n, search in the lower half
            else {
                hi = mid - Int::from(1);
            }
        }

        res
    }

    /// Return `base**exp`.
    pub fn pow(base: &Self, exp: &Self) -> Self {
        // check if base.abs is 1
        // if base.abs is 1, only when base is negative and exp is odd return -1, otherwise return 1
        if base.chunks.len() == 1 && base.chunks[0] == 1 {
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
        let (mut num, mut n, mut res) = (base.clone(), exp.clone(), Int::from(1));
        while !n.is_zero() {
            if n.is_odd() {
                res *= &num;
            }
            num *= num.clone();
            n.small_div(2);
        }
        res
    }

    /// Return `(base**exp) % module` faster.
    pub fn pow_mod(base: &Self, exp: &Self, module: &Self) -> Self {
        // check if base.abs is 1
        // if base.abs is 1, only when base is negative and exp is odd return -1, otherwise return 1
        if base.chunks.len() == 1 && base.chunks[0] == 1 {
            return if base.sign == -1 && exp.is_odd() { (-1).into() } else { 1.into() };
        }

        if exp.is_negative() {
            if base.is_zero() {
                panic!("Error: Math domain error.");
            }

            return Self::new();
        }

        // fast power algorithm
        let (mut num, mut n, mut res) = (base.clone(), exp.clone(), Int::from(1));
        while !n.is_zero() {
            if n.is_odd() {
                res = (&res * &num) % module;
            }
            num = (&num * &num) % module;
            n.small_div(2);
        }
        res
    }

    /// Return the logarithm of integer `n` based on `base`.
    pub fn log(n: &Self, base: &Self) -> Self {
        if n.sign <= 0 || base < &2.into() {
            panic!("Error: Math domain error.");
        }

        if base == &10.into() {
            return (n.digits() as i32 - 1).into();
        }

        let (mut num, mut res) = (n / base, Self::new());
        while !num.is_zero() {
            res.inc();
            num /= base;
        }

        res
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

    /// Generate a random integer of a specified number of `digits`.
    pub fn random(digits: usize) -> Self {
        if digits == 0 {
            panic!("Error: Require digits > 0 for random(digits).");
        }

        // random number generator
        let mut rng = rand::rng();

        // little chunks
        let mut chunks = vec![0; (digits - 1) / DIGITS_PER_CHUNK];
        let chunk = Uniform::try_from(0..BASE).unwrap();
        for d in chunks.iter_mut() {
            *d = rng.sample(chunk);
        }

        // most significant chunk
        let n = (digits - 1) % DIGITS_PER_CHUNK + 1;
        let most_chunk = Uniform::try_from(10i64.pow((n - 1) as u32)..=10i64.pow(n as u32) - 1).unwrap();
        chunks.push(rng.sample(most_chunk));

        Self { sign: 1, chunks }
    }

    /// Calculate the `n`th term of the Fibonacci sequence: 0 (n=0), 1, 1, 2, 3, 5, ...
    pub fn fibonacci(n: &Self) -> Self {
        if n.is_negative() {
            panic!("Error: Require n >= 0 for fibonacci(n).");
        }

        // ref: https://sicp-solutions.net/post/sicp-solution-exercise-1-19

        // T_pq(a, b) = (bq + aq + ap, bp + aq)
        // T_pq(T_pq(a, b)) = ((bp+aq)q + (bq+aq+ap)q + (bq+aq+ap)p, (bp+aq)p + (bq+aq+ap)q)
        //                  = (b(2pq+q^2) + a(p^2+q^2) + a(2pq+q^2), b(p^2+q^2) + a(2pq+q^2))
        //                  = T_p'q'(a, b)
        // => p' = p^2 + q^2, q' = 2pq + q^2

        let (mut a, mut b, mut p, mut q, mut cnt) = (Int::from(1), Int::from(0), Int::from(0), Int::from(1), n.clone());
        while !cnt.is_zero() {
            if cnt.is_even() {
                let p_ = &p * &p + &q * &q;
                let q_ = &p * &q * Int::from(2) + &q * &q;
                p = p_;
                q = q_;
                cnt.small_div(2);
            } else {
                let a_ = &b * &q + &a * (&p + &q);
                let b_ = &b * &p + &a * &q;
                a = a_;
                b = b_;
                cnt.abs_dec();
            }
        }
        b
    }

    /// The well-known Ackermann function (perhaps not so well-known) is a rapidly growing function.
    /// Please input parameters carefully.
    /// See: https://en.wikipedia.org/wiki/Ackermann_function
    pub fn ackermann(m: &Self, n: &Self) -> Self {
        if m.is_negative() || n.is_negative() {
            panic!("Error: Require m >= 0 and n >= 0 for ackermann(m, n).");
        }

        match m.to_number::<i64>() {
            0 => n + Int::from(1),
            1 => n + Int::from(2),
            2 => n * Int::from(2) + Int::from(3),
            3 => Int::pow(&Int::from(2), &(n + Int::from(3))) - Int::from(3),
            _ => {
                if n.is_zero() {
                    Int::ackermann(&(m - Int::from(1)), &Int::from(1))
                } else {
                    Int::ackermann(&(m - Int::from(1)), &Int::ackermann(m, &(n - Int::from(1))))
                }
            }
        }
    }

    /// The hyperoperation sequence is an infinite sequence of arithmetic operations.
    /// This sequence starts with unary successor (n = 0), continues with addition (n = 1), multiplication (n = 2), exponentiation (n = 3), etc.
    /// See: https://en.wikipedia.org/wiki/Hyperoperation
    pub fn hyperoperation(n: &Self, a: &Self, b: &Self) -> Self {
        if n.is_negative() || a.is_negative() || b.is_negative() {
            panic!("Error: Require n >= 0 and a >= 0 and b >= 0 for hyperoperation(n, a, b).");
        }

        // special cases
        if n > &3.into() {
            if a.is_zero() && b.is_even() {
                return 1.into();
            } else if a.is_zero() && b.is_odd() {
                return 0.into();
            } else if a == &1.into() || b.is_zero() {
                return 1.into();
            } else if b == &1.into() {
                return a.clone();
            } else if a == &2.into() && b == &2.into() {
                return 4.into();
            }
        }

        match n.to_number::<i64>() {
            0 => Int::from(1) + b,
            1 => a + b,
            2 => a * b,
            3 => Int::pow(a, b),
            _ => Int::hyperoperation(&(n - Int::from(1)), a, &Int::hyperoperation(n, a, &(b - Int::from(1)))),
        }
    }
}

/*
Construct
*/

impl From<&str> for Int {
    fn from(s: &str) -> Self {
        let s = s.as_bytes();
        if !Self::is_integer(s, s.len()) {
            panic!("Error: Wrong integer literal.");
        }

        let sign = if s[0] == b'-' { -1 } else { 1 };

        // skip symbol
        let digits = if s[0] == b'-' || s[0] == b'+' { &s[1..] } else { s };

        let chunks_len = (digits.len() as f64 / DIGITS_PER_CHUNK as f64).ceil() as usize;
        let mut chunks = vec![0; chunks_len];

        // every DIGITS_PER_CHUNK digits into a chunk (align right)
        let mut chunk = 0;
        let mut idx = chunks_len;
        for i in 0..digits.len() {
            chunk = chunk * 10 + (digits[i] - b'0') as i64;
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

macro_rules! from_signed {
    ($T:ty) => {
        impl From<$T> for Int {
            fn from(mut n: $T) -> Self {
                if n == 0 {
                    return Self::new();
                }

                let mut chunks = vec![];
                let sign = if n > 0 { 1 } else { -1 };
                n = n.abs();
                while n > 0 {
                    chunks.push((n as i128 % BASE as i128) as i64);
                    n = (n as i128 / BASE as i128) as $T;
                }
                Self { sign, chunks }
            }
        }
    };
}

macro_rules! from_unsigned {
    ($T:ty) => {
        impl From<$T> for Int {
            fn from(mut n: $T) -> Self {
                let mut chunks = vec![];
                let sign = if n > 0 { 1 } else { 0 };
                while n > 0 {
                    chunks.push((n as u128 % BASE as u128) as i64);
                    n = (n as u128 / BASE as u128) as $T;
                }
                Self { sign, chunks }
            }
        }
    };
}

from_signed!(i8);
from_signed!(i16);
from_signed!(i32);
from_signed!(i64);
from_signed!(i128);
from_signed!(isize);

from_unsigned!(u8);
from_unsigned!(u16);
from_unsigned!(u32);
from_unsigned!(u64);
from_unsigned!(u128);
from_unsigned!(usize);

#[derive(Debug, PartialEq, Eq)]
pub struct ParseIntError;

impl FromStr for Int {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if !Self::is_integer(s.as_bytes(), s.len()) {
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
            sign: -self.sign,
            chunks: self.chunks,
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
            a[i] = i64::rem_euclid(t, BASE);
            a[i + 1] += i64::div_euclid(t, BASE);
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
                let t = a[i] as i128 * b[j] as i128 + c[i + j] as i128;
                c[i + j] = (t % BASE as i128) as i64; // t%b < b
                c[i + j + 1] += (t / BASE as i128) as i64; // be modulo by the previous line in the next loop, or finally c + t/b <= 0 + ((b-1)^2 + (b-1))/b = b - 1 < b
            }
        }

        result.trim();
        *self = result;
    }
}

#[auto_impl_ops::auto_ops]
impl DivAssign<&Int> for Int {
    fn div_assign(&mut self, rhs: &Self) {
        *self = self.divmod(rhs).0;
    }
}

#[auto_impl_ops::auto_ops]
impl RemAssign<&Int> for Int {
    fn rem_assign(&mut self, rhs: &Self) {
        *self = self.divmod(rhs).1;
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
