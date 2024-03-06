#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    // Compare two fractions.
    fn compare(&self, that: &Self) -> i32 {
        // this = a/b; that = c/d;
        // so, this - that = a/b - c/d = (ad - bc)/(bd)
        // since bd is always positive, compute (ad-bc) only
        let a = self.numerator;
        let b = self.denominator;
        let c = that.numerator;
        let d = that.denominator;

        a * d - b * c
    }

    /// Construct a new fraction object.
    pub fn new() -> Self {
        Fraction {
            numerator: 0,
            denominator: 1,
        }
    }

    /// Construct a new fraction object.
    pub fn from(numerator: i32, denominator: i32) -> Self {
        if denominator == 0 {
            panic!("Error: Zero denominator.");
        }

        let mut f = Fraction {
            numerator,
            denominator,
        };
        f.simplify();
        f
    }

    /// Convert this fraction to decimal.
    pub fn to_decimal(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    /// Return the absolute value of the copy of this.
    pub fn abs(&self) -> Self {
        if self.numerator >= 0 {
            *self
        } else {
            -*self
        }
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.compare(other) {
            ..=-1 => Some(std::cmp::Ordering::Less),
            0 => Some(std::cmp::Ordering::Equal),
            1.. => Some(std::cmp::Ordering::Greater),
        }
    }
}

impl std::ops::Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Self::Output {
        Fraction {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl std::ops::Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Self) -> Self::Output {
        Fraction::from(
            self.numerator * rhs.denominator + self.denominator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl std::ops::Sub for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Self) -> Self::Output {
        Fraction::from(
            self.numerator * rhs.denominator - self.denominator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl std::ops::Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Self) -> Self::Output {
        Fraction::from(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl std::ops::Div for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Self) -> Self::Output {
        Fraction::from(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }
}

impl std::ops::Rem for Fraction {
    type Output = Fraction;

    fn rem(self, rhs: Self) -> Self::Output {
        if rhs.numerator == 0 {
            panic!("Error: Zero denominator.");
        }

        Fraction::from(
            (self.numerator * rhs.denominator) % (rhs.numerator * self.denominator),
            self.denominator * rhs.denominator,
        )
    }
}

impl std::ops::AddAssign for Fraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign for Fraction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::MulAssign for Fraction {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::DivAssign for Fraction {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::RemAssign for Fraction {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}
