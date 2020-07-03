//! An instant of time

use crate::duration::Duration;
use core::{cmp::Ordering, convert::TryFrom, ops};
use num::traits::{WrappingAdd, WrappingSub};

/// Represents an instant of time relative to a specific [`Clock`](crate::clock::Clock)
///
/// # Example
/// Typically an `Instant` will be obtained from a [`Clock`](crate::clock::Clock)
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
    /// Construct a new Instant with ticks of the provided [`Clock`](crate::clock::Clock).
    pub fn new(ticks: Clock::Rep) -> Self {
        Self { ticks }
    }

    /// Returns the [`Duration`] since the given `Instant`
    ///
    /// # Errors
    /// - `()`: RHS is a future `Instant`
    /// - `()`: problem coverting to desired [`Duration`]
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    /// assert_eq!(Instant::<Clock>::new(5).duration_since::<Microseconds<u64>>(&Instant::<Clock>::new(3)),
    ///     Ok(Microseconds(2_000_u64)));
    ///
    /// assert_eq!(Instant::<Clock>::new(3).duration_since::<Microseconds<u64>>(&Instant::<Clock>::new(5)),
    ///     Err(()));
    /// ```
    pub fn duration_since<Dur: Duration>(&self, other: &Self) -> Result<Dur, ()>
    where
        Dur::Rep: TryFrom<Clock::Rep>,
    {
        if self < other {
            Err(())
        } else {
            Dur::from_ticks(self.ticks.wrapping_sub(&other.ticks), Clock::PERIOD).ok_or(())
        }
    }

    /// Returns the [`Duration`] until the given `Instant`
    ///
    /// # Errors
    /// - `()`: RHS is a past `Instant`
    /// - `()`: problem coverting to desired [`Duration`]
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    ///     Err(()));
    /// ```
    pub fn duration_until<Dur: Duration>(&self, other: &Self) -> Result<Dur, ()>
    where
        Dur::Rep: TryFrom<Clock::Rep>,
    {
        if self > other {
            Err(())
        } else {
            Dur::from_ticks(other.ticks.wrapping_sub(&self.ticks), Clock::PERIOD).ok_or(())
        }
    }

    /// Returns the [`Duration`] (in the provided units) since the beginning of
    /// time (or the [`Clock`](trait.Clock.html)'s 0)
    pub fn duration_since_epoch<Dur: Duration>(&self) -> Result<Dur, ()>
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
    /// # #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
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

impl<Clock: crate::Clock, Dur: Duration> ops::Add<Dur> for Instant<Clock>
where
    Clock::Rep: TryFrom<Dur::Rep>,
{
    type Output = Self;

    /// Add a duration to an instant resulting in a new, later instance
    ///
    /// # Panics
    /// If [`Duration::into_ticks()`] returns [`None`]. In this case, `u32::MAX` of seconds
    /// cannot be converted to the clock precision of milliseconds with u32 storage.
    /// ```rust,should_panic
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period =<Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// Instant::<Clock>::new(1) + Seconds(u32::MAX);
    /// ```
    ///
    /// If the the [`Duration`] is greater than the signed Clock::Rep max value which would cause
    /// the result to (logically) overflow
    /// ```rust,should_panic
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period =<Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// let _ = Instant::<Clock>::new(0) + Milliseconds(i32::MAX as u32 + 1);
    /// ```
    ///
    /// See also [`impl Sub for Duration`](duration/units/index.html#addsub)
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period =<Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// assert_eq!(Instant::<Clock>::new(1) + Seconds(3_u32), Instant::<Clock>::new(3_001));
    /// assert_eq!(Instant::<Clock>::new(1) + Milliseconds(700_u32), Instant::<Clock>::new(701));
    /// assert_eq!(Instant::<Clock>::new(1) + Milliseconds(700_u64), Instant::<Clock>::new(701));
    ///
    /// // maximum duration allowed
    /// assert_eq!(Instant::<Clock>::new(0) + Milliseconds(i32::MAX as u32),
    /// Instant::<Clock>::new(u32::MAX/2));
    /// ```
    fn add(self, rhs: Dur) -> Self::Output {
        let add_ticks: Clock::Rep = rhs.into_ticks(Clock::PERIOD).unwrap();
        debug_assert!(add_ticks < <Clock::Rep as num::Bounded>::max_value() / 2.into() + 1.into());

        Self {
            ticks: self.ticks.wrapping_add(&add_ticks),
        }
    }
}

impl<Clock: crate::Clock, Dur: Duration> ops::Sub<Dur> for Instant<Clock>
where
    Clock::Rep: TryFrom<Dur::Rep>,
{
    type Output = Self;

    /// Subtract a duration from an instant resulting in a new, earlier instance
    ///
    /// # Panics
    /// If [`Duration::into_ticks()`] returns [`None`]. In this case, `u32::MAX` of seconds
    /// cannot be converted to the clock precision of milliseconds with u32 storage.
    /// ```rust,should_panic
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period =<Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// Instant::<Clock>::new(1) - Seconds(u32::MAX);
    /// ```
    ///
    /// If the the [`Duration`] is greater than the signed Clock::Rep max value which would cause
    /// the result to (logically) underflow
    /// ```rust,should_panic
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = u32;
    ///     const PERIOD: Period = <Period>::new(1, 1_000);
    ///     // ...
    /// # type ImplError = ();
    /// # fn now(&self) -> Result<Instant<Self>, embedded_time::clock::Error<Self::ImplError>> {unimplemented!()}
    /// }
    ///
    /// let _ = Instant::<Clock>::new(u32::MAX) - Milliseconds(i32::MAX as u32 + 1);
    /// ```
    ///
    /// See also [`impl Sub for Duration`](duration/units/index.html#addsub)
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    fn sub(self, rhs: Dur) -> Self::Output {
        let sub_ticks: Clock::Rep = rhs.into_ticks(Clock::PERIOD).unwrap();
        debug_assert!(sub_ticks < <Clock::Rep as num::Bounded>::max_value() / 2.into() + 1.into());

        Self {
            ticks: self.ticks.wrapping_sub(&sub_ticks),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{self as time, units::*, Instant, Period};

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
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
        assert_eq!(diff, Err(()));
    }
}
