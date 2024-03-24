use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

use crate::Str;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Int {
    // List of digits, represent absolute value of the integer.
    digits: Vec<i8>,

    // Sign of integer, 1 is positive, -1 is negative, and 0 is zero.
    sign: i8,
}

impl Int {
    // Remove leading zeros.
    fn remove_leading_zeros(&mut self) {
        while self.digits.len() > 1 && self.digits.last().unwrap() == &0 {
            self.digits.pop();
        }
    }

    // Add leading zeros.
    fn add_leading_zeros(&mut self, mut n: usize) {
        while n > 0 {
            n -= 1;
            self.digits.push(0);
        }
    }

    // Test whether the characters represent an integer.
    fn is_integer(chars: &str, len: usize) -> bool {
        let have_sign = chars.as_bytes()[0] == b'+' || chars.as_bytes()[0] == b'-';
        if len == 0 || (len == 1 && have_sign) {
            return false;
        }

        for i in usize::from(have_sign)..len {
            if !chars.chars().nth(i).unwrap().is_ascii_digit() {
                return false;
            }
        }

        true
    }

    // Construct an integer with given characters.
    fn construct(&mut self, chars: &str, len: usize) {
        if !Self::is_integer(chars, len) {
            panic!("Error: Wrong integer literal.");
        }

        self.sign = if chars.as_bytes()[0] == b'-' { -1 } else { 1 };
        let s = (chars.as_bytes()[0] == b'+') || (chars.as_bytes()[0] == b'-'); // skip symbol
        for i in (usize::from(s)..len).rev() {
            self.digits.push((chars.as_bytes()[i] - b'0') as i8);
        }

        self.remove_leading_zeros();

        if self.digits.len() == 1 && self.digits[0] == 0 {
            self.sign = 0;
        }
    }

    // Increment the absolute value by 1 quickly.
    fn abs_inc(&mut self) {
        self.digits.push(0); // add a leading zero

        let mut i = 0;
        while self.digits[i] == 9 {
            i += 1;
        }
        self.digits[i] += 1;
        while i != 0 {
            i -= 1;
            self.digits[i] = 0;
        }

        self.remove_leading_zeros();
    }

    // Decrement the absolute value by 1 quickly.
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

        self.remove_leading_zeros();

        // if result is zero, set sign to 0
        self.sign = if self.digits.len() == 1 && self.digits[0] == 0 { 0 } else { self.sign };
    }

    /// Construct a new integer object.
    pub fn new() -> Self {
        Self { digits: Vec::from([0]), sign: 0 }
    }

    /// Count the number of digits in the integer (based 10).
    pub fn digits(&self) -> usize {
        if self.sign == 0 {
            0
        } else {
            self.digits.len()
        }
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
        self.digits[0] % 2 == 0
    }

    /// Determine whether the integer is odd quickly.
    pub fn is_odd(&self) -> bool {
        self.digits[0] % 2 == 1
    }

    /// Increment the value by 1 quickly.
    pub fn inc(&mut self) -> &Self {
        if self.sign == 1 {
            self.abs_inc();
        } else if self.sign == -1 {
            self.abs_dec();
        }
        // self.sign == 0
        else {
            self.sign = 1;
            self.digits[0] = 1;
        }
        self
    }

    /// Decrement the value by 1 quickly.
    pub fn dec(&mut self) -> &Self {
        if self.sign == 1 {
            self.abs_dec();
        } else if self.sign == -1 {
            self.abs_inc();
        }
        // self.sign == 0
        else {
            self.sign = -1;
            self.digits[0] = 1;
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

    /// Return (self**exp) % module (module = 0 means does not perform module).
    pub fn pow(&self, exp: &Int, module: &Int) -> Self {
        if exp.is_negative() {
            return Self::new();
        }

        // fast power algorithm

        let mut num = self.clone();
        let mut n = exp.clone();
        let mut result = Self::from(1); // self**0 == 1

        while !n.is_zero() {
            if n.is_odd() {
                result = if module.is_zero() { &result * &num } else { &(&result * &num) % module };
            }
            num = if module.is_zero() { &num * &num } else { &(&num * &num) % module };
            n /= &Int::from(2); // integer divide
        }
        result
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
            i.dec();
        }
        result
    }

    /// Return the square root of self using Newton's method.
    pub fn sqrt(&self) -> Self {
        if self.sign == -1 {
            panic!("Error: Cannot compute square root of a negative integer.");
        }

        if self.is_zero() {
            return Self::new();
        }
        // can not be omitted, otherwise will enter an infinite loop due to precision problem
        else if self < &Self::from(4) {
            return Self::from(1);
        }

        // as far as possible to reduce the number of iterations
        let mut cur_sqrt = self / &Int::from(2);
        let mut pre_sqrt = Int::from(2);

        while cur_sqrt != pre_sqrt {
            pre_sqrt = cur_sqrt.clone();
            cur_sqrt = &(&cur_sqrt + &(self / &cur_sqrt)) / &Int::from(2);
        }

        cur_sqrt
    }

    /// Calculate the greatest common divisor of two integers using Euclidean algorithm.
    pub fn gcd(int1: &Int, int2: &Int) -> Int {
        let mut a = int1.clone();
        let mut b = int2.clone();

        // a, b = b, a % b until b == 0
        while !b.is_zero() {
            let t = b.clone();
            b = &a % &b;
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
}

/*
Construct
*/

impl From<&str> for Int {
    fn from(value: &str) -> Self {
        let mut obj = Self { digits: Vec::new(), sign: 0 };
        obj.construct(value, value.len());
        obj
    }
}

impl From<Str> for Int {
    fn from(value: Str) -> Self {
        let len = value.len() as usize;
        let mut obj = Self { digits: Vec::new(), sign: 0 };
        obj.construct(String::from(value).as_str(), len);
        obj
    }
}

impl From<i32> for Int {
    fn from(mut value: i32) -> Self {
        if value == 0 {
            return Self::new();
        }

        // value != 0
        let mut obj = Self { digits: Vec::new(), sign: 0 };
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

impl Default for Int {
    fn default() -> Self {
        Self::new()
    }
}

/*
Function
*/

impl PartialOrd for Int {
    fn partial_cmp(&self, that: &Self) -> Option<Ordering> {
        if self.sign != that.sign {
            // self is +, that is - or 0
            if self.sign == 1 {
                return Some(Ordering::Greater);
            }
            // self is -, that is + or 0
            else if self.sign == -1 {
                return Some(Ordering::Less);
            }
            // self is 0, that is + or -
            else {
                return if that.sign == 1 { Some(Ordering::Less) } else { Some(Ordering::Greater) };
            }
        }

        // the sign of two integers is the same

        if self.digits.len() != that.digits.len() {
            if self.sign == 1 {
                return if self.digits.len() > that.digits.len() {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                };
            } else {
                return if self.digits.len() > that.digits.len() {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                };
            }
        }

        for i in (0..self.digits.len()).rev() {
            if self.digits[i] != that.digits[i] {
                if self.sign == 1 {
                    return if self.digits[i] > that.digits[i] {
                        Some(Ordering::Greater)
                    } else {
                        Some(Ordering::Less)
                    };
                } else {
                    return if self.digits[i] > that.digits[i] {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Greater)
                    };
                }
            }
        }

        Some(Ordering::Equal)
    }
}

impl Neg for &Int {
    type Output = Int;

    fn neg(self) -> Self::Output {
        Int {
            digits: self.digits.clone(),
            sign: -self.sign,
        }
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

impl Add<&Int> for &Int {
    type Output = Int;

    fn add(self, rhs: &Int) -> Self::Output {
        // if one of the operands is zero, just return another one
        if self.sign == 0 || rhs.sign == 0 {
            return if self.sign == 0 { rhs.clone() } else { self.clone() };
        }

        // if the operands are of opposite signs, perform subtraction
        if self.sign == 1 && rhs.sign == -1 {
            return self - &-rhs;
        } else if self.sign == -1 && rhs.sign == 1 {
            return rhs - &-self;
        }

        // the sign of two integers is the same and not zero

        // prepare variables
        let size = std::cmp::max(self.digits.len(), rhs.digits.len()) + 1;

        let mut num1 = self.clone();
        num1.add_leading_zeros(size - 1 - num1.digits.len());

        let mut num2 = rhs.clone();
        num2.add_leading_zeros(size - 1 - num2.digits.len());

        let mut result = Int {
            digits: Vec::from([0]),
            sign: self.sign,
        };

        result.add_leading_zeros(size - 1); // result initially has a 0

        // simulate the vertical calculation
        let a = &num1.digits;
        let b = &num2.digits;
        let c = &mut result.digits;
        for i in 0..(size - 1) {
            c[i] += a[i] + b[i];
            c[i + 1] = c[i] / 10;
            c[i] %= 10;
        }

        // remove leading zeros and return result
        result.remove_leading_zeros();
        result
    }
}

impl Add for Int {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Sub<&Int> for &Int {
    type Output = Int;

    fn sub(self, rhs: &Int) -> Self::Output {
        // if one of the operands is zero
        if self.sign == 0 || rhs.sign == 0 {
            return if self.sign == 0 { -rhs } else { self.clone() };
        }

        // if the operands are of opposite signs, perform addition
        if self.sign != rhs.sign {
            return self + &-rhs;
        }

        // the sign of two integers is the same and not zero

        // prepare variables
        let size = std::cmp::max(self.digits.len(), rhs.digits.len());

        let mut num1 = self.clone();
        num1.add_leading_zeros(size - num1.digits.len());

        let mut num2 = rhs.clone();
        num2.add_leading_zeros(size - num2.digits.len());

        let mut result = Int {
            digits: Vec::from([0]),
            sign: self.sign,
        };

        // let num1.abs() >= num2.abs()
        if if self.sign == 1 { num1 < num2 } else { num1 > num2 } {
            std::mem::swap(&mut num1, &mut num2);
            result = -result;
        }
        result.add_leading_zeros(size - 1); // result initially has a 0

        // simulate the vertical calculation, assert a >= b
        let a = &mut num1.digits;
        let b = &num2.digits;
        let c = &mut result.digits;
        for i in 0..size {
            // carry
            if a[i] < b[i] {
                a[i + 1] -= 1;
                a[i] += 10;
            }
            c[i] = a[i] - b[i];
        }

        // remove leading zeros
        result.remove_leading_zeros();

        // if result is zero, set sign to 0
        result.sign = if result.digits.len() == 1 && result.digits[0] == 0 { 0 } else { result.sign };

        // return result
        result
    }
}

impl Sub for Int {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Mul<&Int> for &Int {
    type Output = Int;

    fn mul(self, rhs: &Int) -> Self::Output {
        // if one of the operands is zero, just return zero
        if self.sign == 0 || rhs.sign == 0 {
            return Int::new();
        }

        // the sign of two integers is not zero

        // prepare variables
        let size = self.digits.len() + rhs.digits.len();

        let mut result = Int {
            digits: Vec::from([0]),
            sign: if self.sign == rhs.sign { 1 } else { -1 }, // the sign is depends on the sign of operands
        };
        result.add_leading_zeros(size - 1); // result initially has a 0

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

        // remove leading zeros and return
        result.remove_leading_zeros();
        result
    }
}

impl Mul for Int {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Div<&Int> for &Int {
    type Output = Int;

    fn div(self, rhs: &Int) -> Self::Output {
        // if rhs is zero, panic
        if rhs.sign == 0 {
            panic!("Error: Divide by zero.");
        }

        // if self is zero or self.abs() < rhs.abs(), just return zero
        if self.sign == 0 || self.digits.len() < rhs.digits.len() {
            return Int::new();
        }

        // the sign of two integers is not zero

        // prepare variables
        let size = self.digits.len() - rhs.digits.len() + 1;

        let mut num1 = self.clone().abs();

        let mut tmp = Int { digits: Vec::from([0]), sign: 1 }; // intermediate variable for rhs * 10^i

        let mut result = Int {
            digits: Vec::from([0]),
            sign: if self.sign == rhs.sign { 1 } else { -1 },
        };
        result.add_leading_zeros(size - 1); // result initially has a 0

        // calculation
        let b = &rhs.digits;
        let c = &mut result.digits;
        for i in (0..size).rev() {
            tmp.digits = [0].repeat(i);
            tmp.digits.append(&mut b.clone()); // tmp = rhs * 10^i in O(N)

            // <= 9 loops, so O(1)
            while num1 >= tmp {
                c[i] += 1;
                num1 -= &tmp;
            }
        }

        // if result is zero, set sign to 0
        result.sign = if result.digits.len() == 1 && result.digits[0] == 0 { 0 } else { result.sign };

        // remove leading zeros and return
        result.remove_leading_zeros();
        result
    }
}

impl Div for Int {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        &self / &rhs
    }
}

impl Rem<&Int> for &Int {
    type Output = Int;

    fn rem(self, rhs: &Int) -> Self::Output {
        self - &(&(self / rhs) * rhs)
    }
}

impl Rem for Int {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        &self % &rhs
    }
}

impl AddAssign<&Int> for Int {
    fn add_assign(&mut self, rhs: &Int) {
        *self = &*self + rhs;
    }
}

impl SubAssign<&Int> for Int {
    fn sub_assign(&mut self, rhs: &Int) {
        *self = &*self - rhs;
    }
}

impl MulAssign<&Int> for Int {
    fn mul_assign(&mut self, rhs: &Int) {
        *self = &*self * rhs;
    }
}

impl DivAssign<&Int> for Int {
    fn div_assign(&mut self, rhs: &Int) {
        *self = &*self / rhs;
    }
}

impl RemAssign<&Int> for Int {
    fn rem_assign(&mut self, rhs: &Int) {
        *self = &*self % rhs;
    }
}

/*
Display
*/

impl Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.sign == -1 {
            write!(f, "-")?;
        }

        for i in (0..self.digits.len()).rev() {
            write!(f, "{}", (self.digits[i] as u8 + b'0') as char)?;
        }

        Ok(())
    }
}
