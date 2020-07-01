//! An instant of time

use crate::Duration;
use core::{cmp::Ordering, convert::TryFrom, ops};
use num::traits::{WrappingAdd, WrappingSub};

/// Represents an instant of time relative to a specific [`Clock`](trait.Clock.html)
///
/// # Example
/// Create an `Instant` that is `23 * SomeClock::PERIOD` seconds since the clock's epoch:
/// ```rust,ignore
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

    /// Calculates the difference between two `Instance`s resulting in a [`Duration`]
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = i32;
    ///     const PERIOD: Period = Period::new(1, 1_000);
    ///     // ...
    /// # fn now(&mut self) -> Result<Instant<Self>, embedded_time::Error> {unimplemented!()}
    /// }
    ///
    /// let diff: Option<Milliseconds<_>> = Instant::<Clock>::new(5).duration_since(&Instant::<Clock>::new(3));
    /// assert_eq!(diff, Some(Milliseconds(2_i32)));
    ///
    /// let diff: Option<Microseconds<i64>> = Instant::<Clock>::new(5).duration_since(&Instant::<Clock>::new(3));
    /// assert_eq!(diff, Some(Microseconds(2_000_i64)));
    ///
    /// let diff: Option<Microseconds<i64>> = Instant::<Clock>::new(i32::MIN).duration_since(&Instant::<Clock>::new(i32::MAX));
    /// assert_eq!(diff, Some(Microseconds(1_000_i64)));
    ///
    /// let diff: Option<Seconds<i64>> = Instant::<Clock>::new(1_000).duration_since(&Instant::<Clock>::new(-1_000));
    /// assert_eq!(diff, Some(Seconds(2_i64)));
    /// ```
    pub fn duration_since<Dur: Duration>(&self, other: &Self) -> Option<Dur>
    where
        Dur::Rep: TryFrom<Clock::Rep>,
    {
        Dur::from_ticks(self.ticks.wrapping_sub(&other.ticks), Clock::PERIOD)
    }

    /// Returns the [`Duration`](trait.Duration.html) (in the provided units) since the beginning of
    /// time (or the [`Clock`](trait.Clock.html)'s 0)
    pub fn duration_since_epoch<Dur: Duration>(&self) -> Option<Dur>
    where
        Dur::Rep: TryFrom<Clock::Rep>,
        Clock::Rep: From<i32>,
    {
        Self::duration_since::<Dur>(
            &self,
            &Self {
                ticks: Clock::Rep::from(0_i32),
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
    ///     type Rep = i32;
    ///     const PERIOD: Period = Period::new(1, 1_000);
    ///     // ...
    /// # fn now(&mut self) -> Result<Instant<Self>, embedded_time::Error> {unimplemented!()}
    /// }
    ///
    /// assert!(Instant::<Clock>::new(5) > Instant::<Clock>::new(3));
    /// assert!(Instant::<Clock>::new(5) == Instant::<Clock>::new(5));
    /// assert!(Instant::<Clock>::new(i32::MAX) < Instant::<Clock>::new(i32::MIN));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl<Clock: crate::Clock> Ord for Instant<Clock> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ticks
            .wrapping_sub(&other.ticks)
            .cmp(&Clock::Rep::from(0))
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
    /// If [`Duration::into_ticks()`] returns [`None`]. In this case, `i32::MAX` of seconds
    /// cannot be converted to the clock precision of milliseconds with i32 storage.
    /// ```rust,should_panic
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = i32;
    ///     const PERIOD: Period = Period::new(1, 1_000);
    ///     // ...
    /// # fn now(&mut self) -> Result<Instant<Self>, embedded_time::Error> {unimplemented!()}
    /// }
    ///
    /// Instant::<Clock>::new(1) + Seconds(i32::MAX);
    /// ```
    /// See also: [`impl Add for Duration`](duration/units/index.html#addsub)
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = i32;
    ///     const PERIOD: Period = Period::new(1, 1_000);
    ///     // ...
    /// # fn now(&mut self) -> Result<Instant<Self>, embedded_time::Error> {unimplemented!()}
    /// }
    ///
    /// assert_eq!(Instant::<Clock>::new(1) + Seconds(3), Instant::<Clock>::new(3_001));
    /// assert_eq!(Instant::<Clock>::new(-1) + Milliseconds(5_123), Instant::<Clock>::new(5_122));
    /// assert_eq!(Instant::<Clock>::new(1) + Milliseconds(700), Instant::<Clock>::new(701));
    /// assert_eq!(Instant::<Clock>::new(1_i32) + Milliseconds(700_i64), Instant::<Clock>::new(701_i32));
    /// ```
    fn add(self, rhs: Dur) -> Self::Output {
        let add_ticks: Clock::Rep = rhs.into_ticks(Clock::PERIOD).unwrap();

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
    /// If [`Duration::into_ticks()`] returns [`None`]. In this case, `i32::MAX` of seconds
    /// cannot be converted to the clock precision of milliseconds with i32 storage.
    /// ```rust,should_panic
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = i32;
    ///     const PERIOD: Period = Period::new(1, 1_000);
    ///     // ...
    /// # fn now(&mut self) -> Result<Instant<Self>, embedded_time::Error> {unimplemented!()}
    /// }
    ///
    /// Instant::<Clock>::new(1) - Seconds(i32::MAX);
    /// ```
    /// See also [`impl Sub for Duration`](duration/units/index.html#addsub)
    ///
    /// # Examples
    /// ```rust
    /// # use embedded_time::{Period, units::*, Instant};
    /// # #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    /// struct Clock;
    /// impl embedded_time::Clock for Clock {
    ///     type Rep = i32;
    ///     const PERIOD: Period = Period::new(1, 1_000);
    ///     // ...
    /// # fn now(&mut self) -> Result<Instant<Self>, embedded_time::Error> {unimplemented!()}
    /// }
    ///
    /// assert_eq!(Instant::<Clock>::new(1) - Seconds(3), Instant::<Clock>::new(-2_999));
    /// assert_eq!(Instant::<Clock>::new(-1) - Milliseconds(5_123), Instant::<Clock>::new(-5_124));
    /// assert_eq!(Instant::<Clock>::new(800) - Milliseconds(700), Instant::<Clock>::new(100));
    /// assert_eq!(Instant::<Clock>::new(5_000_i32) - Milliseconds(700_i64), Instant::<Clock>::new(4_300_i32));
    /// ```
    fn sub(self, rhs: Dur) -> Self::Output {
        let sub_ticks: Clock::Rep = rhs.into_ticks(Clock::PERIOD).unwrap();

        Self {
            ticks: self.ticks.wrapping_sub(&sub_ticks),
        }
    }
}
