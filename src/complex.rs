use std::{
    fmt::Display,
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

use regex::Regex;

use crate::utility;

/// Complex provides support for complex number arithmetic.
#[derive(Debug, Clone, Copy, Default)]
pub struct Complex {
    // Real part.
    real: f64,

    // Imaginary part.
    imag: f64,
}

impl Complex {
    /// Construct a new zero complex.
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the real part.
    pub fn real(&self) -> f64 {
        self.real
    }

    /// Return the imaginary part.
    pub fn imag(&self) -> f64 {
        self.imag
    }

    /// Return the absolute value (distance from origin) of this.
    pub fn abs(&self) -> f64 {
        f64::hypot(self.real, self.imag)
    }

    /// Return the phase angle (in radians) of this.
    pub fn arg(&self) -> f64 {
        f64::atan2(self.imag, self.real)
    }

    /// Return the conjugate value of this.
    pub fn conjugate(&self) -> Self {
        Self {
            real: self.real,
            imag: -self.imag,
        }
    }

    /// Return `base**exp`.
    pub fn pow(base: &Self, exp: &Self) -> Self {
        if exp == &0.0.into() {
            return 1.0.into();
        }

        if base == &0.0.into() {
            panic!("Error: Math domain error.");
        }

        let coef = base.abs().powf(exp.real) * (-base.arg() * exp.imag).exp();
        let theta = base.abs().ln() * exp.imag + base.arg() * exp.real;

        Self {
            real: coef * theta.cos(),
            imag: coef * theta.sin(),
        }
    }
}

/*
* Constructor
*/

impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Self { real: value, imag: 0. }
    }
}

impl From<(f64, f64)> for Complex {
    fn from(value: (f64, f64)) -> Self {
        Self { real: value.0, imag: value.1 }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseComplexError;

impl FromStr for Complex {
    type Err = ParseComplexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // handle real only
        if let Ok(real) = s.parse() {
            return Ok(Self { real, imag: 0.0 });
        }

        // handle imag only
        if s.as_bytes().last() == Some(&b'j') {
            if let Ok(imag) = s[..(s.len() - 1)].parse() {
                return Ok(Self { real: 0.0, imag });
            }
        }

        let re = Regex::new(r"^([-+]?\d+\.?\d*)([-+]?\d+\.?\d*)j$").unwrap();
        if let Some(caps) = re.captures(s) {
            if caps.len() == 3 {
                let real = caps[1].parse().map_err(|_| ParseComplexError)?;
                let imag = caps[2].parse().map_err(|_| ParseComplexError)?;
                return Ok(Self { real, imag });
            }
        }

        Err(ParseComplexError)
    }
}

/*
Function
*/

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        let epsilon = f64::EPSILON;
        (self.real - other.real).abs() < epsilon && (self.imag - other.imag).abs() < epsilon
    }
}

impl Eq for Complex {}

impl Hash for Complex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.real.to_bits().hash(state);
        self.imag.to_bits().hash(state);
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

#[auto_impl_ops::auto_ops]
impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from((self.real + rhs.real, self.imag + rhs.imag))
    }
}

#[auto_impl_ops::auto_ops]
impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from((self.real - rhs.real, self.imag - rhs.imag))
    }
}

#[auto_impl_ops::auto_ops]
impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from((self.real * rhs.real - self.imag * rhs.imag, self.real * rhs.imag + self.imag * rhs.real))
    }
}

#[auto_impl_ops::auto_ops]
impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let den = rhs.real * rhs.real + rhs.imag * rhs.imag;
        utility::check_zero(den);

        Self::from((
            (self.real * rhs.real + self.imag * rhs.imag) / den,
            (self.imag * rhs.real - self.real * rhs.imag) / den,
        ))
    }
}

/*
Display
*/

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}{:+}j)", self.real, self.imag)
    }
}
