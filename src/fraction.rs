use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

use crate::detail;

/// Fraction provides support for rational number arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fraction {
    // Numerator.
    num: i32,

    // Denominator.
    den: i32,
}

impl Fraction {
    /// Construct a new zero fraction.
    pub fn new() -> Self {
        Self { num: 0, den: 1 }
    }

    /// Return the absolute value of the fraction.
    pub fn abs(&self) -> Self {
        Self {
            num: self.num.abs(),
            den: self.den,
        }
    }

    /// Get the numerator of self.
    pub fn numerator(&self) -> i32 {
        self.num
    }

    /// Get the denominator of self.
    pub fn denominator(&self) -> i32 {
        self.den
    }

    /// Calculate the greatest common divisor of two fractions.
    pub fn gcd(a: Self, b: Self) -> Self {
        detail::gcd(a.abs(), b.abs())
    }

    /// Calculate the least common multiple of two fractions.
    pub fn lcm(a: Self, b: Self) -> Self {
        if a == 0.into() || b == 0.into() {
            return Self::new();
        }

        (a * b).abs() / Self::gcd(a, b) // LCM = |a * b| / GCD
    }
}

/*
Construct
*/

impl From<i32> for Fraction {
    fn from(value: i32) -> Self {
        Self { num: value, den: 1 }
    }
}

impl From<f64> for Fraction {
    fn from(value: f64) -> Self {
        if !value.is_finite() {
            panic!("Error: Invalid floating-point number.");
        }

        let int_part = value.floor();
        let dec_part = value - int_part;
        let precision = 1_000_000_000; // 10^floor(log10(i32::MAX))

        let gcd = detail::gcd((dec_part * precision as f64).round() as i32, precision);
        let mut num = (dec_part * precision as f64).round() as i32 / gcd;
        let den = precision / gcd;
        num += int_part as i32 * den;

        Self { num, den }
    }
}

impl From<(i32, i32)> for Fraction {
    fn from(value: (i32, i32)) -> Self {
        let (mut num, mut den) = value;

        // make sure the denominator is not zero
        detail::check_zero(den);

        // make sure the denominator is a positive number
        if den < 0 {
            num = -num;
            den = -den;
        }

        // simplify
        let gcd = detail::gcd(num.abs(), den.abs());
        num /= gcd;
        den /= gcd;

        Self { num, den }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseFractionError;

impl FromStr for Fraction {
    type Err = ParseFractionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Ok(num) = s.parse() {
            return Ok(Self { num, den: 1 });
        }

        let (num, den) = s.split_once('/').ok_or(ParseFractionError)?;

        let num = num.parse().map_err(|_| ParseFractionError)?;
        let den = den.parse().map_err(|_| ParseFractionError)?;

        Ok(Self::from((num, den)))
    }
}

impl Default for Fraction {
    fn default() -> Self {
        Self::new()
    }
}

/*
Function
*/

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        // self = a/b; other = c/d;
        // so, self - other = a/b - c/d = (ad - bc)/(bd)
        // since bd is always positive, compute (ad-bc) only

        (self.num * other.den - self.den * other.num).cmp(&0)
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { num: -self.num, den: self.den }
    }
}

#[auto_impl_ops::auto_ops]
impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from((self.num * rhs.den + self.den * rhs.num, self.den * rhs.den))
    }
}

#[auto_impl_ops::auto_ops]
impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from((self.num * rhs.den - self.den * rhs.num, self.den * rhs.den))
    }
}

#[auto_impl_ops::auto_ops]
impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from((self.num * rhs.num, self.den * rhs.den))
    }
}

#[auto_impl_ops::auto_ops]
impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::from((self.num * rhs.den, self.den * rhs.num))
    }
}

#[auto_impl_ops::auto_ops]
impl Rem for Fraction {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        detail::check_zero(rhs.num);

        Self::from(((self.num * rhs.den) % (rhs.num * self.den), self.den * rhs.den))
    }
}

/*
Display
*/

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

/*
Transform
*/

impl From<Fraction> for f64 {
    fn from(value: Fraction) -> Self {
        value.num as f64 / value.den as f64
    }
}
