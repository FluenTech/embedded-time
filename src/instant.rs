//! An instant of time

use crate::{duration::Duration, ConversionError};
use core::{cmp::Ordering, convert::TryFrom, ops};
use num::traits::{WrappingAdd, WrappingSub};

/// Represents an instant of time relative to a specific [`Clock`](clock/trait.Clock.html)
///
/// # Example
/// Typically an `Instant` will be obtained from a [`Clock`](clock/trait.Clock.html)
/// ```rust
/// # use embedded_time::{Period, traits::*, Instant};
/// # #[derive(Debug)]
/// # struct SomeClock;
/// # impl embedded_time::Clock for SomeClock {
/// #     type Rep = u32;
/// #     const PERIOD: Period = <Period>::new(1, 1_000);
/// #     type ImplError = ();
/// #     fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {Ok(Instant::<Self>::new(23))}
/// # }
/// let some_clock = SomeClock;
/// let some_instant = some_clock.now().unwrap();
/// ```
///
/// However, an `Instant` can also be constructed directly. In this case the constructed `Instant`
/// is `23 * SomeClock::PERIOD` seconds since the clock's epoch
/// ```rust,no_run
/// # use embedded_time::{Period, Instant};
/// # #[derive(Debug)]
/// # struct SomeClock;
/// # impl embedded_time::Clock for SomeClock {
/// #     type Rep = u32;
/// #     const PERIOD: Period = <Period>::new(1, 1_000);
/// #     type ImplError = ();
/// #     fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
/// # }
/// Instant::<SomeClock>::new(23);
/// ```
#[derive(Debug)]
pub struct Instant<Clock: crate::Clock> {
    ticks: Clock::Rep,
}

impl<Clock: crate::Clock> Instant<Clock> {
    /// Construct a new Instant from the provided [`Clock`](clock/trait.Clock.html)
    pub fn new(ticks: Clock::Rep) -> Self {
        Self { ticks }
    }

    /// Returns the [`Duration`] since the given `Instant`
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant, ConversionError};
    /// # #[derive(Debug)]
    /// #
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period = <Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// assert_eq!(Ok(Microseconds(2_000_u64)),
    ///     Instant::<Clock>::new(5).duration_since::<Microseconds<u64>>(&Instant::<Clock>::new(3)));
    ///
    /// assert_eq!(Err(ConversionError::NegDuration),
    ///     Instant::<Clock>::new(3).duration_since::<Microseconds<u64>>(&Instant::<Clock>::new(5)));
    /// ```
    ///
    /// # Errors
    ///
    /// - [`ConversionError::NegDuration`] : `Instant` is in the future
    /// - [`ConversionError::Overflow`] : problem coverting to the desired [`Duration`]
    /// - [`ConversionError::ConversionFailure`] : problem coverting to the desired [`Duration`]
    pub fn duration_since<Dur: Duration>(&self, other: &Self) -> Result<Dur, ConversionError>
    where
        Dur::Rep: TryFrom<Clock::Rep>,
    {
        if self >= other {
            Dur::from_ticks(self.ticks.wrapping_sub(&other.ticks), Clock::PERIOD)
        } else {
            Err(ConversionError::NegDuration)
        }
    }

    /// Returns the [`Duration`] until the given `Instant`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant, ConversionError};
    /// # #[derive(Debug)]
    /// #
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period =<Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// assert_eq!(Instant::<Clock>::new(5).duration_until::<Microseconds<u64>>(&Instant::<Clock>::new(7)),
    ///     Ok(Microseconds(2_000_u64)));
    ///
    /// assert_eq!(Instant::<Clock>::new(7).duration_until::<Microseconds<u64>>(&Instant::<Clock>::new(5)),
    ///     Err(ConversionError::NegDuration));
    /// ```
    ///
    /// # Errors
    ///
    /// - [`ConversionError::NegDuration`] : `Instant` is in the past
    /// - [`ConversionError::Overflow`] : problem coverting to the desired [`Duration`]
    /// - [`ConversionError::ConversionFailure`] : problem coverting to the desired [`Duration`]
    pub fn duration_until<Dur: Duration>(&self, other: &Self) -> Result<Dur, ConversionError>
    where
        Dur::Rep: TryFrom<Clock::Rep>,
    {
        if self <= other {
            Dur::from_ticks(other.ticks.wrapping_sub(&self.ticks), Clock::PERIOD)
        } else {
            Err(ConversionError::NegDuration)
        }
    }

    /// Returns the [`Duration`] (in the provided units) since the beginning of time (the
    /// [`Clock`](clock/trait.Clock.html)'s 0)
    ///
    /// If it is a _wrapping_ clock, the result is meaningless.
    pub fn duration_since_epoch<Dur: Duration>(&self) -> Result<Dur, ConversionError>
    where
        Dur::Rep: TryFrom<Clock::Rep>,
        Clock::Rep: TryFrom<Dur::Rep>,
    {
        Self::duration_since::<Dur>(
            &self,
            &Self {
                ticks: Clock::Rep::from(0),
            },
        )
    }

    /// Subtract a [`Duration`] from an `Instant` resulting in a new, earlier `Instant`
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period =<Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// assert_eq!(Instant::<Clock>::new(800) - Milliseconds(700_u32), Instant::<Clock>::new(100));
    /// assert_eq!(Instant::<Clock>::new(5_000) - Milliseconds(700_u64), Instant::<Clock>::new(4_300));
    ///
    /// // maximum duration allowed
    /// assert_eq!(Instant::<Clock>::new(u32::MAX) - Milliseconds(i32::MAX as u32),
    /// Instant::<Clock>::new(u32::MAX/2 + 1));
    /// ```
    /// # Errors
    /// [`ConversionError::Overflow`] : The duration is more than half the wrap-around period of the clock
    ///
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant, ConversionError};
    /// # #[derive(Debug)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period =<Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// assert_eq!(Instant::<Clock>::new(0).checked_add_duration(Milliseconds(u32::MAX/2 + 1)),
    ///     Err(ConversionError::Overflow));
    /// ```
    pub fn checked_add_duration<Dur: Duration>(self, duration: Dur) -> Result<Self, ConversionError>
    where
        Clock::Rep: TryFrom<Dur::Rep>,
    {
        let add_ticks: Clock::Rep = duration.into_ticks(Clock::PERIOD)?;
        if add_ticks <= (<Clock::Rep as num::Bounded>::max_value() / 2.into()) {
            Ok(Self {
                ticks: self.ticks.wrapping_add(&add_ticks),
            })
        } else {
            Err(ConversionError::Overflow)
        }
    }

    /// Adds a [`Duration`] to an `Instant` resulting in a new, later `Instant`
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period =<Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// assert_eq!(Instant::<Clock>::new(800) - Milliseconds(700_u32), Instant::<Clock>::new(100));
    /// assert_eq!(Instant::<Clock>::new(5_000) - Milliseconds(700_u64), Instant::<Clock>::new(4_300));
    ///
    /// // maximum duration allowed
    /// assert_eq!(Instant::<Clock>::new(u32::MAX) - Milliseconds(i32::MAX as u32),
    /// Instant::<Clock>::new(u32::MAX/2 + 1));
    /// ```
    /// # Errors
    /// [`ConversionError::Overflow`] : The duration is more than half the wrap-around period of the clock
    ///
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant, ConversionError};
    /// # #[derive(Debug)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period =<Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// assert_eq!(Instant::<Clock>::new(u32::MAX).checked_sub_duration(Milliseconds(u32::MAX/2 + 1)),
    ///     Err(ConversionError::Overflow));
    /// ```
    pub fn checked_sub_duration<Dur: Duration>(self, duration: Dur) -> Result<Self, ConversionError>
    where
        Clock::Rep: TryFrom<Dur::Rep>,
    {
        let sub_ticks: Clock::Rep = duration.into_ticks(Clock::PERIOD)?;
        if sub_ticks <= (<Clock::Rep as num::Bounded>::max_value() / 2.into()) {
            Ok(Self {
                ticks: self.ticks.wrapping_sub(&sub_ticks),
            })
        } else {
            Err(ConversionError::Overflow)
        }
    }
}

impl<Clock: crate::Clock> Copy for Instant<Clock> {}

impl<Clock: crate::Clock> Clone for Instant<Clock> {
    fn clone(&self) -> Self {
        Self { ticks: self.ticks }
    }
}

impl<Clock: crate::Clock> PartialEq for Instant<Clock> {
    fn eq(&self, other: &Self) -> bool {
        self.ticks == other.ticks
    }
}

impl<Clock: crate::Clock> Eq for Instant<Clock> {}

impl<Clock: crate::Clock> PartialOrd for Instant<Clock> {
    /// Calculates the difference between two `Instance`s resulting in a [`Duration`]
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period =<Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// assert!(Instant::<Clock>::new(5) > Instant::<Clock>::new(3));
    /// assert!(Instant::<Clock>::new(5) == Instant::<Clock>::new(5));
    /// assert!(Instant::<Clock>::new(u32::MAX) < Instant::<Clock>::new(u32::MIN));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl<Clock: crate::Clock> Ord for Instant<Clock> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ticks
            .wrapping_sub(&other.ticks)
            .cmp(&(<Clock::Rep as num::Bounded>::max_value() / 2.into()))
            .reverse()
    }
}

/// Add a [`Duration`] to an `Instant` resulting in a later `Instant`
impl<Clock: crate::Clock, Dur: Duration> ops::Add<Dur> for Instant<Clock>
where
    Clock::Rep: TryFrom<Dur::Rep>,
{
    type Output = Self;

    /// Add a [`Duration`] to an `Instant` resulting in a new, later `Instant`
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period =<Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// assert_eq!(Instant::<Clock>::new(1) + Seconds(3_u32),
    ///     Instant::<Clock>::new(3_001));
    /// assert_eq!(Instant::<Clock>::new(1) + Milliseconds(700_u32),
    ///     Instant::<Clock>::new(701));
    /// assert_eq!(Instant::<Clock>::new(1) + Milliseconds(700_u64),
    ///     Instant::<Clock>::new(701));
    ///
    /// // maximum duration allowed
    /// assert_eq!(Instant::<Clock>::new(0) + Milliseconds(i32::MAX as u32),
    ///    Instant::<Clock>::new(u32::MAX/2));
    /// ```
    ///
    /// # Panics
    /// Virtually the same reason the integer operation would panic. Namely, if the
    /// result overflows the type. Specifically, if the duration is more than half
    /// the wrap-around period of the clock.
    ///
    /// ```rust,should_panic
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period =<Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// Instant::<Clock>::new(0) + Milliseconds(u32::MAX/2 + 1);
    /// ```
    fn add(self, rhs: Dur) -> Self::Output {
        self.checked_add_duration(rhs).unwrap()
    }
}

/// Subtract a [`Duration`] from an `Instant` resulting in an earlier `Instant`
impl<Clock: crate::Clock, Dur: Duration> ops::Sub<Dur> for Instant<Clock>
where
    Clock::Rep: TryFrom<Dur::Rep>,
{
    type Output = Self;

    /// Subtract a [`Duration`] from an `Instant` resulting in a new, earlier `Instant`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period =<Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// assert_eq!(Instant::<Clock>::new(5_001) - Seconds(3_u32),
    ///     Instant::<Clock>::new(2_001));
    /// assert_eq!(Instant::<Clock>::new(800) - Milliseconds(700_u32),
    ///     Instant::<Clock>::new(100));
    /// assert_eq!(Instant::<Clock>::new(5_000) - Milliseconds(700_u64),
    ///     Instant::<Clock>::new(4_300));
    ///
    /// // maximum duration allowed
    /// assert_eq!(Instant::<Clock>::new(u32::MAX) - Milliseconds(i32::MAX as u32),
    ///     Instant::<Clock>::new(u32::MAX/2 + 1));
    /// ```
    ///
    /// # Panics
    ///
    /// Virtually the same reason the integer operation would panic. Namely, if the
    /// result overflows the type. Specifically, if the duration is more than half
    /// the wrap-around period of the clock.
    ///
    /// ```rust,should_panic
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period =<Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// Instant::<Clock>::new(u32::MAX) - Milliseconds(u32::MAX/2 + 1);
    /// ```
    fn sub(self, rhs: Dur) -> Self::Output {
        self.checked_sub_duration(rhs).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{self as time, units::*, ConversionError, Instant, Period};

    #[derive(Debug)]
    struct Clock;

    impl time::Clock for Clock {
        type Rep = u32;
        const PERIOD: Period = <Period>::new(1, 1_000);
        type ImplError = ();

        fn now(&self) -> Result<Instant<Self>, time::clock::Error<Self::ImplError>> {
            unimplemented!()
        }
    }

    #[test]
    fn duration_since() {
        let diff: Result<Milliseconds<_>, _> =
            Instant::<Clock>::new(5).duration_since(&Instant::<Clock>::new(3));
        assert_eq!(diff, Ok(Milliseconds(2_u32)));

        let diff: Result<Microseconds<u64>, _> =
            Instant::<Clock>::new(5).duration_since(&Instant::<Clock>::new(3));
        assert_eq!(diff, Ok(Microseconds(2_000_u64)));

        let diff: Result<Microseconds<u64>, _> =
            Instant::<Clock>::new(u32::MIN).duration_since(&Instant::<Clock>::new(u32::MAX));
        assert_eq!(diff, Ok(Microseconds(1_000_u64)));

        let diff: Result<Microseconds<u64>, _> =
            Instant::<Clock>::new(5).duration_since(&Instant::<Clock>::new(6));
        assert_eq!(diff, Err(ConversionError::NegDuration));
    }
}
