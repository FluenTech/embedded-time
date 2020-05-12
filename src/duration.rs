use crate::integer::{IntTrait, Integer};
use crate::numerical_duration::TimeRep;
use crate::Period;
use core::convert::Infallible;
use core::{fmt, num::TryFromIntError, ops, prelude::v1::*};

pub trait Time {}

#[derive(Debug, Eq, PartialEq)]
pub struct TryFromDurError;

#[derive(Debug, Eq, PartialEq)]
pub struct TryMapFromError;

impl From<TryFromIntError> for TryMapFromError {
    fn from(_: TryFromIntError) -> Self {
        TryMapFromError
    }
}

impl From<Infallible> for TryMapFromError {
    fn from(_: Infallible) -> Self {
        TryMapFromError
    }
}

pub trait Duration<Rep>: Sized + Copy + fmt::Display + Time
where
    Rep: TimeRep,
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
    fn new(value: Rep) -> Self;

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds(123).count(), 123);
    /// ```
    fn count(self) -> Rep;

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// use embedded_time::Period;
    /// assert_eq!(Microseconds::from_ticks(5, Period::new_raw(1, 1_000)), Microseconds(5_000));
    /// ```
    fn from_ticks(ticks: Rep, period: Period) -> Self {
        Self::new(*(Integer(ticks) * (period / Self::PERIOD)))
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds::<i32>::try_from_dur(Milliseconds(23_000_i32)), Ok(Seconds(23_i32)));
    /// //assert_eq!(Seconds::<i32>::try_from_dur(Milliseconds(23_000_i64)), _); <- Won't compile
    /// ```
    fn try_from_dur<FromDur>(other: FromDur) -> Result<Self, TryFromDurError>
    where
        FromDur: Duration<Rep>,
    {
        Ok(Self::new(
            *(Integer(other.count()) * (FromDur::PERIOD / Self::PERIOD)),
        ))
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds(23).try_into_dur(), Ok(Milliseconds(23_000)));
    /// ```
    fn try_into_dur<DestDur>(self) -> Result<DestDur, TryFromDurError>
    where
        DestDur: Duration<Rep>,
    {
        Ok(DestDur::new(
            *(Integer(self.count()) * (Self::PERIOD / DestDur::PERIOD)),
        ))
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds::<i32>::min_value(), i32::MIN);
    /// ```
    fn min_value() -> Rep {
        Rep::min_value()
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds::<i32>::max_value(), i32::MAX);
    /// ```
    fn max_value() -> Rep {
        Rep::max_value()
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
    fn from_dur<FromDur>(other: FromDur) -> Self
    where
        FromDur: Duration<Rep>,
    {
        Self::new(*(Integer(other.count()) * (FromDur::PERIOD / Self::PERIOD)))
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// let millis: Milliseconds<_> = Seconds(1_000).into_dur();
    /// assert_eq!(millis, Milliseconds(1_000_000));
    /// let seconds: Seconds<_> = Milliseconds(2_345).into_dur();
    /// assert_eq!(seconds, Seconds(2));
    /// ```
    fn into_dur<DestDur: Duration<Rep>>(self) -> DestDur {
        DestDur::new(*(Integer(self.count()) * (Self::PERIOD / DestDur::PERIOD)))
    }
}

pub mod time_units {
    use super::Period;
    use crate::duration::{Duration, Time, TryFromDurError, TryMapFromError};
    use crate::numerical_duration::TimeRep;
    use core::convert::TryInto;
    use core::{
        cmp,
        convert::TryFrom,
        fmt::{self, Formatter},
        num::TryFromIntError,
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

                impl<FromRep, ToRep> TryMapFrom<$name<FromRep>, FromRep> for ToRep
                where
                    FromRep: TimeRep,
                    ToRep: TimeRep + TryFrom<FromRep>,
                    TryMapFromError: From<<Self as TryFrom<FromRep>>::Error>,
                {
                    type Error = TryMapFromError;
                    type Output = $name<Self>;

                    fn try_map_from(other: $name<FromRep>) -> Result<$name<Self>, TryMapFromError> {
                        Ok($name::new(Self::try_from(other.count())?))
                    }
                }

                impl<FromRep, ToDur, ToRep> TryMapInto<ToDur, ToRep> for $name<FromRep>
                where
                    FromRep: TimeRep,
                    ToDur: Duration<ToRep>,
                    ToRep: TimeRep + TryMapFrom<$name<FromRep>, FromRep, Output = ToDur>,
                {
                    type Error = <ToRep as TryMapFrom<$name<FromRep>, FromRep>>::Error;

                    fn try_map_into(
                        self,
                    ) -> Result<ToDur, <ToRep as TryMapFrom<$name<FromRep>, FromRep>>::Error> {
                        ToRep::try_map_from(self)
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
    pub struct Seconds<Rep>(pub Rep)
    where
        Rep: TimeRep;

    // impl<Rep> Seconds<Rep> {
    //
    // }

    impl<Rep> Time for Seconds<Rep> where Rep: TimeRep {}

    impl<Rep> Duration<Rep> for Seconds<Rep>
    where
        Rep: TimeRep,
    {
        const PERIOD: Period = Period::new_raw(1, 1);

        fn new(value: Rep) -> Self {
            Self(value)
        }

        fn count(self) -> Rep {
            self.0
        }
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(format!("{}", Seconds(123)), "123");
    /// ```
    impl<Rep> fmt::Display for Seconds<Rep>
    where
        Rep: TimeRep,
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
    impl<Rep, U> ops::Add<U> for Seconds<Rep>
    where
        Rep: TimeRep,
        U: Duration<Rep>,
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
    impl<Rep, U> ops::Sub<U> for Seconds<Rep>
    where
        Rep: TimeRep,
        U: Duration<Rep>,
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
    impl<Rep, U> cmp::PartialEq<U> for Seconds<Rep>
    where
        Rep: TimeRep,
        U: Duration<Rep>,
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
    /// ```
    impl<Rep, U> PartialOrd<U> for Seconds<Rep>
    where
        Rep: TimeRep,
        U: Duration<Rep>,
    {
        fn partial_cmp(&self, other: &U) -> Option<core::cmp::Ordering> {
            if Self::PERIOD < U::PERIOD {
                Some(self.count().cmp(&Self::from_dur(*other).count()))
            } else {
                Some(U::from_dur(*self).count().cmp(&other.count()))
            }
        }
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(i32::try_map_from(Seconds(23_000_i64)), Ok(Seconds(23_000_i32)));
    /// assert_eq!(i32::try_map_from(Seconds(23_000_i32)), Ok(Seconds(23_000_i32)));
    /// assert_eq!(i64::try_map_from(Seconds(23_000_i32)), Ok(Seconds(23_000_i64)));
    ///
    /// assert_eq!(i32::try_map_from(Milliseconds(23_000_i64)), Ok(Milliseconds(23_000_i32)));
    /// ```
    impl<FromRep, ToRep> TryMapFrom<Seconds<FromRep>, FromRep> for ToRep
    where
        FromRep: TimeRep,
        ToRep: TimeRep + TryFrom<FromRep>,
        TryMapFromError: From<<Self as TryFrom<FromRep>>::Error>,
    {
        type Error = TryMapFromError;
        type Output = Seconds<Self>;

        fn try_map_from(other: Seconds<FromRep>) -> Result<Seconds<Self>, TryMapFromError> {
            Ok(Seconds::new(Self::try_from(other.count())?))
        }
    }

    pub trait TryMapFrom<FromDur, FromRep>: TimeRep + TryFrom<FromRep>
    where
        FromRep: TimeRep,
    {
        type Error: From<<Self as TryFrom<FromRep>>::Error>;
        type Output;

        fn try_map_from(
            other: FromDur,
        ) -> Result<
            <Self as TryMapFrom<FromDur, FromRep>>::Output,
            <Self as TryMapFrom<FromDur, FromRep>>::Error,
        >;
    }

    pub trait TryMapInto<ToDur, ToRep> {
        type Error;

        fn try_map_into(self) -> Result<ToDur, Self::Error>;
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds(23_000_i64).try_map_into(), Ok(Seconds(23_000_i32)));
    /// assert_eq!(Seconds(23_000_i32).try_map_into(), Ok(Seconds(23_000_i32)));
    /// assert_eq!(Ok(Seconds(23_000_i64)), (Seconds(23_000_i32).try_map_into()));
    ///
    /// assert_eq!(Milliseconds(23_000_i64).try_map_into(), Ok(Milliseconds(23_000_i32)));
    /// ```
    impl<FromRep, ToRep> TryMapInto<Seconds<ToRep>, ToRep> for Seconds<FromRep>
    where
        FromRep: TimeRep,
        ToRep: TimeRep + TryMapFrom<Seconds<FromRep>, FromRep, Output = Seconds<ToRep>>,
    {
        type Error = <ToRep as TryMapFrom<Seconds<FromRep>, FromRep>>::Error;

        fn try_map_into(
            self,
        ) -> Result<Seconds<ToRep>, <ToRep as TryMapFrom<Seconds<FromRep>, FromRep>>::Error>
        {
            ToRep::try_map_from(self)
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
