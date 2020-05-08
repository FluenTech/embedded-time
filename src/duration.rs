use crate::integer::{IntTrait, Integer};
use crate::numerical_duration::NumericalDuration;
use crate::Ratio;
use core::convert::TryInto;
use core::fmt::Formatter;
use core::{cmp, convert, fmt, ops};

pub trait Time {}

pub trait Duration<T: IntTrait + NumericalDuration>: Sized + Copy + fmt::Display + Time {
    const PERIOD: Period;

    /// Not generally useful or needed as the duration can be instantiated like this:
    /// ```no_run
    /// # use embedded_time::duration::Seconds;
    /// # use embedded_time::prelude::*;
    /// Seconds(123);
    /// 123.seconds();
    /// ```
    /// It only exists to allow Duration methods with default definitions to create a
    /// new duration
    fn new(value: T) -> Self;

    /// ```rust
    /// # use embedded_time::duration::{Seconds, Duration};
    /// assert_eq!(Seconds(123).count(), 123);
    /// ```
    fn count(self) -> T;

    // fn period() -> Period {
    //     Self::PERIOD
    // }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::duration::Seconds;
    /// assert_eq!(Seconds::<i32>::min_value(), i32::MIN);
    /// ```
    fn min_value() -> T {
        T::min_value()
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::duration::Seconds;
    /// assert_eq!(Seconds::<i32>::max_value(), i32::MAX);
    /// ```
    fn max_value() -> T {
        T::max_value()
    }

    /// ```rust
    /// # use embedded_time::duration::{Seconds, Milliseconds, Microseconds, Duration};
    /// assert_eq!(Milliseconds::from_dur(Seconds(1_000)), Milliseconds(1_000_000));
    /// assert_eq!(Seconds::from_dur(Milliseconds(1_234)), Seconds(1));
    /// assert_eq!(Microseconds::from_dur(Milliseconds(1_234)), Microseconds(1_234_000));
    /// ```
    fn from_dur<U: Duration<T>>(other: U) -> Self {
        Self::new(*(Integer(other.count()) * (U::PERIOD / Self::PERIOD)))
    }

    /// ```rust
    /// # use embedded_time::duration::{Seconds, Milliseconds, Microseconds, Duration};
    /// let millis: Milliseconds<_> = Seconds(1_000).into_dur();
    /// assert_eq!(millis, Milliseconds(1_000_000));
    /// let seconds: Seconds<_> = Milliseconds(2_345).into_dur();
    /// assert_eq!(seconds, Seconds(2));
    /// ```
    fn into_dur<U: Duration<T>>(self) -> U {
        U::new(*(Integer(self.count()) * (Self::PERIOD / U::PERIOD)))
    }
}

macro_rules! durations {
    ( $( $name:ident, ($numer:expr, $denom:expr) );+ ) => {
        $(
            #[derive(Copy, Clone, Eq, Debug)]
            pub struct $name<T: IntTrait + NumericalDuration>(pub T);

            impl<T: IntTrait + NumericalDuration> Time for $name<T>{}

            impl<T: IntTrait + NumericalDuration> Duration<T> for $name<T> {
                const PERIOD: Period = Period::new_raw($numer, $denom);

                fn new(value: T) -> Self {
                    Self(value)
                }

                fn count(self) -> T {
                    self.0
                }
            }

            /// ```rust
            /// # use embedded_time::duration::Seconds;
            /// assert_eq!(format!("{}", Seconds(123)), "123");
            /// ```
            impl<T: IntTrait + NumericalDuration> fmt::Display for $name<T> {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    self.0.fmt(f)
                    // write!(f, "{}", self.count())
                }
            }

            /// ```rust
            /// # use embedded_time::prelude::*;
            /// use embedded_time::duration::{Seconds, Milliseconds};
            /// assert_eq!((Seconds(3_i32) + Seconds(2_i32)).count(), 5_i32);
            /// assert_eq!((Milliseconds(234) + Seconds(2)), Milliseconds(2_234));
            /// ```
            impl<T: IntTrait + NumericalDuration, U: Duration<T>> ops::Add<U> for $name<T> {
                type Output = Self;

                #[inline]
                fn add(self, rhs: U) -> Self::Output {
                    Self(self.0 + Self::from_dur(rhs).0)
                }
            }

            /// ```rust
            /// # use embedded_time::prelude::*;
            /// use embedded_time::duration::{Seconds, Milliseconds};
            /// assert_eq!((Seconds(3_i32) - Seconds(2_i32)).count(), 1_i32);
            /// assert_eq!((Milliseconds(3_234) - Seconds(2)), Milliseconds(1_234));
            /// ```
            impl<T: IntTrait + NumericalDuration, U: Duration<T>> ops::Sub<U> for $name<T>
            {
                type Output = Self;

                #[inline]
                fn sub(self, rhs: U) -> Self::Output {
                    Self(self.0 - Self::from_dur(rhs).0)
                }
            }

            /// ```
            /// # use embedded_time::duration::{Seconds, Milliseconds};
            /// assert_eq!(Seconds(123), Seconds(123));
            /// assert_eq!(Seconds(123), Milliseconds(123_000));
            /// assert_ne!(Seconds(123), Milliseconds(123_001));
            /// assert_ne!(Milliseconds(123_001), Seconds(123));
            /// ```
            impl<T: IntTrait + NumericalDuration, U: Duration<T>> cmp::PartialEq<U> for $name<T> {
                fn eq(&self, other: &U) -> bool {
                    if Self::PERIOD < U::PERIOD {
                        self.count() == Self::from_dur(*other).count()
                    } else {
                        U::from_dur(*self).count() == other.count()
                    }
                }
            }
         )+
     };
}
durations![Seconds, (1, 1); Milliseconds, (1, 1_000); Microseconds, (1, 1_000_000)];

pub(crate) type Period = Ratio<i32>;

impl<T: IntTrait> ops::Mul<Period> for Integer<T> {
    type Output = Self;

    fn mul(self, rhs: Period) -> Self::Output {
        Self(self.0 * (*rhs.numer()).into() / (*rhs.denom()).into())
    }
}

impl<T: IntTrait> ops::Div<Period> for Integer<T> {
    type Output = Self;

    fn div(self, rhs: Period) -> Self::Output {
        Self(self.0 * (*rhs.denom()).into() / (*rhs.numer()).into())
    }
}

//     // /// Computes `self + rhs`, returning `None` if an overflow occurred.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Duration, prelude::*, Ratio};
//     // /// assert_eq!(5.seconds().checked_add(5.seconds()), Some(10.seconds()));
//     // /// assert_eq!(Duration::max_value(Ratio::new(1,1_000)).checked_add(1.milliseconds()), None);
//     // /// assert_eq!((-5).seconds().checked_add(5.seconds()), Some(0.seconds()));
//     // /// ```
//     // #[inline]
//     // pub fn checked_add(self, rhs: Self) -> Option<Self> {
//     //     let value = self.value.checked_add(&rhs.value)?;
//     //
//     //     Some(Self {
//     //         value,
//     //         period: self.period,
//     //     })
//     // }
//
//     // /// Computes `self - rhs`, returning `None` if an overflow occurred.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Duration, prelude::*};
//     // /// assert_eq!(5.seconds().checked_sub(5.seconds()), Some(Duration::zero()));
//     // /// assert_eq!(Duration::min_value().checked_sub(1.nanoseconds()), None);
//     // /// assert_eq!(5.seconds().checked_sub(10.seconds()), Some((-5).seconds()));
//     // /// ```
//     // #[inline(always)]
//     // pub fn checked_sub(self, rhs: Self) -> Option<Self> {
//     //     self.checked_add(-rhs)
//     // }
//     //
//     // /// Computes `self * rhs`, returning `None` if an overflow occurred.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Duration, prelude::*};
//     // /// assert_eq!(5.seconds().checked_mul(2), Some(10.seconds()));
//     // /// assert_eq!(5.seconds().checked_mul(-2), Some((-10).seconds()));
//     // /// assert_eq!(5.seconds().checked_mul(0), Some(0.seconds()));
//     // /// assert_eq!(Duration::max_value().checked_mul(2), None);
//     // /// assert_eq!(Duration::min_value().checked_mul(2), None);
//     // /// ```
//     // #[inline(always)]
//     // pub fn checked_mul(self, rhs: Integer) -> Option<Self> {
//     //     // Multiply nanoseconds as i64, because it cannot overflow that way.
//     //     let value = self.value.checked_mul(rhs)?;
//     //
//     //     Some(Self { value })
//     // }
//     //
//     // /// Computes `self / rhs`, returning `None` if `rhs == 0`.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::prelude::*;
//     // /// assert_eq!(10.seconds().checked_div(2), Some(5.seconds()));
//     // /// assert_eq!(10.seconds().checked_div(-2), Some((-5).seconds()));
//     // /// assert_eq!(1.seconds().checked_div(0), None);
//     // /// ```
//     // #[inline(always)]
//     // pub fn checked_div(self, rhs: Integer) -> Option<Self> {
//     //     if rhs == 0 {
//     //         return None;
//     //     }
//     //     let value = self.value / rhs;
//     //
//     //     Some(Self { value })
//     // }
// }

// impl<R: IntTrait + NumericalDuration> ops::AddAssign for Duration<R> {
//     #[inline(always)]
//     fn add_assign(&mut self, rhs: Self) {
//         *self = *self + rhs;
//     }
// }
//
// impl<R: IntTrait> ops::Neg for Duration<R> {
//     type Output = Self;
//
//     #[inline(always)]
//     fn neg(self) -> Self::Output {
//         self * R::from(-1).unwrap()
//     }
// }
//
// /// ```rust
// /// # use embedded_time::prelude::*;
// /// assert_eq!(2.seconds() - 500.milliseconds(), 1_500.milliseconds());
// /// ```
// impl<R: IntTrait> ops::Sub for Duration<R> {
//     type Output = Self;
//
//     #[inline]
//     fn sub(self, rhs: Self) -> Self::Output {
//         let fraction = (Ratio::from_integer(self.value) * self.period)
//             - (Ratio::from_integer(rhs.value) * rhs.period);
//         let value = (fraction / self.period).to_integer();
//
//         Self {
//             value,
//             period: self.period,
//         }
//     }
// }
//
// impl<R: IntTrait> ops::SubAssign for Duration<R> {
//     #[inline(always)]
//     fn sub_assign(&mut self, rhs: Self) {
//         *self = *self - rhs;
//     }
// }
//
// impl<R: IntTrait> ops::Mul<R> for Duration<R> {
//     type Output = Self;
//
//     #[inline(always)]
//     #[allow(trivial_numeric_casts)]
//     fn mul(self, rhs: R) -> Self::Output {
//         let value = self.value * rhs;
//
//         Self {
//             value,
//             period: self.period,
//         }
//     }
// }
//
// impl<R: IntTrait> ops::MulAssign<R> for Duration<R> {
//     #[inline(always)]
//     fn mul_assign(&mut self, rhs: R) {
//         *self = *self * rhs;
//     }
// }
//
// impl<R: IntTrait> ops::Div<R> for Duration<R> {
//     type Output = Self;
//
//     #[inline(always)]
//     fn div(self, rhs: R) -> Self::Output {
//         let value = self.value / rhs;
//
//         Self {
//             value,
//             period: self.period,
//         }
//     }
// }
//
// impl<R: IntTrait> ops::DivAssign<R> for Duration<R> {
//     #[inline(always)]
//     fn div_assign(&mut self, rhs: R) {
//         *self = *self / rhs;
//     }
// }
