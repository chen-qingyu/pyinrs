use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

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
        if value.1 == 0 {
            panic!("Error: Zero denominator.");
        }

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
        if let Ok(numerator) = s.trim().parse::<i32>() {
            return Ok(Fraction { numerator, denominator: 1 });
        }

        let (numerator, denominator) = s.trim().split_once('/').ok_or(ParseFractionError)?;

        let numerator = numerator.parse::<i32>().map_err(|_| ParseFractionError)?;
        let denominator = denominator.parse::<i32>().map_err(|_| ParseFractionError)?;

        Ok(Fraction::from((numerator, denominator)))
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
        // self = a/b; other = c/d;
        // so, self - other = a/b - c/d = (ad - bc)/(bd)
        // since bd is always positive, compute (ad-bc) only
        let a = self.numerator;
        let b = self.denominator;
        let c = other.numerator;
        let d = other.denominator;

        match a * d - b * c {
            ..=-1 => Some(Ordering::Less),
            0 => Some(Ordering::Equal),
            1.. => Some(Ordering::Greater),
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

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from((
            self.numerator * rhs.denominator + self.denominator * rhs.numerator,
            self.denominator * rhs.denominator,
        ))
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from((
            self.numerator * rhs.denominator - self.denominator * rhs.numerator,
            self.denominator * rhs.denominator,
        ))
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from((self.numerator * rhs.numerator, self.denominator * rhs.denominator))
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::from((self.numerator * rhs.denominator, self.denominator * rhs.numerator))
    }
}

impl Rem for Fraction {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        if rhs.numerator == 0 {
            panic!("Error: Zero denominator.");
        }

        Self::from((
            (self.numerator * rhs.denominator) % (rhs.numerator * self.denominator),
            self.denominator * rhs.denominator,
        ))
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl RemAssign for Fraction {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
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
