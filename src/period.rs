use crate::frequency::units::Hertz;
use crate::TimeInt;
use core::{cmp, ops};
use num::{rational::Ratio, CheckedDiv, CheckedMul};

/// A fractional time period
///
/// Used primarily to define the period of one count of a [`Duration`], [`Instant`] and [`Clock`]
/// impl types but also convertible to/from [`Hertz`].
///
/// The default inner type is [`u32`].
///
/// [`Duration`]: duration/trait.Duration.html
/// [`Clock`]: clock/trait.Clock.html
/// [`Instant`]: instant/struct.Instant.html
#[derive(Debug)]
pub struct Period<T = u32>(Ratio<T>);

impl<T> Period<T> {
    /// Construct a new fractional `Period`.
    ///
    /// A reduction is **not** performed. If reduction is needed, use [`Period::new_reduce()`]
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

impl<T: TimeInt> Period<T> {
    /// Construct a new fractional `Period`.
    ///
    /// A reduction **is** performed.
    pub fn new_reduce(numerator: T, denominator: T) -> Self {
        Self(Ratio::new(numerator, denominator))
    }

    /// Returns a frequency in [`Hertz`] from the `Period`
    pub fn to_frequency(&self) -> Hertz<T> {
        Hertz(self.0.recip().to_integer())
    }

    /// Constructs a `Period` from a frequency in [`Hertz`]
    pub fn from_frequency(freq: Hertz<T>) -> Self {
        Self(Ratio::from_integer(freq.0).recip())
    }

    /// Returns the value truncated to an integer
    pub fn to_integer(&self) -> T {
        self.0.to_integer()
    }

    /// Constructs a `Period` from an integer.
    ///
    /// Equivalent to `Period::new(value,1)`.
    pub fn from_integer(value: T) -> Self {
        Self(Ratio::from_integer(value))
    }

    /// Checked `Period` * `Period` = `Period`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Period, ConversionError};
    /// #
    /// assert_eq!(<Period>::new(1000, 1).checked_mul(&<Period>::new(5,5)),
    ///     Ok(<Period>::new(5_000, 5)));
    ///
    /// assert_eq!(<Period>::new(u32::MAX, 1).checked_mul(&<Period>::new(2,1)),
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

    /// Checked `Period` / `Period` = `Period`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Period, ConversionError};
    /// #
    /// assert_eq!(<Period>::new(1000, 1).checked_div(&<Period>::new(10, 1000)),
    ///     Ok(<Period>::new(1_000_000, 10)));
    ///
    /// assert_eq!(<Period>::new(1, u32::MAX).checked_div(&<Period>::new(2,1)),
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

    /// Checked `Period` * integer = `Period`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::Period;
    /// assert_eq!(<Period>::new(1000, 1).checked_mul_integer(5_u32),
    ///     Some(<Period>::new(5_000, 1)));
    ///
    /// assert_eq!(<Period>::new(u32::MAX, 1).checked_mul_integer(2_u32),
    ///     None);
    /// ```
    pub fn checked_mul_integer(&self, multiplier: T) -> Option<Self> {
        Some(Self(Ratio::checked_mul(
            &self.0,
            &Ratio::from_integer(multiplier),
        )?))
    }

    /// ```rust
    /// # use embedded_time::Period;
    /// assert_eq!(<Period>::new(1000, 1).checked_div_integer(5_u32),
    ///     Some(<Period>::new(200, 1)));
    /// assert_eq!(<Period>::new(1, 1000).checked_div_integer(5_u32),
    ///     Some(<Period>::new(1, 5000)));
    ///
    /// assert_eq!(<Period>::new(1, u32::MAX).checked_div_integer(2_u32),
    ///     None);
    /// ```
    pub fn checked_div_integer(&self, divisor: T) -> Option<Self> {
        Some(Self(Ratio::checked_div(
            &self.0,
            &Ratio::from_integer(divisor),
        )?))
    }
}

impl<T> ops::Mul for Period<T>
where
    T: TimeInt,
{
    type Output = Self;

    /// ```rust
    /// # use embedded_time::Period;
    /// assert_eq!(Period::<u32>::new(1000, 1) * <Period>::new(5,5),
    ///     <Period>::new(5_000, 5));
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(&rhs).unwrap()
    }
}

impl<T> ops::Div for Period<T>
where
    T: TimeInt,
{
    type Output = Self;

    /// ```rust
    /// # use embedded_time::Period;
    /// assert_eq!(Period::<u32>::new(1000, 1) / <Period>::new(10, 1_000),
    ///     <Period>::new(1_000_000, 10));
    /// ```
    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(&rhs).unwrap()
    }
}

impl<T: TimeInt> PartialOrd for Period<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: TimeInt> PartialEq for Period<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
