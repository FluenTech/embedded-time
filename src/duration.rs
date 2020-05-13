use crate::integer::{IntTrait, Integer};
use crate::numerical_duration::TimeRep;
use crate::{Period, Wrapper};
use core::convert::Infallible;
use core::num::TryFromIntError;
use core::{fmt, ops, prelude::v1::*};
use num::rational::Ratio;

pub trait Time {}

#[derive(Debug, Eq, PartialEq)]
pub struct TryFromDurError;

#[derive(Debug, Eq, PartialEq)]
pub struct TryConvertFromError;

impl From<TryFromIntError> for TryConvertFromError {
    fn from(_: TryFromIntError) -> Self {
        TryConvertFromError
    }
}

impl From<Infallible> for TryConvertFromError {
    fn from(_: Infallible) -> Self {
        TryConvertFromError
    }
}

pub trait Duration<Rep>: Sized + Copy + fmt::Display + Time + Period + Wrapper
where
    Rep: TimeRep,
{
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
    /// # use num::rational::Ratio;
    /// assert_eq!(Microseconds::from_ticks(5, Ratio::<i32>::new_raw(1, 1_000)), Microseconds(5_000));
    /// ```
    fn from_ticks(ticks: Rep, period: Ratio<i32>) -> Self {
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
    use crate::duration::{Duration, Time, TryConvertFromError};
    use crate::integer::Integer;
    use crate::numerical_duration::TimeRep;
    use crate::Wrapper;
    use core::{
        cmp,
        convert::TryFrom,
        fmt::{self, Formatter},
        ops,
    };
    use num::rational::Ratio;

    macro_rules! durations {
        ( $( $name:ident, ($numer:expr, $denom:expr) );+ ) => {
            $(
                #[derive(Copy, Clone, Eq, Debug, Ord)]
                pub struct $name<T: TimeRep>(pub T);

                impl<T: TimeRep> Time for $name<T>{}

                impl<T: TimeRep> Period for $name<T> {
                    const PERIOD: Ratio<i32> = Ratio::<i32>::new_raw($numer, $denom);
                }

                impl<T: TimeRep> Wrapper for $name<T> {
                    type Rep = T;

                    fn unwrap(self) -> Self::Rep {
                        self.0
                    }
                }

                impl<T: TimeRep> Duration<T> for $name<T> {
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
                        fmt::Display::fmt(&self.0, f)
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

                impl<Rep, Dur2> cmp::PartialEq<Dur2> for $name<Rep>
                where
                    Rep: TimeRep,
                    Dur2: Wrapper + Period + TryConvertInto<Self> + TryConvertFrom<Self> + Copy,
                    Dur2::Rep: PartialEq,
                {
                    fn eq(&self, other: &Dur2) -> bool {
                        if Self::PERIOD < Dur2::PERIOD {
                            self.count() == (*other).try_convert_into().unwrap().count()
                        } else {
                            Dur2::try_convert_from(*self).unwrap().unwrap() == other.unwrap()
                        }
                    }
                }

                impl<Rep, OtherDur> PartialOrd<OtherDur> for $name<Rep>
                where
                    Rep: TimeRep,
                    OtherDur: Wrapper + Period + TryConvertInto<Self> + TryConvertFrom<Self> + Copy,
                    OtherDur::Rep: PartialEq + Ord,
                {
                    fn partial_cmp(&self, other: &OtherDur) -> Option<core::cmp::Ordering> {
                        if Self::PERIOD < OtherDur::PERIOD {
                            Some(self.count().cmp(&other.try_convert_into().unwrap().count()))
                        } else {
                            Some(
                                OtherDur::try_convert_from(*self)
                                    .unwrap()
                                    .unwrap()
                                    .cmp(&other.unwrap()),
                            )
                        }
                    }
                }

                impl<Rep, Source> TryConvertFrom<Source> for $name<Rep>
                where
                    Source: Wrapper + Period,
                    Rep: TimeRep + TryFrom<Source::Rep>,
                    TryConvertFromError: From<<Rep as TryFrom<Source::Rep>>::Error>,
                {
                    type Error = TryConvertFromError;

                    fn try_convert_from(
                        other: Source,
                    ) -> Result<Self, <Self as TryConvertFrom<Source>>::Error> {
                        let source_count = Rep::try_from(other.unwrap())?;
                        let converted_count = *(Integer(source_count) * (Source::PERIOD / Self::PERIOD));
                        Ok($name(converted_count))
                    }
                }

                impl<Rep, Dest> TryConvertInto<Dest> for $name<Rep>
                where
                    Rep: TimeRep,
                    Dest: TryConvertFrom<Self>,
                {
                    type Error = <Dest as TryConvertFrom<Self>>::Error;

                    fn try_convert_into(self) -> Result<Dest, <Dest as TryConvertFrom<Self>>::Error> {
                        Dest::try_convert_from(self)
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

    impl<Rep> Time for Seconds<Rep> where Rep: TimeRep {}

    impl<Rep> Period for Seconds<Rep>
    where
        Rep: TimeRep,
    {
        const PERIOD: Ratio<i32> = Ratio::<i32>::new_raw(1, 1);
    }

    impl<Rep> Wrapper for Seconds<Rep>
    where
        Rep: TimeRep,
    {
        type Rep = Rep;

        fn unwrap(self) -> Self::Rep {
            self.0
        }
    }

    impl<Rep> Duration<Rep> for Seconds<Rep>
    where
        Rep: TimeRep,
    {
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
            fmt::Display::fmt(&self.0, f)
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
    /// assert_ne!(Milliseconds(123_001_i64), Seconds(123_i64));
    /// assert_ne!(Seconds(123_i64), Milliseconds(123_001_i64));
    /// assert_ne!(Seconds(123_i64), Milliseconds(123_001_i32));
    /// ```
    impl<Rep, Dur2> cmp::PartialEq<Dur2> for Seconds<Rep>
    where
        Rep: TimeRep,
        Dur2: Wrapper + Period + TryConvertInto<Self> + TryConvertFrom<Self> + Copy,
        Dur2::Rep: PartialEq,
    {
        fn eq(&self, other: &Dur2) -> bool {
            if Self::PERIOD < Dur2::PERIOD {
                self.count() == (*other).try_convert_into().unwrap().count()
            } else {
                Dur2::try_convert_from(*self).unwrap().unwrap() == other.unwrap()
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
    ///
    /// assert!(Seconds(2_i32) < Milliseconds(2_001_i64));
    /// assert!(Seconds(2_i64) < Milliseconds(2_001_i32));
    /// ```
    impl<Rep, OtherDur> PartialOrd<OtherDur> for Seconds<Rep>
    where
        Rep: TimeRep,
        OtherDur: Wrapper + Period + TryConvertInto<Self> + TryConvertFrom<Self> + Copy,
        OtherDur::Rep: PartialEq + Ord,
    {
        fn partial_cmp(&self, other: &OtherDur) -> Option<core::cmp::Ordering> {
            if Self::PERIOD < OtherDur::PERIOD {
                Some(self.count().cmp(&other.try_convert_into().unwrap().count()))
            } else {
                Some(
                    OtherDur::try_convert_from(*self)
                        .unwrap()
                        .unwrap()
                        .cmp(&other.unwrap()),
                )
            }
        }
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds::<i32>::try_convert_from(Milliseconds(23_000_i64)), Ok(Seconds(23_i32)));
    /// assert_eq!(Seconds::<i64>::try_convert_from(Milliseconds(23_000_i32)), Ok(Seconds(23_i64)));
    /// ```
    impl<Rep, Source> TryConvertFrom<Source> for Seconds<Rep>
    where
        Source: Wrapper + Period,
        Rep: TimeRep + TryFrom<Source::Rep>,
        TryConvertFromError: From<<Rep as TryFrom<Source::Rep>>::Error>,
    {
        type Error = TryConvertFromError;

        fn try_convert_from(
            other: Source,
        ) -> Result<Self, <Self as TryConvertFrom<Source>>::Error> {
            let source_count = Rep::try_from(other.unwrap())?;
            let converted_count = *(Integer(source_count) * (Source::PERIOD / Self::PERIOD));
            Ok(Seconds(converted_count))
        }
    }

    pub trait TryConvertFrom<Source>: Sized {
        type Error: fmt::Debug;

        fn try_convert_from(other: Source)
            -> Result<Self, <Self as TryConvertFrom<Source>>::Error>;
    }

    pub trait TryConvertInto<Dest> {
        type Error: fmt::Debug;

        fn try_convert_into(self) -> Result<Dest, Self::Error>;
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::time_units::*;
    /// assert_eq!(Seconds(23_000_i64).try_convert_into(), Ok(Seconds(23_000_i32)));
    /// assert_eq!(Seconds(23_000_i32).try_convert_into(), Ok(Seconds(23_000_i32)));
    /// assert_eq!(Ok(Seconds(23_000_i64)), (Seconds(23_000_i32).try_convert_into()));
    ///
    /// assert_eq!(Milliseconds(23_000_i64).try_convert_into(), Ok(Seconds(23_i32)));
    /// assert_eq!(Milliseconds(23_000_i32).try_convert_into(), Ok(Seconds(23_i64)));
    /// ```
    impl<Rep, Dest> TryConvertInto<Dest> for Seconds<Rep>
    where
        Rep: TimeRep,
        Dest: TryConvertFrom<Self>,
    {
        type Error = <Dest as TryConvertFrom<Self>>::Error;

        fn try_convert_into(self) -> Result<Dest, <Dest as TryConvertFrom<Self>>::Error> {
            Dest::try_convert_from(self)
        }
    }

    // todo
    // /// ```rust
    // /// # use embedded_time::prelude::*;
    // /// # use embedded_time::time_units::*;
    // /// use std::convert::TryFrom;
    // /// //assert_eq!(i32::try_convert_from(Seconds(23_000_i64)), Ok(Seconds(23_000_i32)));
    // /// //assert_eq!(i32::try_convert_from(Seconds(23_000_i32)), Ok(Seconds(23_000_i32)));
    // /// //assert_eq!(i64::try_convert_from(Seconds(23_000_i32)), Ok(Seconds(23_000_i64)));
    // /// // todo
    // /// assert_eq!(Seconds::<i32>::try_from(Milliseconds(23_000_i64)), Ok(Seconds(23_i32)));
    // /// assert_eq!(Seconds::<i64>::try_from(Milliseconds(23_000_i32)), Ok(Seconds(23_i64)));
    // /// assert_eq!(Seconds::<i64>::try_from(Seconds(23_i32)), Ok(Seconds(23_i64)));
    // /// assert_eq!(Seconds::<i32>::try_from(Seconds(23_i64)), Ok(Seconds(23_i32)));
    // /// ```
    // impl<Rep, Rep2> TryFrom<Milliseconds<Rep2>> for Seconds<Rep>
    // where
    //     Rep2: TimeRep,
    //     Rep: TimeRep + TryFrom<Rep2>,
    //     TryConvertFromError: From<<Rep as TryFrom<Rep2>>::Error>,
    // {
    //     type Error = TryConvertFromError;
    //
    //     fn try_from(
    //         source: Milliseconds<Rep2>,
    //     ) -> Result<Self, <Self as TryFrom<Milliseconds<Rep2>>>::Error> {
    //         let source_count = Rep::try_from(source.unwrap())?;
    //         let converted_count =
    //             *(Integer(source_count) * (Milliseconds::<Rep2>::PERIOD / Self::PERIOD));
    //         Ok(Seconds(converted_count))
    //     }
    // }
    // impl TryFrom<Seconds<i32>> for Seconds<i64> {
    //     type Error = Infallible;
    //
    //     fn try_from(source: Seconds<i32>) -> Result<Self, Self::Error> {
    //         Ok(Seconds(source.unwrap().into()))
    //     }
    // }
    // impl TryFrom<Seconds<i64>> for Seconds<i32> {
    //     type Error = <i32 as TryFrom<i64>>::Error;
    //
    //     fn try_from(source: Seconds<i64>) -> Result<Self, Self::Error> {
    //         Ok(Seconds(i32::try_from(source.unwrap())?))
    //     }
    // }
}

impl<T> ops::Mul<Ratio<i32>> for Integer<T>
where
    T: IntTrait,
{
    type Output = Self;

    fn mul(self, rhs: Ratio<i32>) -> Self::Output {
        Self(self.0 * (*rhs.numer()).into() / (*rhs.denom()).into())
    }
}

impl<T> ops::Div<Ratio<i32>> for Integer<T>
where
    T: IntTrait,
{
    type Output = Self;

    fn div(self, rhs: Ratio<i32>) -> Self::Output {
        Self(self.0 * (*rhs.denom()).into() / (*rhs.numer()).into())
    }
}
