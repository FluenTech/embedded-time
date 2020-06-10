use crate::{duration::time_units::*, Period};
use core::{convert::TryFrom, convert::TryInto, fmt};

/// Create `Duration`s from primitive and core numeric types.
///
/// This trait can be imported with `use embedded-time::prelude::*`.
///
/// # Examples
/// Basic construction of `Duration`s.
/// ```rust
/// # use embedded_time::prelude::*;
/// # use embedded_time::time_units::*;
/// assert_eq!(5.nanoseconds(), Nanoseconds(5));
/// assert_eq!(5.microseconds(), Microseconds(5));
/// assert_eq!(5.milliseconds(), Milliseconds(5));
/// assert_eq!(5.seconds(), Seconds(5));
/// assert_eq!(5.minutes(), Minutes(5));
/// assert_eq!(5.hours(), Hours(5));
/// ```
///
/// Signed integers work as well!
/// ```rust
/// # use embedded_time::prelude::*;
/// # use embedded_time::time_units::*;
/// assert_eq!((-5).nanoseconds(), Nanoseconds(-5));
/// assert_eq!((-5).microseconds(), Microseconds(-5));
/// assert_eq!((-5).milliseconds(), Milliseconds(-5));
/// assert_eq!((-5).seconds(), Seconds(-5));
/// assert_eq!((-5).minutes(), Minutes(-5));
/// assert_eq!((-5).hours(), Hours(-5));
/// ```
pub trait TimeRep:
    Sized
    + Copy
    + num::Num
    + num::Bounded
    + PartialOrd
    + Ord
    + Eq
    + num::traits::WrappingAdd
    + num::traits::WrappingSub
    + num::CheckedMul
    + num::CheckedDiv
    + From<i32>
    + TryInto<i32>
    + TryFrom<i64>
    + Into<i64>
    + fmt::Display
    + fmt::Debug
{
    fn nanoseconds(self) -> Nanoseconds<Self>;
    fn microseconds(self) -> Microseconds<Self>;
    fn milliseconds(self) -> Milliseconds<Self>;
    fn seconds(self) -> Seconds<Self>;
    fn minutes(self) -> Minutes<Self>;
    fn hours(self) -> Hours<Self>;

    fn checked_mul(&self, ratio: &Period) -> Option<Self> {
        Some(<Self as num::CheckedDiv>::checked_div(
            &<Self as num::CheckedMul>::checked_mul(&self, &(*ratio.numer()).into())?,
            &(*ratio.denom()).into(),
        )?)
    }

    fn checked_div(&self, ratio: &Period) -> Option<Self> {
        Some(<Self as num::CheckedDiv>::checked_div(
            &<Self as num::CheckedMul>::checked_mul(&self, &(*ratio.denom()).into())?,
            &(*ratio.numer()).into(),
        )?)
    }
}

macro_rules! impl_numerical_duration {
    ($($type:ty),* $(,)?) => {
        $(
            /// Create a duration from a primitive integer type
            impl TimeRep for $type {
                #[inline(always)]
                fn nanoseconds(self) -> Nanoseconds<$type> {
                    Nanoseconds(self)
                }

                #[inline(always)]
                fn microseconds(self) -> Microseconds<$type> {
                    Microseconds(self)
                }

                #[inline(always)]
                fn milliseconds(self) -> Milliseconds<$type> {
                    Milliseconds(self)
                }

                #[inline(always)]
                fn seconds(self) -> Seconds<$type> {
                    Seconds(self)
                }

                #[inline(always)]
                fn minutes(self) -> Minutes<$type> {
                    Minutes(self)
                }

                #[inline(always)]
                fn hours(self) -> Hours<$type> {
                    Hours(self)
                }
            }
        )*
    };
}

impl_numerical_duration![i32, i64];
