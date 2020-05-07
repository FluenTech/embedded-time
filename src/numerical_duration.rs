use crate::duration::{Milliseconds, Seconds};
use crate::integer::IntTrait;

/// Create `Duration`s from primitive and core numeric types.
///
/// This trait can be imported with `use embedded-time::prelude::*`.
///
/// Due to limitations in rustc, the NonZero_ methods are currently _not_ `const fn`.
/// See [RFC 2632](https://github.com/rust-lang/rfcs/pull/2632) for details.
///
/// # Examples
///
/// Basic construction of `Duration`s.
///
/// ```rust
/// # use embedded_time::{prelude::*};
/// # use embedded_time::duration::{Seconds, Milliseconds};
/// //assert_eq!(5.nanoseconds(), Duration::from_nanos(5));
/// //assert_eq!(5.microseconds(), Duration::from_micros(5));
/// assert_eq!(5.milliseconds(), Milliseconds(5));
/// assert_eq!(5.seconds(), Seconds(5));
/// //assert_eq!(5.minutes(), Duration::from_mins(5));
/// //assert_eq!(5.hours(), Duration::from_hours(5));
/// ```
///
/// Signed integers work as well!
///
/// ```rust
/// # use embedded_time::{prelude::*};
/// # use embedded_time::duration::{Seconds, Milliseconds};
/// //assert_eq!((-5).nanoseconds(), Duration::from_nanos(-5));
/// //assert_eq!((-5).microseconds(), Duration::from_micros(-5));
/// assert_eq!((-5).milliseconds(), Milliseconds(-5));
/// assert_eq!((-5).seconds(), Seconds(-5));
/// //assert_eq!((-5).minutes(), Duration::from_mins(-5));
/// //assert_eq!((-5).hours(), Duration::from_hours(-5));
/// ```
///
pub trait NumericalDuration: IntTrait {
    // /// Create a `Duration` from the number of nanoseconds.
    // fn nanoseconds(self) -> Duration<Self>;
    // /// Create a `Duration` from the number of microseconds.
    // fn microseconds(self) -> Duration<Self>;
    /// Create a `Duration` from the number of milliseconds.
    fn milliseconds(self) -> Milliseconds<Self>;
    /// Create a `Duration` from the number of seconds.
    fn seconds(self) -> Seconds<Self>;
    // /// Create a `Duration` from the number of minutes.
    // fn minutes(self) -> Duration<Self>;
    // /// Create a `Duration` from the number of hours.
    // fn hours(self) -> Duration<Self>;
}

macro_rules! impl_numerical_duration {
    ($($type:ty),* $(,)?) => {
        $(
            /// Create a duration from a primitive integer type
            /// Duration storage ensures at least +/- 10 years range
            impl NumericalDuration for $type {
                // // at least 64 bits
                // #[inline(always)]
                // fn nanoseconds(self) -> Duration<$type> {
                //     Duration::from_nanos(self)
                // }
                //
                // // at least 50 bits
                // #[inline(always)]
                // fn microseconds(self) -> Duration<$type> {
                //     Duration::from_micros(self)
                // }

                // at least 40 bits
                #[inline(always)]
                fn milliseconds(self) -> Milliseconds<$type> {
                    Milliseconds(self)
                }

                // at least 30 bits
                #[inline(always)]
                fn seconds(self) -> Seconds<$type> {
                    Seconds(self)
                }

                // // at least 24 bits
                // #[inline(always)]
                // fn minutes(self) -> Duration<$type> {
                //     Duration::from_mins(self)
                // }
                //
                // /// at least 18 bits
                // #[inline(always)]
                // fn hours(self) -> Duration<$type> {
                //     Duration::from_hours(self)
                // }
            }
        )*
    };
}

// macro_rules! impl_numerical_duration_nonzero {
//     ($($type:ty),* $(,)?) => {
//         $(
//             impl NumericalDuration for $type {
// //                 #[inline(always)]
// //                 fn nanoseconds(self) -> Duration {
// //                     Duration::from_nanos(self.get() as i64)
// //                 }
// //
// //                 #[inline(always)]
// //                 fn microseconds(self) -> Duration {
// //                     Duration::from_micros(self.get() as i64)
// //                 }
//
//                 #[inline(always)]
//                 fn milliseconds(self) -> Duration<Self> {
//                     Duration::from_millis(self)
//                 }
//
// //                 #[inline(always)]
// //                 fn seconds(self) -> Duration {
// //                     Duration::from_secs(self.get() as i64)
// //                 }
// //
// //                 #[inline(always)]
// //                 fn minutes(self) -> Duration {
// //                     Duration::from_mins(self.get() as i64)
// //                 }
// //
// //                 #[inline(always)]
// //                 fn hours(self) -> Duration {
// //                     Duration::from_hours(self.get() as i64)
// //                 }
//             }
//         )*
//     };
// }

impl_numerical_duration![i32, i64];

// impl_numerical_duration_nonzero![
//     core::num::NonZeroU8,
//     core::num::NonZeroU16,
//     core::num::NonZeroU32,
//     core::num::NonZeroI8,
//     core::num::NonZeroI16,
//     core::num::NonZeroI32,
//     core::num::NonZeroI64,
// ];
