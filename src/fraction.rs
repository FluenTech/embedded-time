//! Fractional/Rational values
use crate::ConversionError;
use core::ops;
use num::{rational::Ratio, CheckedDiv, CheckedMul, Zero};

/// A fractional value
///
/// Used primarily to define the _scaling factor_ for the [`Duration`], [`Rate`], [`Instant`] and
/// [`Clock`] traits and types.
///
/// [`Duration`]: duration/trait.Duration.html
/// [`Rate`]: rate/trait.Rate.html
/// [`Clock`]: clock/trait.Clock.html
/// [`Instant`]: instant/struct.Instant.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Fraction(Ratio<u32>);

impl Fraction {
    /// Construct a new `Fraction`.
    ///
    /// A reduction is **not** performed. Also there is no check for a denominator of `0`. If these
    /// features are needed, use [`Fraction::new_reduce()`]
    pub const fn new(numerator: u32, denominator: u32) -> Self {
        Self(Ratio::new_raw(numerator, denominator))
    }

    /// Return the numerator of the fraction
    pub const fn numerator(&self) -> &u32 {
        self.0.numer()
    }

    /// Return the denominator of the fraction
    pub const fn denominator(&self) -> &u32 {
        self.0.denom()
    }
}

impl Fraction {
    /// Construct a new `Fraction`.
    ///
    /// A reduction and `denominator == 0` check **are** performed.
    ///
    /// # Errors
    ///
    /// [`ConversionError::DivByZero`] : A `0` denominator was detected
    // TODO: add example
    pub fn new_reduce(numerator: u32, denominator: u32) -> Result<Self, ConversionError> {
        if !denominator.is_zero() {
            Ok(Self(Ratio::new(numerator, denominator)))
        } else {
            Err(ConversionError::DivByZero)
        }
    }

    /// Returns the value truncated to an integer
    pub fn to_integer(&self) -> u32 {
        self.0.to_integer()
    }

    /// Constructs a `Fraction` from an integer.
    ///
    /// Equivalent to `Fraction::new(value,1)`.
    pub fn from_integer(value: u32) -> Self {
        Self(Ratio::from_integer(value))
    }

    /// Returns the reciprocal of the fraction
    pub fn recip(self) -> Self {
        Self(self.0.recip())
    }

    /// Checked `Fraction` × `Fraction` = `Fraction`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, ConversionError};
    /// #
    /// assert_eq!(Fraction::new(1000, 1).checked_mul(&Fraction::new(5,5)),
    ///     Ok(Fraction::new(5_000, 5)));
    ///
    /// assert_eq!(Fraction::new(u32::MAX, 1).checked_mul(&Fraction::new(2,1)),
    ///     Err(ConversionError::Overflow));
    /// ```
    ///
    /// # Errors
    ///
    /// [`ConversionError::Overflow`]
    // TODO: add example
    pub fn checked_mul(&self, v: &Self) -> Result<Self, ConversionError> {
        Ok(Self(
            self.0.checked_mul(&v.0).ok_or(ConversionError::Overflow)?,
        ))
    }

    /// Checked `Fraction` / `Fraction` = `Fraction`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, ConversionError};
    /// #
    /// assert_eq!(Fraction::new(1000, 1).checked_div(&Fraction::new(10, 1000)),
    ///     Ok(Fraction::new(1_000_000, 10)));
    ///
    /// assert_eq!(Fraction::new(1, u32::MAX).checked_div(&Fraction::new(2,1)),
    ///     Err(ConversionError::Overflow));
    /// ```
    ///
    /// # Errors
    ///
    /// [`ConversionError::Overflow`]
    // TODO: add example
    pub fn checked_div(&self, v: &Self) -> Result<Self, ConversionError> {
        Ok(Self(
            self.0.checked_div(&v.0).ok_or(ConversionError::Overflow)?,
        ))
    }

    /// Checked `Fraction` × integer = `Fraction`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, ConversionError};
    /// #
    /// assert_eq!(Fraction::new(1000, 1).checked_mul_integer(5_u32),
    ///     Ok(Fraction::new(5_000, 1)));
    ///
    /// assert_eq!(Fraction::new(u32::MAX, 1).checked_mul_integer(2_u32),
    ///     Err(ConversionError::Overflow));
    /// ```
    ///
    /// # Errors
    ///
    /// [`ConversionError::Overflow`]
    // TODO: add example
    /// [`ConversionError::DivByZero`]
    // TODO: add example
    pub fn checked_mul_integer(&self, multiplier: u32) -> Result<Self, ConversionError> {
        Ok(Self(
            Ratio::checked_mul(&self.0, &Ratio::from_integer(multiplier))
                .ok_or(ConversionError::Overflow)?,
        ))
    }

    /// Checked `Fraction` / integer = `Fraction`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, ConversionError};
    /// #
    /// assert_eq!(Fraction::new(1000, 1).checked_div_integer(5_u32),
    ///     Ok(Fraction::new(200, 1)));
    ///
    /// assert_eq!(Fraction::new(1, 1000).checked_div_integer(5_u32),
    ///     Ok(Fraction::new(1, 5000)));
    ///
    /// assert_eq!(Fraction::new(1, u32::MAX).checked_div_integer(2_u32),
    ///     Err(ConversionError::Overflow));
    /// ```
    ///
    /// # Errors
    ///
    /// [`ConversionError::Overflow`]
    // TODO: add example
    /// [`ConversionError::DivByZero`]
    // TODO: add example
    pub fn checked_div_integer(&self, divisor: u32) -> Result<Self, ConversionError> {
        if divisor == 0 {
            Err(ConversionError::DivByZero)
        } else {
            Ok(Self(
                Ratio::checked_div(&self.0, &Ratio::from_integer(divisor))
                    .ok_or(ConversionError::Overflow)?,
            ))
        }
    }
}

impl ops::Mul for Fraction {
    type Output = Self;

    /// Panicky `Fraction` × `Fraction` = `Fraction`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::Fraction;
    /// assert_eq!(Fraction::new(1000, 1) * Fraction::new(5,5),
    ///     Fraction::new(5_000, 5));
    /// ```
    ///
    /// # Panics
    ///
    /// The same reason the integer operation would panic. Namely, if the
    /// result overflows the type.
    // TODO: add example
    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(&rhs).unwrap()
    }
}

impl ops::Div for Fraction {
    type Output = Self;

    /// Panicky `Fraction` / `Fraction` = `Fraction`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::Fraction;
    /// assert_eq!(Fraction::new(1000, 1) / Fraction::new(10, 1_000),
    ///     Fraction::new(1_000_000, 10));
    /// ```
    ///
    /// # Panics
    ///
    /// The same reason the integer operation would panic. Namely, if the
    /// result overflows the type.
    // TODO: add example
    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(&rhs).unwrap()
    }
}

impl Default for Fraction {
    fn default() -> Self {
        Self::new(1, 1)
    }
}
