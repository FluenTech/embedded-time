use crate::integer::{IntTrait, Integer};
use crate::numerical_duration::TimeRep;
use crate::Period;
use core::num::TryFromIntError;
use core::{
    convert::{TryFrom, TryInto},
    fmt, ops,
    prelude::v1::*,
};

pub trait Map {
    fn map_into<T>(self) -> Self;
}

pub trait Time {}

pub trait Duration<T>: Sized + Copy + fmt::Display + Time
where
    T: TimeRep,
{
    const PERIOD: Period;

    /// Not generally useful or needed as the duration can be instantiated like this:
    /// ```no_run
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// Seconds(123);
    /// 123.seconds();
    /// ```
    /// It only exists to allow Duration methods with default definitions to create a
    /// new duration
    fn new(value: T) -> Self;

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds(123).count(), 123);
    /// ```
    fn count(self) -> T;

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// use embedded_time::Period;
    /// assert_eq!(Microseconds::from_ticks(5, Period::new_raw(1, 1_000)), Microseconds(5_000));
    /// ```
    fn from_ticks(ticks: T, period: Period) -> Self {
        Self::new(*(Integer(ticks) * (period / Self::PERIOD)))
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds::<i32>::try_from(Seconds(23_i64)), Ok(Seconds(23_i32)));
    /// assert_eq!(Milliseconds::<i32>::try_from(Seconds(23_i64)), Ok(Milliseconds(23_i32)));
    /// ```
    fn try_from<T1, R1>(other: T1) -> Result<Self, TryFromIntError>
    where
        R1: TimeRep,
        T1: Duration<R1>,
        T: TryFrom<R1>,
        TryFromIntError: From<<T as TryFrom<R1>>::Error>,
    {
        Ok(Self::new(T::try_from(other.count())?))
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds(23_i64).try_into::<_,_>(), Ok(Milliseconds(23_000_i32)));
    /// ```
    fn try_into<U, R>(self) -> Result<U, TryFromIntError>
    where
        R: TimeRep,
        R: TryFrom<T>,
        U: Duration<R>,
        TryFromIntError: From<<T as TryInto<R>>::Error>,
    {
        Ok(U::new(
            *(Integer(R::try_from(self.count())?) * (Self::PERIOD / U::PERIOD)),
        ))
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds::<i32>::min_value(), i32::MIN);
    /// ```
    fn min_value() -> T {
        T::min_value()
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds::<i32>::max_value(), i32::MAX);
    /// ```
    fn max_value() -> T {
        T::max_value()
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Milliseconds::from_dur(Seconds(1_000)), Milliseconds(1_000_000));
    /// assert_eq!(Seconds::from_dur(Milliseconds(1_234)), Seconds(1));
    /// assert_eq!(Microseconds::from_dur(Milliseconds(1_234)), Microseconds(1_234_000));
    /// assert_eq!(Microseconds::from_dur(Milliseconds(1_234_i64)), Microseconds(1_234_000_i64));
    /// assert_eq!(Microseconds::from_dur(Nanoseconds(3_234_i64)), Microseconds(3_i64));
    /// ```
    fn from_dur<U: Duration<T>>(other: U) -> Self {
        Self::new(*(Integer(other.count()) * (U::PERIOD / Self::PERIOD)))
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// let millis: Milliseconds<_> = Seconds(1_000).into_dur();
    /// assert_eq!(millis, Milliseconds(1_000_000));
    /// let seconds: Seconds<_> = Milliseconds(2_345).into_dur();
    /// assert_eq!(seconds, Seconds(2));
    /// ```
    fn into_dur<U: Duration<T>>(self) -> U {
        U::new(*(Integer(self.count()) * (Self::PERIOD / U::PERIOD)))
    }
}

pub mod time_units {
    use super::Period;
    use crate::duration::{Duration, Time};
    use crate::integer::Integer;
    use crate::numerical_duration::TimeRep;
    use core::fmt::{self, Formatter};
    use core::{
        cmp,
        convert::{TryFrom, TryInto},
        ops,
    };
    // use crate::Period;

    macro_rules! durations {
        ( $( $name:ident, ($numer:expr, $denom:expr) );+ ) => {
            $(
                #[derive(Copy, Clone, Eq, Debug, Ord)]
                pub struct $name<T: TimeRep>(pub T);

                impl<T: TimeRep> Time for $name<T>{}

                impl<T: TimeRep> Duration<T> for $name<T> {
                    const PERIOD: Period = Period::new_raw($numer, $denom);

                    fn new(value: T) -> Self {
                        Self(value)
                    }

                    fn count(self) -> T {
                        self.0
                    }
                }

                /// ```rust
                /// # use embedded_time::prelude::*;
                /// # use embedded_time::time_units::*;
                /// assert_eq!(format!("{}", Seconds(123)), "123");
                /// ```
                impl<T: TimeRep> fmt::Display for $name<T> {
                    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                        self.0.fmt(f)
                        // write!(f, "{}", self.count())
                    }
                }

                /// ```rust
                /// # use embedded_time::prelude::*;
                /// # use embedded_time::time_units::*;
                /// assert_eq!((Seconds(3_i32) + Seconds(2_i32)).count(), 5_i32);
                /// assert_eq!((Milliseconds(234) + Seconds(2)), Milliseconds(2_234));
                /// ```
                impl<T: TimeRep, U: Duration<T>> ops::Add<U> for $name<T> {
                    type Output = Self;

                    #[inline]
                    fn add(self, rhs: U) -> Self::Output {
                        Self(self.0 + Self::from_dur(rhs).0)
                    }
                }

                /// ```rust
                /// # use embedded_time::prelude::*;
                /// # use embedded_time::time_units::*;
                /// assert_eq!((Seconds(3_i32) - Seconds(2_i32)).count(), 1_i32);
                /// assert_eq!((Milliseconds(3_234) - Seconds(2)), Milliseconds(1_234));
                /// ```
                impl<T: TimeRep, U: Duration<T>> ops::Sub<U> for $name<T>
                {
                    type Output = Self;

                    #[inline]
                    fn sub(self, rhs: U) -> Self::Output {
                        Self(self.0 - Self::from_dur(rhs).0)
                    }
                }

                /// ```
                /// # use embedded_time::prelude::*;
                /// # use embedded_time::time_units::*;
                /// assert_eq!(Seconds(123), Seconds(123));
                /// assert_eq!(Seconds(123), Milliseconds(123_000));
                /// assert_ne!(Seconds(123), Milliseconds(123_001));
                /// assert_ne!(Milliseconds(123_001), Seconds(123));
                /// ```
                impl<T: TimeRep, U: Duration<T>> cmp::PartialEq<U> for $name<T> {
                    fn eq(&self, other: &U) -> bool {
                        if Self::PERIOD < U::PERIOD {
                            self.count() == Self::from_dur(*other).count()
                        } else {
                            U::from_dur(*self).count() == other.count()
                        }
                    }
                }

                /// ```
                /// # use embedded_time::prelude::*;
                /// # use embedded_time::time_units::*;
                /// assert!(Seconds(2) < Seconds(3));
                /// assert!(Seconds(2) < Milliseconds(2_001));
                /// assert!(Seconds(2) == Milliseconds(2_000));
                /// assert!(Seconds(2) > Milliseconds(1_999));
                /// ```
                impl<T: TimeRep, U: Duration<T>> PartialOrd<U> for $name<T> {
                    fn partial_cmp(&self, other: &U) -> Option<core::cmp::Ordering> {
                        if Self::PERIOD < U::PERIOD {
                            Some(self.count().cmp(&Self::from_dur(*other).count()))
                        } else {
                            Some(U::from_dur(*self).count().cmp(&other.count()))
                        }
                    }
                }

             )+
         };
    }

    durations![
        Hours,     (3600, 1);
        Minutes,     (60, 1);
        // Seconds,      (1, 1);
        Milliseconds, (1, 1_000);
        Microseconds, (1, 1_000_000);
        Nanoseconds,  (1, 1_000_000_000)
    ];

    #[derive(Copy, Clone, Eq, Debug, Ord)]
    pub struct Seconds<T>(pub T)
    where
        T: TimeRep;

    impl<T> Time for Seconds<T> where T: TimeRep {}

    impl<T> Duration<T> for Seconds<T>
    where
        T: TimeRep,
    {
        const PERIOD: Period = Period::new_raw(1, 1);

        fn new(value: T) -> Self {
            Self(value)
        }

        fn count(self) -> T {
            self.0
        }
    }

    impl<T> Map for Seconds<T> {
        fn map_into<T2>(self) -> Seconds<T2>
        where
            T2: TimeRep,
        {
        }
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(format!("{}", Seconds(123)), "123");
    /// ```
    impl<T> fmt::Display for Seconds<T>
    where
        T: TimeRep,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!((Seconds(3_i32) + Seconds(2_i32)).count(), 5_i32);
    /// assert_eq!((Milliseconds(234) + Seconds(2)), Milliseconds(2_234));
    /// ```
    impl<T, U> ops::Add<U> for Seconds<T>
    where
        T: TimeRep,
        U: Duration<T>,
    {
        type Output = Self;

        #[inline]
        fn add(self, rhs: U) -> Self::Output {
            Self(self.0 + Self::from_dur(rhs).0)
        }
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!((Seconds(3_i32) - Seconds(2_i32)).count(), 1_i32);
    /// assert_eq!((Milliseconds(3_234) - Seconds(2)), Milliseconds(1_234));
    /// ```
    impl<T, U> ops::Sub<U> for Seconds<T>
    where
        T: TimeRep,
        U: Duration<T>,
    {
        type Output = Self;

        #[inline]
        fn sub(self, rhs: U) -> Self::Output {
            Self(self.0 - Self::from_dur(rhs).0)
        }
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds(123), Seconds(123));
    /// assert_eq!(Seconds(123), Milliseconds(123_000));
    /// assert_ne!(Seconds(123), Milliseconds(123_001));
    /// assert_ne!(Milliseconds(123_001), Seconds(123));
    /// //assert_ne!(Milliseconds(123_001_i64), Seconds(123_i32)); <- not allowed
    /// ```
    impl<T, U> cmp::PartialEq<U> for Seconds<T>
    where
        T: TimeRep,
        U: Duration<T>,
    {
        fn eq(&self, other: &U) -> bool {
            if Self::PERIOD < U::PERIOD {
                self.count() == Self::from_dur(*other).count()
            } else {
                U::from_dur(*self).count() == other.count()
            }
        }
    }

    /// ```
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert!(Seconds(2) < Seconds(3));
    /// assert!(Seconds(2) < Milliseconds(2_001));
    /// assert!(Seconds(2) == Milliseconds(2_000));
    /// assert!(Seconds(2) > Milliseconds(1_999));
    /// assert!(Seconds(2_i32).try_into::<Milliseconds<i64>,_>().unwrap() > Milliseconds(1_999_i64));
    /// ```
    impl<T, U> PartialOrd<U> for Seconds<T>
    where
        T: TimeRep,
        U: Duration<T>,
    {
        fn partial_cmp(&self, other: &U) -> Option<core::cmp::Ordering> {
            if Self::PERIOD < U::PERIOD {
                Some(self.count().cmp(&Self::from_dur(*other).count()))
            } else {
                Some(U::from_dur(*self).count().cmp(&other.count()))
            }
        }
    }
}

impl<T> ops::Mul<Period> for Integer<T>
where
    T: IntTrait,
{
    type Output = Self;

    fn mul(self, rhs: Period) -> Self::Output {
        Self(self.0 * (*rhs.numer()).into() / (*rhs.denom()).into())
    }
}

impl<T> ops::Div<Period> for Integer<T>
where
    T: IntTrait,
{
    type Output = Self;

    fn div(self, rhs: Period) -> Self::Output {
        Self(self.0 * (*rhs.denom()).into() / (*rhs.numer()).into())
    }
}
