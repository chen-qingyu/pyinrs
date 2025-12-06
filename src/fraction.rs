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
    num: i128,

    // Denominator.
    den: i128,
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
    pub fn numerator(&self) -> i128 {
        self.num
    }

    /// Get the denominator of self.
    pub fn denominator(&self) -> i128 {
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

    fn from_integer<T: Into<i128>>(value: T) -> Self {
        Self { num: value.into(), den: 1 }
    }

    fn from_ratio<T: Into<i128>>(num: T, den: T) -> Self {
        let (mut num, mut den) = (num.into(), den.into());

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

/*
Construct
*/

macro_rules! impl_from_integer {
    ($($t:ty),+ $(,)?) => {
        $(
            impl From<$t> for Fraction {
                fn from(value: $t) -> Self {
                    Fraction::from_integer(value)
                }
            }

            impl From<($t, $t)> for Fraction {
                fn from(value: ($t, $t)) -> Self {
                    Fraction::from_ratio(value.0, value.1)
                }
            }
        )+
    };
}

// Implement `From` for integer types, u128 is not included because it may overflow when negated
impl_from_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64);

impl From<f64> for Fraction {
    fn from(value: f64) -> Self {
        if !value.is_finite() {
            panic!("Error: Invalid floating-point number.");
        }

        let int_part = value.floor();
        let dec_part = value - int_part;
        let precision = i128::pow(10, 15); // precision as f64 may lose accuracy for very large numbers

        let gcd = detail::gcd((dec_part * (precision as f64)).round() as i128, precision);
        let mut num = (dec_part * precision as f64).round() as i128 / gcd;
        let den = precision / gcd;
        num += int_part as i128 * den;

        Self { num, den }
    }
}

impl From<f32> for Fraction {
    fn from(value: f32) -> Self {
        Fraction::from(value as f64)
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

        let num = num.parse::<i128>().map_err(|_| ParseFractionError)?;
        let den = den.parse::<i128>().map_err(|_| ParseFractionError)?;

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

impl From<Fraction> for f32 {
    fn from(value: Fraction) -> Self {
        value.num as f32 / value.den as f32
    }
}
