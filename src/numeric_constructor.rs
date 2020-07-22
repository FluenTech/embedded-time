//! Construction of time-based types from integers

use crate::{duration::units::*, rate::units::*, time_int::TimeInt};

/// Create time-based values from primitive and core numeric types.
///
/// This trait is anonomously re-exported in [`traits`](crate::traits)
///
/// # Examples
/// Basic construction of time-based values.
/// ```rust
/// # use embedded_time::{traits::*, duration::units::*, rate::units::*};
/// assert_eq!(5_u32.nanoseconds(), Nanoseconds(5_u32));
/// assert_eq!(5_u32.microseconds(), Microseconds(5_u32));
/// assert_eq!(5_u32.milliseconds(), Milliseconds(5_u32));
/// assert_eq!(5_u32.seconds(), Seconds(5_u32));
/// assert_eq!(5_u32.minutes(), Minutes(5_u32));
/// assert_eq!(5_u32.hours(), Hours(5_u32));
///
/// assert_eq!(5_u32.hertz(), Hertz(5_u32));
/// ```
pub trait NumericConstructor: TimeInt {
    /// Construct the duration implementation
    fn nanoseconds(self) -> Nanoseconds<Self>;
    /// Construct the duration implementation
    fn microseconds(self) -> Microseconds<Self>;
    /// Construct the duration implementation
    fn milliseconds(self) -> Milliseconds<Self>;
    /// Construct the duration implementation
    fn seconds(self) -> Seconds<Self>;
    /// Construct the duration implementation
    fn minutes(self) -> Minutes<Self>;
    /// Construct the duration implementation
    fn hours(self) -> Hours<Self>;

    /// Construct the frequency type
    fn hertz(self) -> Hertz<Self>;
}

macro_rules! impl_numeric_constructors {
        ($($type:ty),* $(,)?) => {
            $(
                impl NumericConstructor for $type {
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

                    #[inline(always)]
                    fn hertz(self) -> Hertz<$type> {
                        Hertz(self)
                    }
                }
            )*
        };
    }

impl_numeric_constructors![u32, u64];
