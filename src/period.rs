use crate::frequency::units::Hertz;
use crate::TimeInt;
use core::{cmp, ops};
use num::{rational::Ratio, CheckedDiv, CheckedMul};

/// A fractional time period
///
/// Used primarily to define the period of one count of a [`crate::Duration`] type
#[derive(Debug)]
pub struct Period<T = i32>(Ratio<T>);

impl<T> Period<T> {
    /// Construct a new fractional `Period`.
    ///
    /// A reduction is **not** performed.
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

    /// Returns an integer approximation of the Period
    ///
    /// Any remainder is truncated.
    pub fn to_integer(&self) -> T {
        self.0.to_integer()
    }

    /// Constructs a `Period` from an integer.
    ///
    /// Equivalent to `Period::new(value,1)`.
    pub fn from_integer(value: T) -> Self {
        Self(Ratio::from_integer(value))
    }

    /// ```rust
    /// # use embedded_time::Period;
    /// assert_eq!(Period::new(1000, 1).checked_mul_integer(5), Some(Period::new(5_000, 1)));
    ///
    /// assert_eq!(Period::new(i32::MAX, 1).checked_mul_integer(2), None);
    /// ```
    pub fn checked_mul_integer(&self, multiplier: T) -> Option<Self> {
        Some(Self(Ratio::checked_mul(
            &self.0,
            &Ratio::from_integer(multiplier),
        )?))
    }

    /// ```rust
    /// # use embedded_time::Period;
    /// assert_eq!(Period::new(1000, 1).checked_div_integer(5), Some(Period::new(200, 1)));
    /// assert_eq!(Period::new(1, 1000).checked_div_integer(5), Some(Period::new(1, 5000)));
    ///
    /// assert_eq!(Period::new(1, i32::MAX).checked_div_integer(2), None);
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
    /// assert_eq!(Period::new(1000, 1) * Period::new(5,5),
    ///     Period::new(5_000, 5));
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<T> num::CheckedMul for Period<T>
where
    T: TimeInt,
{
    /// ```rust
    /// # use embedded_time::Period;
    /// assert_eq!(<Period as num::CheckedMul>::checked_mul(&Period::new(1000, 1),
    ///     &Period::new(5,5)), Some(Period::new(5_000, 5)));
    ///
    /// assert_eq!(<Period as num::CheckedMul>::checked_mul(&Period::new(i32::MAX, 1),
    ///     &Period::new(2,1)), None);
    /// ```
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        Some(Self(self.0.checked_mul(&v.0)?))
    }
}

impl<T> ops::Div for Period<T>
where
    T: TimeInt,
{
    type Output = Self;

    /// ```rust
    /// # use embedded_time::Period;
    /// assert_eq!(Period::new(1000, 1) / Period::new(10, 1_000),
    ///     Period::new(1_000_000, 10));
    /// ```
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl<T> num::CheckedDiv for Period<T>
where
    T: TimeInt,
{
    /// ```rust
    /// # use embedded_time::Period;
    /// assert_eq!(<Period as num::CheckedDiv>::checked_div(&Period::new(1000, 1),
    ///     &Period::new(10, 1000)), Some(Period::new(1_000_000, 10)));
    ///
    /// assert_eq!(<Period as num::CheckedDiv>::checked_div(&Period::new(1, i32::MAX),
    ///     &Period::new(2,1)), None);
    /// ```
    fn checked_div(&self, v: &Self) -> Option<Self> {
        Some(Self(self.0.checked_div(&v.0)?))
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
