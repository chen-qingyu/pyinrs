use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

use regex::Regex;

use crate::Fraction;

/// Decimal provides decimal arithmetic with repeating support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Decimal {
    value: Fraction,
}

impl Decimal {
    pub const MAX: Self = Self { value: Fraction::MAX };
    pub const MIN: Self = Self { value: Fraction::MIN };
    pub const EPSILON: Self = Self { value: Fraction::EPSILON };

    /// Construct a new zero decimal.
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert to fraction.
    pub fn as_fraction(&self) -> Fraction {
        self.value
    }

    /// Return the absolute value.
    pub fn abs(&self) -> Self {
        Self::from(self.value.abs())
    }

    fn from_str(input: &str) -> Result<Self, ParseDecimalError> {
        let re_dec = Regex::new(r"^([-+])?(\d+)\.?(\d+)?~?(\d+)?#?(\d+)?$").unwrap();
        let caps = re_dec.captures(input).ok_or(ParseDecimalError)?;

        let sign = caps.get(1).map_or("+", |m| m.as_str());
        let radix = caps.get(5).map_or(10, |m| m.as_str().parse::<u32>().unwrap());
        let integral = i128::from_str_radix(&caps[2], radix).unwrap();
        let decimal = caps.get(3).map(|x| i128::from_str_radix(x.as_str(), radix).unwrap());
        let cyclic = caps.get(4).map(|x| i128::from_str_radix(x.as_str(), radix).unwrap());

        // x = c/((base^len(c)-1)*(base^len(d))) = c/(base^len(c+d)-base^len(d))
        let decimal_len = caps.get(3).map_or(0, |m| m.len()) as u32;
        let cyclic_len = caps.get(4).map_or(0, |m| m.len()) as u32;
        let scale = i128::pow(radix as i128, decimal_len);
        let repeat = i128::pow(radix as i128, decimal_len + cyclic_len) - scale;

        let value = match (integral, decimal, cyclic) {
            // integer only
            (i, None, None) => Fraction::from(i),
            // integer + decimal
            (i, Some(d), None) => Fraction::from((i * scale + d, scale)),
            // integer + cyclic
            (i, None, Some(c)) => Fraction::from(i) + Fraction::from((c, repeat)),
            // integer + decimal + cyclic
            (i, Some(d), Some(c)) => Fraction::from((i * scale + d, scale)) + Fraction::from((c, repeat)),
        };

        Ok(if sign == "-" { -Self { value } } else { Self { value } })
    }

    // a is numerator, b is denominator, return (start_index, length)
    fn find_cyclic(a: i128, b: i128) -> Option<(usize, usize)> {
        let mut remainders = Vec::new();
        let mut remainder = a % b;

        // check for repeating cycle
        while remainder != 0 {
            if remainders.contains(&remainder) {
                // found a repeating cycle
                let start = remainders.iter().position(|r| *r == remainder).unwrap();
                let length = remainders.len() - start;
                return Some((start, length));
            }
            remainders.push(remainder);
            remainder = (remainder * 10) % b; // left shift and continue iteration
        }

        // no repeating cycle found
        None
    }
}

/*
Construct
*/

macro_rules! impl_from_integer {
    ($($t:ty),+ $(,)?) => { $(
        impl From<$t> for Decimal {
            fn from(value: $t) -> Self {
                Self::from(Fraction::from(value))
            }
        }
    )+ };
}

impl_from_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64);

impl From<f64> for Decimal {
    fn from(value: f64) -> Self {
        Self::from(Fraction::from(value))
    }
}

impl From<f32> for Decimal {
    fn from(value: f32) -> Self {
        Self::from(Fraction::from(value))
    }
}

impl From<Fraction> for Decimal {
    fn from(value: Fraction) -> Self {
        Self { value }
    }
}

impl From<&str> for Decimal {
    fn from(value: &str) -> Self {
        Self::from_str(value.trim()).unwrap_or_else(|_| panic!("expect format: `[+-]integer[.decimal][~cyclic][#radix]` but got `{}`", value))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDecimalError;

impl FromStr for Decimal {
    type Err = ParseDecimalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s.trim())
    }
}

/*
Function
*/

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Neg for Decimal {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from(-self.value)
    }
}

#[auto_impl_ops::auto_ops]
impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.value + rhs.value)
    }
}

#[auto_impl_ops::auto_ops]
impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(self.value - rhs.value)
    }
}

#[auto_impl_ops::auto_ops]
impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.value * rhs.value)
    }
}

#[auto_impl_ops::auto_ops]
impl Div for Decimal {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::from(self.value / rhs.value)
    }
}

#[auto_impl_ops::auto_ops]
impl Rem for Decimal {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::from(self.value % rhs.value)
    }
}

/*
Display
*/

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some((start, length)) = Self::find_cyclic(self.value.numerator(), self.value.denominator()) {
            let s = f64::from(self.value).to_string();
            write!(f, "{}...", str::from_utf8(&s.as_bytes()[..=(s.find('.').unwrap() + start + length * 3)]).unwrap())
        } else {
            write!(f, "{}", f64::from(self.value))
        }
    }
}

/*
Transform
*/

impl From<Decimal> for f64 {
    fn from(value: Decimal) -> Self {
        f64::from(value.value)
    }
}

impl From<Decimal> for f32 {
    fn from(value: Decimal) -> Self {
        f32::from(value.value)
    }
}
