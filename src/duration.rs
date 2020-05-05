use crate::numerical_traits::NumericalDuration;
use crate::ratio::{IntTrait, Integer};
use crate::Ratio;
use core::fmt;
use core::ops;

/// A time duration with a fractional period in seconds
///
/// It replicates many of the `as_` and `from_` methods found on the [`core::time::Duration`](https://doc.rust-lang.org/core/time/struct.Duration.html) type.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialOrd)]
pub struct Duration<R: IntTrait> {
    value: R,
    period: Ratio<R>,
}

impl<R: IntTrait + NumericalDuration> Duration<R> {
    /// The number of seconds in one minute.
    const SECONDS_PER_MINUTE: u8 = 60;

    /// The number of seconds in one hour.
    const SECONDS_PER_HOUR: u8 = 60 * Self::SECONDS_PER_MINUTE;

    pub const fn new(value: R, period: Ratio<R>) -> Self {
        Self { value, period }
    }

    // /// Equivalent to `0.seconds()`.
    // ///
    // /// ```rust
    // /// # use embedded_time::{Duration, prelude::*};
    // /// assert_eq!(Duration::zero(), 0.seconds());
    // /// ```
    // #[inline(always)]
    // pub const fn zero() -> Self {
    //     Self::from_secs(0)
    // }

    /// The maximum possible duration. Adding any positive duration to this will
    /// cause an overflow.
    ///
    /// ```rust
    /// use embedded_time::{Ratio, Duration};
    /// let max = Duration::max_value(Ratio::new(1,1));
    /// assert_eq!(max.as_secs(), i32::MAX);
    /// ```
    ///
    /// The value returned by this method may change at any time.
    #[inline(always)]
    pub fn max_value(period: Ratio<R>) -> Self {
        Self {
            value: R::max_value(),
            period,
        }
    }

    // /// The minimum possible duration. Adding any negative duration to this will
    // /// cause an overflow.
    // ///
    // /// The value returned by this method may change at any time.
    // #[inline(always)]
    // pub const fn min_value() -> Self {
    //     Self {
    //         value: Integer::min_value(),
    //     }
    // }

    // /// Create a new `Duration` with the given number of hours. Equivalent to
    // /// `Duration::seconds(hours * 3_600)`.
    // ///
    // /// ```rust
    // /// # use embedded_time::{Duration, prelude::*};
    // /// assert_eq!(Duration::from_hours(1), 3_600.seconds());
    // /// ```
    // #[inline(always)]
    // pub const fn from_hours(hours: i64) -> Self {
    //     Self::from_secs(hours * Self::SECONDS_PER_HOUR)
    // }
    //
    // /// Get the number of whole hours in the duration.
    // ///
    // /// ```rust
    // /// # use embedded_time::{Duration, prelude::*};
    // /// assert_eq!(1.hours().as_hours(), 1);
    // /// assert_eq!((-1).hours().as_hours(), -1);
    // /// assert_eq!(59.minutes().as_hours(), 0);
    // /// assert_eq!((-59).minutes().as_hours(), 0);
    // /// ```
    // #[inline(always)]
    // pub const fn as_hours(self) -> i64 {
    //     self.as_secs() / Self::SECONDS_PER_HOUR
    // }
    //
    // /// Create a new `Duration` with the given number of minutes. Equivalent to
    // /// `Duration::seconds(minutes * 60)`.
    // ///
    // /// ```rust
    // /// # use embedded_time::{Duration, prelude::*};
    // /// assert_eq!(Duration::from_mins(1), 60.seconds());
    // /// ```
    // #[inline(always)]
    // pub const fn from_mins(minutes: i64) -> Self {
    //     Self::from_secs(minutes * Self::SECONDS_PER_MINUTE)
    // }
    //
    // /// Get the number of whole minutes in the duration.
    // ///
    // /// ```rust
    // /// # use embedded_time::{Duration, prelude::*};
    // /// assert_eq!(1.minutes().as_mins(), 1);
    // /// assert_eq!((-1).minutes().as_mins(), -1);
    // /// assert_eq!(59.seconds().as_mins(), 0);
    // /// assert_eq!((-59).seconds().as_mins(), 0);
    // /// ```
    // #[inline(always)]
    // pub const fn as_mins(self) -> i64 {
    //     self.as_secs() / Self::SECONDS_PER_MINUTE
    // }

    /// Create a new `Duration` with the given number of seconds.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(Duration::from_secs(1).as_millis(), 1_000.milliseconds().as_millis());
    /// ```
    #[inline(always)]
    pub fn from_secs(seconds: R) -> Self {
        Self {
            value: seconds,
            period: Ratio::new(R::from(1).unwrap(), R::from(1).unwrap()),
        }
    }

    /// Get the number of whole seconds in the duration.
    ///
    /// ```rust
    /// # use embedded_time::prelude::*;
    /// assert_eq!(1.seconds().as_secs(), 1);
    /// assert_eq!((-1).seconds().as_secs(), -1);
    /// //assert_eq!(1.minutes().as_secs(), 60);
    /// //assert_eq!((-1).minutes().as_secs(), -60);
    /// ```
    #[inline(always)]
    pub const fn as_secs(self) -> R {
        self.value
    }

    /// Create a new `Duration` with the given number of milliseconds.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(Duration::from_millis(1_000), 1.seconds());
    /// assert_eq!(Duration::from_millis(-1_000), (-1).seconds());
    /// ```
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn from_millis(milliseconds: R) -> Self {
        Self {
            value: milliseconds,
            period: Ratio::<R>::new(R::from(1).unwrap(), R::from(1_000).unwrap()),
        }
    }

    /// Get the number of whole milliseconds in the duration.
    ///
    /// ```rust
    /// # use embedded_time::prelude::*;
    /// assert_eq!(1.seconds().as_millis(), 1_000);
    /// assert_eq!((-1).seconds().as_millis(), -1_000);
    /// assert_eq!(1.milliseconds().as_millis(), 1);
    /// assert_eq!((-1).milliseconds().as_millis(), -1);
    /// ```
    #[inline(always)]
    pub fn as_millis(self) -> R {
        let millis = Integer(self.value) / Ratio::new(R::from(1).unwrap(), R::from(1_000).unwrap())
            * self.period;
        *millis
    }

    // /// Create a new `Duration` with the given number of microseconds.
    // ///
    // /// ```rust
    // /// # use embedded_time::{Duration, prelude::*};
    // /// assert_eq!(Duration::from_micros(1), 1_000.nanoseconds());
    // /// assert_eq!(Duration::from_micros(-1), (-1_000).nanoseconds());
    // /// ```
    // #[inline(always)]
    // #[allow(clippy::cast_possible_truncation)]
    // pub const fn from_micros(microseconds: Integer) -> Self {
    //     Self {
    //         value: microseconds,
    //     }
    // }
    //
    // /// Get the number of whole microseconds in the duration.
    // ///
    // /// ```rust
    // /// # use embedded_time::prelude::*;
    // /// assert_eq!(1.milliseconds().as_micros(), 1_000);
    // /// assert_eq!((-1).milliseconds().as_micros(), -1_000);
    // /// assert_eq!(1.microseconds().as_micros(), 1);
    // /// assert_eq!((-1).microseconds().as_micros(), -1);
    // /// ```
    // #[inline(always)]
    // pub const fn as_micros(self) -> Integer {
    //     self.value
    // }
    //
    // /// Create a new `Duration` with the given number of nanoseconds.
    // ///
    // /// ```rust
    // /// # use embedded_time::{Duration, prelude::*};
    // /// assert_eq!(Duration::from_nanos(1), 1.microseconds() / 1_000);
    // /// assert_eq!(Duration::from_nanos(-1), (-1).microseconds() / 1_000);
    // /// ```
    // #[inline(always)]
    // #[allow(clippy::cast_possible_truncation)]
    // pub const fn from_nanos(nanoseconds: Integer) -> Self {
    //     Self { value: nanoseconds }
    // }
    //
    // /// Get the number of nanoseconds in the duration.
    // ///
    // /// ```rust
    // /// # use embedded_time::prelude::*;
    // /// assert_eq!(1.microseconds().as_nanos(), 1_000);
    // /// assert_eq!((-1).microseconds().as_nanos(), -1_000);
    // /// assert_eq!(1.nanoseconds().as_nanos(), 1);
    // /// assert_eq!((-1).nanoseconds().as_nanos(), -1);
    // /// ```
    // #[inline(always)]
    // pub const fn as_nanos(self) -> Integer {
    //     self.value
    // }

    /// Computes `self + rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*, Ratio};
    /// assert_eq!(5.seconds().checked_add(5.seconds()), Some(10.seconds()));
    /// assert_eq!(Duration::max_value(Ratio::new(1,1_000)).checked_add(1.milliseconds()), None);
    /// assert_eq!((-5).seconds().checked_add(5.seconds()), Some(0.seconds()));
    /// ```
    #[inline]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        let value = self.value.checked_add(&rhs.value)?;

        Some(Self {
            value,
            period: self.period,
        })
    }

    // /// Computes `self - rhs`, returning `None` if an overflow occurred.
    // ///
    // /// ```rust
    // /// # use embedded_time::{Duration, prelude::*};
    // /// assert_eq!(5.seconds().checked_sub(5.seconds()), Some(Duration::zero()));
    // /// assert_eq!(Duration::min_value().checked_sub(1.nanoseconds()), None);
    // /// assert_eq!(5.seconds().checked_sub(10.seconds()), Some((-5).seconds()));
    // /// ```
    // #[inline(always)]
    // pub fn checked_sub(self, rhs: Self) -> Option<Self> {
    //     self.checked_add(-rhs)
    // }
    //
    // /// Computes `self * rhs`, returning `None` if an overflow occurred.
    // ///
    // /// ```rust
    // /// # use embedded_time::{Duration, prelude::*};
    // /// assert_eq!(5.seconds().checked_mul(2), Some(10.seconds()));
    // /// assert_eq!(5.seconds().checked_mul(-2), Some((-10).seconds()));
    // /// assert_eq!(5.seconds().checked_mul(0), Some(0.seconds()));
    // /// assert_eq!(Duration::max_value().checked_mul(2), None);
    // /// assert_eq!(Duration::min_value().checked_mul(2), None);
    // /// ```
    // #[inline(always)]
    // pub fn checked_mul(self, rhs: Integer) -> Option<Self> {
    //     // Multiply nanoseconds as i64, because it cannot overflow that way.
    //     let value = self.value.checked_mul(rhs)?;
    //
    //     Some(Self { value })
    // }
    //
    // /// Computes `self / rhs`, returning `None` if `rhs == 0`.
    // ///
    // /// ```rust
    // /// # use embedded_time::prelude::*;
    // /// assert_eq!(10.seconds().checked_div(2), Some(5.seconds()));
    // /// assert_eq!(10.seconds().checked_div(-2), Some((-5).seconds()));
    // /// assert_eq!(1.seconds().checked_div(0), None);
    // /// ```
    // #[inline(always)]
    // pub fn checked_div(self, rhs: Integer) -> Option<Self> {
    //     if rhs == 0 {
    //         return None;
    //     }
    //     let value = self.value / rhs;
    //
    //     Some(Self { value })
    // }
}

impl<R: IntTrait> PartialEq for Duration<R> {
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(1_000.milliseconds(), 1.seconds());
    /// assert_eq!((-1_000).milliseconds(), (-1).seconds());
    /// ```
    fn eq(&self, other: &Self) -> bool {
        (self.value * self.period.numerator * other.period.denominator)
            == (other.value * other.period.numerator * self.period.denominator)
    }
}

impl<R: IntTrait + NumericalDuration> fmt::Display for Duration<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let milliseconds = (*self).as_millis().milliseconds();
        write!(f, "{}", milliseconds.as_millis())
    }
}

impl<R: IntTrait + NumericalDuration> ops::Add for Duration<R> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).unwrap()
    }
}

impl<R: IntTrait + NumericalDuration> ops::AddAssign for Duration<R> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<R: IntTrait> ops::Neg for Duration<R> {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        self * R::from(-1).unwrap()
    }
}

impl<R: IntTrait> ops::Sub for Duration<R> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let value = self.value.checked_sub(&rhs.value).unwrap();
        Self {
            value,
            period: self.period,
        }
    }
}

impl<R: IntTrait> ops::SubAssign for Duration<R> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<R: IntTrait> ops::Mul<R> for Duration<R> {
    type Output = Self;

    #[inline(always)]
    #[allow(trivial_numeric_casts)]
    fn mul(self, rhs: R) -> Self::Output {
        let value = self.value * rhs;

        Self {
            value,
            period: self.period,
        }
    }
}

// impl<R: IntTrait> ops::Mul<Duration<R>> for Integer<R> {
//     type Output = Self;
//
//     #[inline(always)]
//     #[allow(trivial_numeric_casts)]
//     fn mul(self, rhs: Duration<R>) -> Self::Output {
//         self * rhs.value
//     }
// }

impl<R: IntTrait> ops::MulAssign<R> for Duration<R> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: R) {
        *self = *self * rhs;
    }
}

impl<R: IntTrait> ops::Div<R> for Duration<R> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: R) -> Self::Output {
        let value = self.value / rhs;

        Self {
            value,
            period: self.period,
        }
    }
}

impl<R: IntTrait> ops::DivAssign<R> for Duration<R> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: R) {
        *self = *self / rhs;
    }
}
