use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

use rand::{distributions::Uniform, Rng};

use crate::utility;

/// Int provides support for big integer arithmetic.
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Int {
    // List of digits, represent absolute value of the integer.
    // Base 10, little endian.
    // Example: `12345000`
    // ```
    // digit: 0 0 0 5 4 3 2 1
    // index: 0 1 2 3 4 5 6 7
    // ```
    digits: Vec<i8>,

    // Sign of integer, 1 is positive, -1 is negative, and 0 is zero.
    sign: i8,
}

impl Int {
    // Remove leading zeros elegantly and correct sign.
    fn trim(&mut self) {
        let mut i = self.digits() - 1; // i = -1 if is zero, ok
        while i >= 0 && self.digits[i as usize] == 0 {
            i -= 1;
        }
        self.digits.truncate((i + 1) as usize);

        if self.digits.is_empty() {
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
        self.digits.push(0);

        let mut i = 0;
        while self.digits[i] == 9 {
            i += 1;
        }
        self.digits[i] += 1;
        while i != 0 {
            i -= 1;
            self.digits[i] = 0;
        }

        self.trim();

        // keep sign unchanged
    }

    // Decrement the absolute value by 1 quickly.
    // Require self != 0
    fn abs_dec(&mut self) {
        let mut i = 0;
        while self.digits[i] == 0 {
            i += 1;
        }
        self.digits[i] -= 1;
        while i != 0 {
            i -= 1;
            self.digits[i] = 9;
        }

        self.trim();
    }

    /// Construct a new zero integer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Count the number of digits in the integer (based 10).
    pub fn digits(&self) -> i32 {
        self.digits.len() as i32
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
            self.digits[0] & 1 == 0
        }
    }

    /// Determine whether the integer is odd quickly.
    pub fn is_odd(&self) -> bool {
        if self.is_zero() {
            false
        } else {
            self.digits[0] & 1 == 1
        }
    }

    /// Determine whether the integer is prime number.
    pub fn is_prime(&self) -> bool {
        if self <= &1.into() {
            return false; // prime >= 2
        }

        let mut i = Int::from(2);
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
            self.digits.push(1);
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
            self.digits.push(1);
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

        let mut result = Int::from(1); // 0! == 1
        let mut i = self.clone();

        // fast judgement, fast decrement
        while i.is_positive() {
            result *= &i;
            i.abs_dec();
        }

        result
    }

    /// Calculate the next prime that greater than self.
    pub fn next_prime(&self) -> Int {
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
    pub fn to_integer<T: AddAssign + MulAssign + std::convert::From<i8>>(&self) -> T {
        let mut result: T = 0.into();

        for i in (0..self.digits.len()).rev() {
            result *= 10.into();
            result += self.digits[i].into();
        }

        result *= self.sign.into();
        result
    }

    /// Return the square root of `integer`.
    pub fn sqrt(integer: &Int) -> Int {
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
        let mut digits = vec![0; integer.digits.len() / 2 - 1]; // integer.digits() >= 2
        digits.push(1);
        let mut cur_sqrt = Int { digits, sign: 1 };

        let mut pre_sqrt = Int::from(-1);

        while cur_sqrt != pre_sqrt {
            pre_sqrt = cur_sqrt.clone();
            cur_sqrt = (&cur_sqrt + integer / &cur_sqrt) / Int::from(2);
        }

        cur_sqrt
    }

    /// Return `base**exp`.
    pub fn pow(base: &Int, exp: &Int) -> Self {
        // check if base.abs() is 1
        // if base.abs() is 1, only when base is negative and exp is odd return -1, otherwise return 1
        if base.digits() == 1 && base.digits[0] == 1 {
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
            n /= Int::from(2); // integer divide
        }
        result
    }

    /// Return `(base**exp) % module` faster.
    pub fn pow_mod(base: &Int, exp: &Int, module: &Int) -> Self {
        // check if base.abs() is 1
        // if base.abs() is 1, only when base is negative and exp is odd return -1, otherwise return 1
        if base.digits() == 1 && base.digits[0] == 1 {
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
            n /= Int::from(2); // integer divide
        }
        result
    }

    /// Return the logarithm of `integer` based on `base`.
    pub fn log(integer: &Int, base: &Int) -> Int {
        if integer.sign <= 0 || base < &2.into() {
            panic!("Error: Math domain error.");
        }

        if base == &10.into() {
            return (integer.digits() - 1).into();
        }

        let mut result = Int::new();
        let mut value = integer / base;

        while !value.is_zero() {
            result.inc();
            value /= base;
        }

        result
    }

    /// Calculate the greatest common divisor of two integers.
    pub fn gcd(int1: &Int, int2: &Int) -> Int {
        // using Euclidean algorithm

        let mut a = int1.clone();
        let mut b = int2.clone();

        // a, b = b, a % b until b == 0
        while !b.is_zero() {
            let t = b.clone();
            b = a % b;
            a = t;
        }

        a // a is GCD
    }

    /// Calculate the least common multiple of two integers.
    pub fn lcm(int1: &Int, int2: &Int) -> Int {
        if int1.is_zero() || int2.is_zero() {
            return Int::new();
        }

        (int1 * int2) / Int::gcd(int1, int2) // LCM = (int1 * int2) / GCD
    }

    /// Return a non-negative random integer with a specific number of `digits`.
    pub fn random(digits: usize) -> Int {
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

        Int { digits: digits_vec, sign }
    }
}

/*
Construct
*/

impl From<&str> for Int {
    fn from(value: &str) -> Self {
        let mut obj = Self::new();
        if !Self::is_integer(value, value.len()) {
            panic!("Error: Wrong integer literal.");
        }

        obj.sign = if value.as_bytes()[0] == b'-' { -1 } else { 1 };
        let s = (value.as_bytes()[0] == b'+') || (value.as_bytes()[0] == b'-'); // skip symbol
        obj.digits = value.as_bytes().iter().skip(usize::from(s)).map(|d| (d - b'0') as i8).rev().collect();

        obj.trim();
        obj
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
            obj.digits.push((value % 10) as i8);
            value /= 10;
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

        Ok(Int::from(s))
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
            return match (self.sign, that.sign) {
                (1, _) => Ordering::Greater,  // self is +, that is - or 0
                (-1, _) => Ordering::Less,    // self is -, that is + or 0
                (0, 1) => Ordering::Less,     // self is 0, that is +
                (0, -1) => Ordering::Greater, // self is 0, that is -
                _ => unreachable!(),
            };
        }

        // the sign of two integers is the same

        let this_len = self.digits.len();
        let that_len = that.digits.len();

        if this_len != that_len {
            if self.sign == 1 {
                return if this_len > that_len { Ordering::Greater } else { Ordering::Less };
            } else {
                return if this_len > that_len { Ordering::Less } else { Ordering::Greater };
            }
        }

        for i in (0..this_len).rev() {
            if self.digits[i] != that.digits[i] {
                if self.sign == 1 {
                    return if self.digits[i] > that.digits[i] { Ordering::Greater } else { Ordering::Less };
                } else {
                    return if self.digits[i] > that.digits[i] { Ordering::Less } else { Ordering::Greater };
                }
            }
        }

        Ordering::Equal
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
            digits: self.digits,
            sign: -self.sign,
        }
    }
}

#[auto_impl_ops::auto_ops]
impl AddAssign<&Int> for Int {
    fn add_assign(&mut self, rhs: &Int) {
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

        // the sign of two integers is the same and not zero

        let size = std::cmp::max(self.digits.len(), rhs.digits.len()) + 1;
        self.digits.resize(size, 0); // the digits is max+1

        // simulate the vertical calculation, assert a.len() > b.len()
        let a = &mut self.digits;
        let b = &rhs.digits;
        for i in 0..b.len() {
            a[i] += b[i];
            if a[i] > 9 {
                a[i + 1] += 1;
                a[i] %= 10;
            }
        }
        for i in b.len()..a.len() {
            if a[i] > 9 {
                a[i + 1] += 1;
                a[i] %= 10;
            }
        }

        self.trim();
    }
}

#[auto_impl_ops::auto_ops]
impl SubAssign<&Int> for Int {
    fn sub_assign(&mut self, rhs: &Int) {
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

        // the sign of two integers is the same and not zero

        // prepare variables
        let size = std::cmp::max(self.digits.len(), rhs.digits.len());

        self.digits.resize(size, 0);

        let mut num = rhs.clone();
        num.digits.resize(size, 0);

        let mut result = Int::new();
        result.sign = self.sign; // the signs are same

        // let self.abs() >= num.abs()
        if if self.sign == 1 { *self < num } else { *self > num } {
            std::mem::swap(&mut *self, &mut num);
            result.sign = -result.sign;
        }
        result.digits.resize(size, 0);

        // simulate the vertical calculation, assert a >= b
        let a = &self.digits;
        let b = &num.digits;
        let c = &mut result.digits;
        for i in 0..size {
            c[i] += a[i] - b[i];
            // carry
            if c[i] < 0 {
                c[i + 1] -= 1;
                c[i] += 10;
            }
        }

        result.trim();
        *self = result;
    }
}

#[auto_impl_ops::auto_ops]
impl MulAssign<&Int> for Int {
    fn mul_assign(&mut self, rhs: &Int) {
        // if one of the operands is zero, just return zero
        if self.sign == 0 || rhs.sign == 0 {
            *self = 0.into();
        }

        // the sign of two integers is not zero

        // prepare variables
        let size = self.digits.len() + rhs.digits.len();

        let mut result = Int::new();
        result.sign = if self.sign == rhs.sign { 1 } else { -1 }; // the sign is depends on the sign of operands
        result.digits.resize(size, 0);

        // simulate the vertical calculation
        let a = &self.digits;
        let b = &rhs.digits;
        let c = &mut result.digits;
        for i in 0..a.len() {
            for j in 0..b.len() {
                c[i + j] += a[i] * b[j];
                c[i + j + 1] += c[i + j] / 10;
                c[i + j] %= 10;
            }
        }

        result.trim();
        *self = result;
    }
}

#[auto_impl_ops::auto_ops]
impl DivAssign<&Int> for Int {
    fn div_assign(&mut self, rhs: &Int) {
        // if rhs is zero, panic
        utility::check_zero(rhs.sign);

        // if self.abs() < rhs.abs(), just return 0
        if self.digits.len() < rhs.digits.len() {
            return *self = Int::new();
        }

        // the sign of two integers is not zero

        // prepare variables
        let size = self.digits.len() - rhs.digits.len() + 1;

        // tmp = rhs * 10^(size), not size-1, since the for loop will pop at first, so tmp is rhs * 10^(size-1) at first
        let mut digits = [0i8].repeat(size);
        digits.extend(rhs.digits.clone());
        let mut tmp = Int { digits, sign: 1 }; // intermediate variable for rhs * 10^i, positive

        let mut result = Int::new();
        result.sign = if self.sign == rhs.sign { 1 } else { -1 }; // the sign is depends on the sign of operands
        result.digits.resize(size, 0);

        self.sign = 1;

        // calculation
        for i in (0..size).rev() {
            // tmp = rhs * 10^i
            tmp.digits.remove(0); // faster than use VecDeque::pop_front()

            // <= 9 loops
            while *self >= tmp {
                result.digits[i] += 1;
                *self -= &tmp;
            }
        }

        result.trim();
        *self = result;
    }
}

#[auto_impl_ops::auto_ops]
impl RemAssign<&Int> for Int {
    fn rem_assign(&mut self, rhs: &Int) {
        // if rhs is zero, panic
        utility::check_zero(rhs.sign);

        // if self.abs() < rhs.abs(), just return self
        if self.digits.len() < rhs.digits.len() {
            return;
        }

        // the sign of two integers is not zero

        // prepare variables
        let size = self.digits.len() - rhs.digits.len() + 1;

        self.sign = 1;

        // tmp = rhs * 10^(size), not size-1, since the for loop will pop at first, so tmp is rhs * 10^(size-1) at first
        let mut digits = [0i8].repeat(size);
        digits.extend(rhs.digits.clone());
        let mut tmp = Int { digits, sign: 1 }; // intermediate variable for rhs * 10^i, positive

        // calculation
        for _ in 0..size {
            // tmp = rhs * 10^i
            tmp.digits.remove(0); // faster than use VecDeque::pop_front()

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

        for i in (0..self.digits.len()).rev() {
            write!(f, "{}", (self.digits[i] as u8 + b'0') as char)?;
        }

        Ok(())
    }
}
