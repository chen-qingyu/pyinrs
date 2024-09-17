use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

use crate::utility;

/// Fraction provides support for rational number arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fraction {
    // Numerator.
    numerator: i32,

    // Denominator.
    denominator: i32,
}

impl Fraction {
    // Simplify the fraction.
    fn simplify(&mut self) {
        // make sure the denominator is a positive number
        if self.denominator < 0 {
            self.numerator = -self.numerator;
            self.denominator = -self.denominator;
        }

        // Euclid's algorithm
        let mut a = self.numerator.abs();
        let mut b = self.denominator.abs();
        while b > 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        self.numerator /= a;
        self.denominator /= a;
    }

    /// Construct a new zero fraction.
    pub fn new() -> Self {
        Self { numerator: 0, denominator: 1 }
    }

    /// Return the absolute value of the fraction.
    pub fn abs(&self) -> Self {
        if self.numerator >= 0 {
            *self
        } else {
            -*self
        }
    }

    /// Get the numerator of self.
    pub fn numerator(&self) -> i32 {
        self.numerator
    }

    /// Get the denominator of self.
    pub fn denominator(&self) -> i32 {
        self.denominator
    }

    /// Calculate the greatest common divisor of two fractions.
    pub fn gcd(x: Self, y: Self) -> Self {
        // using Euclidean algorithm

        let mut a = x;
        let mut b = y;

        // a, b = b, a % b until b == 0
        while b != 0.into() {
            let t = b; // copy
            b = a % b;
            a = t;
        }

        a // a is GCD
    }

    /// Calculate the least common multiple of two fractions.
    pub fn lcm(x: Self, y: Self) -> Self {
        if x == 0.into() || y == 0.into() {
            return Self::new();
        }

        (x * y) / Self::gcd(x, y) // LCM = (x * y) / GCD
    }
}

/*
Construct
*/

impl From<i32> for Fraction {
    fn from(value: i32) -> Self {
        Self {
            numerator: value,
            denominator: 1,
        }
    }
}

impl From<(i32, i32)> for Fraction {
    fn from(value: (i32, i32)) -> Self {
        utility::check_zero(value.1);

        let mut f = Self {
            numerator: value.0,
            denominator: value.1,
        };
        f.simplify();
        f
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseFractionError;

impl FromStr for Fraction {
    type Err = ParseFractionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(numerator) = s.trim().parse() {
            return Ok(Self { numerator, denominator: 1 });
        }

        let (numerator, denominator) = s.trim().split_once('/').ok_or(ParseFractionError)?;

        let numerator = numerator.parse().map_err(|_| ParseFractionError)?;
        let denominator = denominator.parse().map_err(|_| ParseFractionError)?;

        Ok(Self::from((numerator, denominator)))
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

        match self.numerator * other.denominator - self.denominator * other.numerator {
            ..=-1 => Ordering::Less,
            0 => Ordering::Equal,
            1.. => Ordering::Greater,
        }
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

#[auto_impl_ops::auto_ops]
impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from((
            self.numerator * rhs.denominator + self.denominator * rhs.numerator,
            self.denominator * rhs.denominator,
        ))
    }
}

#[auto_impl_ops::auto_ops]
impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from((
            self.numerator * rhs.denominator - self.denominator * rhs.numerator,
            self.denominator * rhs.denominator,
        ))
    }
}

#[auto_impl_ops::auto_ops]
impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from((self.numerator * rhs.numerator, self.denominator * rhs.denominator))
    }
}

#[auto_impl_ops::auto_ops]
impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::from((self.numerator * rhs.denominator, self.denominator * rhs.numerator))
    }
}

#[auto_impl_ops::auto_ops]
impl Rem for Fraction {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        utility::check_zero(rhs.numerator);

        Self::from((
            (self.numerator * rhs.denominator) % (rhs.numerator * self.denominator),
            self.denominator * rhs.denominator,
        ))
    }
}

/*
Display
*/

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

/*
Transform
*/

impl From<Fraction> for f64 {
    fn from(value: Fraction) -> Self {
        value.numerator as f64 / value.denominator as f64
    }
}
