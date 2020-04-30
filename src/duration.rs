use crate::numerical_traits::NumericalDuration;
use core::fmt::{self, Display, Formatter};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Ord, PartialOrd)]
pub struct Duration {
    nanoseconds: i64,
}

/// The number of seconds in one minute.
const SECONDS_PER_MINUTE: i64 = 60;

/// The number of seconds in one hour.
const SECONDS_PER_HOUR: i64 = 60 * SECONDS_PER_MINUTE;

impl Duration {
    /// Equivalent to `0.seconds()`.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(Duration::zero(), 0.seconds());
    /// ```
    #[inline(always)]
    pub const fn zero() -> Self {
        Self::from_secs(0)
    }

    /// The maximum possible duration. Adding any positive duration to this will
    /// cause an overflow.
    ///
    /// The value returned by this method may change at any time.
    #[inline(always)]
    pub const fn max_value() -> Self {
        Self {
            nanoseconds: i64::max_value(),
        }
    }

    /// The minimum possible duration. Adding any negative duration to this will
    /// cause an overflow.
    ///
    /// The value returned by this method may change at any time.
    #[inline(always)]
    pub const fn min_value() -> Self {
        Self {
            nanoseconds: i64::min_value(),
        }
    }

    /// Get the absolute value of the duration.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(1.seconds().abs(), 1.seconds());
    /// assert_eq!(0.seconds().abs(), 0.seconds());
    /// assert_eq!((-1).seconds().abs(), 1.seconds());
    /// ```
    #[inline(always)]
    // #[rustversion::attr(since(1.39), const)]
    pub fn abs(self) -> Self {
        Self {
            nanoseconds: self.nanoseconds.abs(),
        }
    }

    /// Create a new `Duration` with the given number of hours. Equivalent to
    /// `Duration::seconds(hours * 3_600)`.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(Duration::from_hours(1), 3_600.seconds());
    /// ```
    #[inline(always)]
    pub const fn from_hours(hours: i64) -> Self {
        Self::from_secs(hours * SECONDS_PER_HOUR)
    }

    /// Get the number of whole hours in the duration.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(1.hours().as_hours(), 1);
    /// assert_eq!((-1).hours().as_hours(), -1);
    /// assert_eq!(59.minutes().as_hours(), 0);
    /// assert_eq!((-59).minutes().as_hours(), 0);
    /// ```
    #[inline(always)]
    pub const fn as_hours(self) -> i64 {
        self.as_secs() / SECONDS_PER_HOUR
    }

    /// Create a new `Duration` with the given number of minutes. Equivalent to
    /// `Duration::seconds(minutes * 60)`.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(Duration::from_mins(1), 60.seconds());
    /// ```
    #[inline(always)]
    pub const fn from_mins(minutes: i64) -> Self {
        Self::from_secs(minutes * SECONDS_PER_MINUTE)
    }

    /// Get the number of whole minutes in the duration.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(1.minutes().as_mins(), 1);
    /// assert_eq!((-1).minutes().as_mins(), -1);
    /// assert_eq!(59.seconds().as_mins(), 0);
    /// assert_eq!((-59).seconds().as_mins(), 0);
    /// ```
    #[inline(always)]
    pub const fn as_mins(self) -> i64 {
        self.as_secs() / SECONDS_PER_MINUTE
    }

    /// Create a new `Duration` with the given number of seconds.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(Duration::from_secs(1), 1_000.milliseconds());
    /// ```
    #[inline(always)]
    pub const fn from_secs(seconds: i64) -> Self {
        Self {
            nanoseconds: seconds * 1_000_000_000,
        }
    }

    /// Get the number of whole seconds in the duration.
    ///
    /// ```rust
    /// # use embedded_time::prelude::*;
    /// assert_eq!(1.seconds().as_secs(), 1);
    /// assert_eq!((-1).seconds().as_secs(), -1);
    /// assert_eq!(1.minutes().as_secs(), 60);
    /// assert_eq!((-1).minutes().as_secs(), -60);
    /// ```
    #[inline(always)]
    pub const fn as_secs(self) -> i64 {
        self.nanoseconds / 1_000_000_000
    }

    /// Create a new `Duration` with the given number of milliseconds.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(Duration::from_millis(1), 1_000.microseconds());
    /// assert_eq!(Duration::from_millis(-1), (-1_000).microseconds());
    /// ```
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn from_millis(milliseconds: i64) -> Self {
        Self {
            nanoseconds: milliseconds * 1_000_000,
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
    pub const fn as_millis(self) -> i64 {
        self.nanoseconds / 1_000_000
    }

    /// Create a new `Duration` with the given number of microseconds.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(Duration::from_micros(1), 1_000.nanoseconds());
    /// assert_eq!(Duration::from_micros(-1), (-1_000).nanoseconds());
    /// ```
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn from_micros(microseconds: i64) -> Self {
        Self {
            nanoseconds: microseconds * 1_000,
        }
    }

    /// Get the number of whole microseconds in the duration.
    ///
    /// ```rust
    /// # use embedded_time::prelude::*;
    /// assert_eq!(1.milliseconds().as_micros(), 1_000);
    /// assert_eq!((-1).milliseconds().as_micros(), -1_000);
    /// assert_eq!(1.microseconds().as_micros(), 1);
    /// assert_eq!((-1).microseconds().as_micros(), -1);
    /// ```
    #[inline(always)]
    pub const fn as_micros(self) -> i64 {
        self.nanoseconds / 1_000
    }

    /// Create a new `Duration` with the given number of nanoseconds.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(Duration::from_nanos(1), 1.microseconds() / 1_000);
    /// assert_eq!(Duration::from_nanos(-1), (-1).microseconds() / 1_000);
    /// ```
    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn from_nanos(nanoseconds: i64) -> Self {
        Self { nanoseconds }
    }

    /// Get the number of nanoseconds in the duration.
    ///
    /// ```rust
    /// # use embedded_time::prelude::*;
    /// assert_eq!(1.microseconds().as_nanos(), 1_000);
    /// assert_eq!((-1).microseconds().as_nanos(), -1_000);
    /// assert_eq!(1.nanoseconds().as_nanos(), 1);
    /// assert_eq!((-1).nanoseconds().as_nanos(), -1);
    /// ```
    #[inline(always)]
    pub const fn as_nanos(self) -> i64 {
        self.nanoseconds
    }

    /// Computes `self + rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(5.seconds().checked_add(5.seconds()), Some(10.seconds()));
    /// assert_eq!(Duration::max_value().checked_add(1.nanoseconds()), None);
    /// assert_eq!((-5).seconds().checked_add(5.seconds()), Some(0.seconds()));
    /// ```
    #[inline]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        let nanoseconds = self.nanoseconds.checked_add(rhs.nanoseconds)?;

        Some(Self { nanoseconds })
    }

    /// Computes `self - rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(5.seconds().checked_sub(5.seconds()), Some(Duration::zero()));
    /// assert_eq!(Duration::min_value().checked_sub(1.nanoseconds()), None);
    /// assert_eq!(5.seconds().checked_sub(10.seconds()), Some((-5).seconds()));
    /// ```
    #[inline(always)]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.checked_add(-rhs)
    }

    /// Computes `self * rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use embedded_time::{Duration, prelude::*};
    /// assert_eq!(5.seconds().checked_mul(2), Some(10.seconds()));
    /// assert_eq!(5.seconds().checked_mul(-2), Some((-10).seconds()));
    /// assert_eq!(5.seconds().checked_mul(0), Some(0.seconds()));
    /// assert_eq!(Duration::max_value().checked_mul(2), None);
    /// assert_eq!(Duration::min_value().checked_mul(2), None);
    /// ```
    #[inline(always)]
    pub fn checked_mul(self, rhs: i64) -> Option<Self> {
        // Multiply nanoseconds as i64, because it cannot overflow that way.
        let nanoseconds = self.nanoseconds.checked_mul(rhs)?;

        Some(Self { nanoseconds })
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0`.
    ///
    /// ```rust
    /// # use embedded_time::prelude::*;
    /// assert_eq!(10.seconds().checked_div(2), Some(5.seconds()));
    /// assert_eq!(10.seconds().checked_div(-2), Some((-5).seconds()));
    /// assert_eq!(1.seconds().checked_div(0), None);
    /// ```
    #[inline(always)]
    pub fn checked_div(self, rhs: i64) -> Option<Self> {
        if rhs == 0 {
            return None;
        }
        let nanoseconds = self.nanoseconds / rhs;

        Some(Self { nanoseconds })
    }
}

impl From<Duration> for u32 {
    fn from(duration: Duration) -> Self {
        duration.nanoseconds as u32
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let hours = self.as_hours().hours();
        let minutes = self.as_mins().minutes() - hours;
        let seconds = self.as_secs().seconds() - minutes - hours;
        let milliseconds = self.as_millis().milliseconds() - seconds - minutes - hours;
        write!(
            f,
            "{:02}:{:02}:{:02}.{:03}",
            hours.as_hours(),
            minutes.as_mins(),
            seconds.as_secs(),
            milliseconds.as_millis(),
        )
    }
}

impl Add for Duration {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
            .expect("overflow when adding durations")
    }
}

impl AddAssign for Duration {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Neg for Duration {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        -1 * self
    }
}

impl Sub for Duration {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
            .expect("overflow when subtracting durations")
    }
}

impl SubAssign for Duration {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

macro_rules! duration_mul_div_int {
    ($($type:ty),+) => {
        $(
            impl Mul<$type> for Duration {
                type Output = Self;

                #[inline(always)]
                #[allow(trivial_numeric_casts)]
                fn mul(self, rhs: $type) -> Self::Output {
                let nanoseconds = self.nanoseconds * rhs as i64;
                    Self{ nanoseconds }
                }
            }

            impl MulAssign<$type> for Duration {
                #[inline(always)]
                fn mul_assign(&mut self, rhs: $type) {
                    *self = *self * rhs;
                }
            }

            impl Mul<Duration> for $type {
                type Output = Duration;

                #[inline(always)]
                fn mul(self, rhs: Duration) -> Self::Output {
                    rhs * self
                }
            }

            impl Div<$type> for Duration {
                type Output = Self;

                #[inline(always)]
                #[allow(trivial_numeric_casts)]
                fn div(self, rhs: $type) -> Self::Output {
                    let nanoseconds = self.nanoseconds / rhs as i64;

                    Self { nanoseconds }
                }
            }

            impl DivAssign<$type> for Duration {
                #[inline(always)]
                fn div_assign(&mut self, rhs: $type) {
                    *self = *self / rhs;
                }
            }
        )+
    };
}
duration_mul_div_int![i8, i16, i32, u8, u16, u32];
