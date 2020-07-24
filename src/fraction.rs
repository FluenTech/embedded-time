use crate::{ConversionError, TimeInt};
use core::{cmp, ops};
use num::{rational::Ratio, CheckedDiv, CheckedMul};

/// A fractional/rational value
///
/// Used primarily to define the _scaling factor_ for [`Duration`], [`Instant`], and [`Clock`]
/// impl types but also convertible to/from [`Hertz`].
///
/// The default inner type is [`u32`].
///
/// [`Duration`]: duration/trait.Duration.html
/// [`Clock`]: clock/trait.Clock.html
/// [`Instant`]: instant/struct.Instant.html
/// [`Hertz`]: rate/units/struct.Hertz.html
#[derive(Debug)]
pub struct Fraction<T = u32>(Ratio<T>);

impl<T> Fraction<T> {
    /// Construct a new fractional `Fraction`.
    ///
    /// A reduction is **not** performed. If reduction is needed, use [`Fraction::new_reduce()`]
    pub const fn new(numerator: T, denominator: T) -> Self {
        Self(Ratio::new_raw(numerator, denominator))
    }

    /// Return the numerator of the fraction
    pub const fn numerator(&self) -> &T {
        self.0.numer()
    }

    /// Return the denominator of the fraction
    pub const fn denominator(&self) -> &T {
        self.0.denom()
    }
}

impl<T: TimeInt> Fraction<T> {
    /// Construct a new fractional `Fraction`.
    ///
    /// A reduction **is** performed.
    ///
    /// # Errors
    ///
    /// [`ConversionError::DivByZero`] : A `0` denominator was detected
    pub fn new_reduce(numerator: T, denominator: T) -> Result<Self, ConversionError> {
        if !denominator.is_zero() {
            Ok(Self(Ratio::new(numerator, denominator)))
        } else {
            Err(ConversionError::DivByZero)
        }
    }

    /// Returns the value truncated to an integer
    pub fn to_integer(&self) -> T {
        self.0.to_integer()
    }

    /// Constructs a `Fraction` from an integer.
    ///
    /// Equivalent to `Fraction::new(value,1)`.
    pub fn from_integer(value: T) -> Self {
        Self(Ratio::from_integer(value))
    }

    /// Returns the reciprocal of the fraction
    pub fn recip(self) -> Self {
        Self(self.0.recip())
    }

    /// Checked `Fraction` * `Fraction` = `Fraction`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, ConversionError};
    /// #
    /// assert_eq!(<Fraction>::new(1000, 1).checked_mul(&<Fraction>::new(5,5)),
    ///     Ok(<Fraction>::new(5_000, 5)));
    ///
    /// assert_eq!(<Fraction>::new(u32::MAX, 1).checked_mul(&<Fraction>::new(2,1)),
    ///     Err(ConversionError::Overflow));
    /// ```
    ///
    /// # Errors
    ///
    /// [`ConversionError::Overflow`]
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
    /// assert_eq!(<Fraction>::new(1000, 1).checked_div(&<Fraction>::new(10, 1000)),
    ///     Ok(<Fraction>::new(1_000_000, 10)));
    ///
    /// assert_eq!(<Fraction>::new(1, u32::MAX).checked_div(&<Fraction>::new(2,1)),
    ///     Err(ConversionError::Overflow));
    /// ```
    ///
    /// # Errors
    ///
    /// [`ConversionError::Overflow`]
    pub fn checked_div(&self, v: &Self) -> Result<Self, ConversionError> {
        Ok(Self(
            self.0.checked_div(&v.0).ok_or(ConversionError::Overflow)?,
        ))
    }

    /// Checked `Fraction` * integer = `Fraction`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Fraction, ConversionError};
    /// #
    /// assert_eq!(<Fraction>::new(1000, 1).checked_mul_integer(5_u32),
    ///     Ok(<Fraction>::new(5_000, 1)));
    ///
    /// assert_eq!(<Fraction>::new(u32::MAX, 1).checked_mul_integer(2_u32),
    ///     Err(ConversionError::Overflow));
    /// ```
    ///
    /// # Errors
    ///
    /// [`ConversionError::Overflow`]
    pub fn checked_mul_integer(&self, multiplier: T) -> Result<Self, ConversionError> {
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
    /// assert_eq!(<Fraction>::new(1000, 1).checked_div_integer(5_u32),
    ///     Ok(<Fraction>::new(200, 1)));
    ///
    /// assert_eq!(<Fraction>::new(1, 1000).checked_div_integer(5_u32),
    ///     Ok(<Fraction>::new(1, 5000)));
    ///
    /// assert_eq!(<Fraction>::new(1, u32::MAX).checked_div_integer(2_u32),
    ///     Err(ConversionError::Overflow));
    /// ```
    ///
    /// # Errors
    ///
    /// [`ConversionError::Overflow`]
    pub fn checked_div_integer(&self, divisor: T) -> Result<Self, ConversionError> {
        Ok(Self(
            Ratio::checked_div(&self.0, &Ratio::from_integer(divisor))
                .ok_or(ConversionError::Overflow)?,
        ))
    }
}

impl<T> ops::Mul for Fraction<T>
where
    T: TimeInt,
{
    type Output = Self;

    /// ```rust
    /// # use embedded_time::Fraction;
    /// assert_eq!(Fraction::<u32>::new(1000, 1) * <Fraction>::new(5,5),
    ///     <Fraction>::new(5_000, 5));
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(&rhs).unwrap()
    }
}

impl<T> ops::Div for Fraction<T>
where
    T: TimeInt,
{
    type Output = Self;

    /// ```rust
    /// # use embedded_time::Fraction;
    /// assert_eq!(Fraction::<u32>::new(1000, 1) / <Fraction>::new(10, 1_000),
    ///     <Fraction>::new(1_000_000, 10));
    /// ```
    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(&rhs).unwrap()
    }
}

impl<T: TimeInt> PartialOrd for Fraction<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: TimeInt> PartialEq for Fraction<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
