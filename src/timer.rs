use crate::{
    duration::{self, *},
    fixed_point::FixedPoint,
    timer::param::*,
    ConversionError, Instant, TimeError,
};
use core::{convert::TryFrom, marker::PhantomData, ops::Add, prelude::v1::*};

pub(crate) mod param {
    #[derive(Debug)]
    pub struct None;

    #[derive(Debug)]
    pub struct Armed;

    #[derive(Debug)]
    pub struct Running;

    #[derive(Debug)]
    pub struct Periodic;

    #[derive(Debug)]
    pub struct OneShot;
}

/// A `Timer` counts toward an expiration, can be polled for elapsed and remaining time, and can be
/// one-shot or continuous/periodic.
#[derive(Debug)]
pub struct Timer<'a, Type, State, Clock: crate::Clock, Dur: Duration> {
    clock: &'a Clock,
    duration: Dur,
    expiration: Instant<Clock>,
    _type: PhantomData<Type>,
    _state: PhantomData<State>,
}

impl<'a, Clock: crate::Clock, Dur: Duration> Timer<'_, param::None, param::None, Clock, Dur> {
    /// Construct a new, `OneShot` `Timer`
    #[allow(clippy::new_ret_no_self)]
    pub fn new(clock: &Clock, duration: Dur) -> Timer<OneShot, Armed, Clock, Dur> {
        Timer::<OneShot, Armed, Clock, Dur> {
            clock,
            duration,
            expiration: Instant::new(Clock::T::from(0)),
            _type: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<'a, Type, State, Clock: crate::Clock, Dur: Duration> Timer<'a, Type, State, Clock, Dur> {
    /// Change timer type to one-shot
    pub fn into_oneshot(self) -> Timer<'a, OneShot, State, Clock, Dur> {
        Timer::<OneShot, State, Clock, Dur> {
            clock: self.clock,
            duration: self.duration,
            expiration: self.expiration,
            _type: PhantomData,
            _state: PhantomData,
        }
    }

    /// Change timer type into periodic
    pub fn into_periodic(self) -> Timer<'a, Periodic, State, Clock, Dur> {
        Timer::<Periodic, State, Clock, Dur> {
            clock: self.clock,
            duration: self.duration,
            expiration: self.expiration,
            _type: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<'a, Type, Clock: crate::Clock, Dur: Duration> Timer<'a, Type, Armed, Clock, Dur> {
    /// Start the timer from this instant
    pub fn start(self) -> Result<Timer<'a, Type, Running, Clock, Dur>, TimeError<Clock::ImplError>>
    where
        Clock::T: TryFrom<Dur::T>,
        Dur: FixedPoint,
    {
        Ok(Timer::<Type, Running, Clock, Dur> {
            clock: self.clock,
            duration: self.duration,
            expiration: self.clock.try_now()?.checked_add_duration(self.duration)?,
            _type: PhantomData,
            _state: PhantomData,
        })
    }
}

impl<Type, Clock: crate::Clock, Dur: Duration> Timer<'_, Type, Running, Clock, Dur> {
    fn _is_expired(&self) -> Result<bool, TimeError<Clock::ImplError>> {
        Ok(self.clock.try_now()? >= self.expiration)
    }

    /// Returns the [`Duration`] of time elapsed since it was started
    ///
    /// **The duration is truncated, not rounded**.
    ///
    /// The units of the [`Duration`] are the same as that used to construct the `Timer`.
    pub fn elapsed(&self) -> Result<Dur, TimeError<Clock::ImplError>>
    where
        Dur: FixedPoint + TryFrom<duration::Generic<Clock::T>, Error = ConversionError>,
        Dur::T: TryFrom<Clock::T>,
        Clock::T: TryFrom<Dur::T>,
    {
        Ok(Dur::try_from(self.clock.try_now()?.duration_since(
            &(self.expiration.checked_sub_duration(self.duration)?),
        )?)?)
    }

    /// Returns the [`Duration`] until the expiration of the timer
    ///
    /// **The duration is truncated, not rounded**.
    ///
    /// The units of the [`Duration`] are the same as that used to construct the `Timer`.
    pub fn remaining(&self) -> Result<Dur, TimeError<Clock::ImplError>>
    where
        Dur: FixedPoint + TryFrom<duration::Generic<Clock::T>, Error = ConversionError>,
        Dur::T: TryFrom<u32> + TryFrom<Clock::T>,
        Clock::T: TryFrom<Dur::T>,
    {
        match self.expiration.duration_since(&self.clock.try_now()?) {
            Ok(duration) => Ok(Dur::try_from(duration)?),
            Err(error) if error == ConversionError::NegDuration => {
                Ok(Dur::new(Dur::T::from(0_u32)))
            }
            Err(error) => Err(error.into()),
        }
    }
}

impl<'a, Clock: crate::Clock, Dur: Duration> Timer<'a, OneShot, Running, Clock, Dur> {
    /// Block until the timer has expired
    pub fn wait(
        self,
    ) -> Result<Timer<'a, OneShot, Armed, Clock, Dur>, TimeError<Clock::ImplError>> {
        // since the timer is running, _is_expired() will return a value
        while !self._is_expired()? {}

        Ok(Timer::<param::None, param::None, Clock, Dur>::new(
            self.clock,
            self.duration,
        ))
    }

    /// Check whether the timer has expired
    ///
    /// The timer is not restarted
    pub fn is_expired(&self) -> Result<bool, TimeError<Clock::ImplError>> {
        self._is_expired()
    }
}

impl<Clock: crate::Clock, Dur: Duration> Timer<'_, Periodic, Running, Clock, Dur> {
    /// Block until the timer has expired
    ///
    /// The timer is restarted
    pub fn wait(self) -> Result<Self, TimeError<Clock::ImplError>>
    where
        Instant<Clock>: Add<Dur, Output = Instant<Clock>>,
    {
        // since the timer is running, _is_expired() will return a value
        while !self._is_expired()? {}

        Ok(Self {
            clock: self.clock,
            duration: self.duration,
            // The `+` will never panic since this duration has already applied to the same
            // `Instant` type without a problem
            expiration: self.expiration + self.duration,
            _type: PhantomData,
            _state: PhantomData,
        })
    }

    /// Check whether a _periodic_ timer has elapsed
    ///
    /// The timer is restarted if it has elapsed.
    pub fn period_complete(&mut self) -> Result<bool, TimeError<Clock::ImplError>>
    where
        Instant<Clock>: Add<Dur, Output = Instant<Clock>>,
    {
        // since the timer is running, _is_expired() will return a value
        if self._is_expired()? {
            // The `+` will never panic since this duration has already applied to the same
            // `Instant` type without a problem
            self.expiration = self.expiration + self.duration;

            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
#[allow(unsafe_code)]
mod test {
    use crate::{
        duration::{units::*, Duration},
        fixed_point::FixedPoint,
        fraction::Fraction,
        Clock as _, Instant,
    };
    use core::convert::Infallible;
    use crossbeam_utils::thread;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TICKS: AtomicU64 = AtomicU64::new(0);

    #[derive(Debug)]
    struct Clock;
    impl crate::Clock for Clock {
        type T = u64;
        type ImplError = Infallible;
        const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000);

        fn try_now(&self) -> Result<Instant<Self>, crate::clock::Error<Self::ImplError>> {
            Ok(Instant::new(TICKS.load(Ordering::SeqCst)))
        }
    }

    #[test]
    fn oneshot_wait() {
        init_ticks();
        let clock = Clock;

        let timer = clock.new_timer(1_u32.seconds()).start().unwrap();

        thread::scope(|s| {
            let timer_handle = s.spawn(|_| timer.wait());

            add_to_ticks(1_u32.seconds());

            let result = timer_handle.join();

            assert!(result.is_ok());

            add_to_ticks(1_u32.seconds());

            let timer = result.unwrap().unwrap().start().unwrap();
            assert!(!timer.is_expired().unwrap());

            let timer_handle = s.spawn(|_| timer.wait());
            add_to_ticks(1_u32.seconds());

            assert!(timer_handle.join().is_ok());
        })
        .unwrap();
    }

    #[test]
    fn periodic_wait() {
        init_ticks();
        let clock = Clock;

        let timer = clock
            .new_timer(1_u32.seconds())
            .into_periodic()
            .start()
            .unwrap();

        thread::scope(|s| {
            let timer_handle = s.spawn(|_| timer.wait());

            add_to_ticks(1_u32.seconds());

            let result = timer_handle.join();

            assert!(result.is_ok());

            let timer = result.unwrap();

            // WHEN blocking on a timer
            let timer_handle = s.spawn(|_| timer.unwrap().wait());

            add_to_ticks(1_u32.seconds());

            assert!(timer_handle.join().is_ok());
        })
        .unwrap();
    }

    #[test]
    fn periodic_expiration() {
        init_ticks();
        let clock = Clock;

        let mut timer = clock
            .new_timer(1_u32.seconds())
            .into_periodic()
            .start()
            .unwrap();

        add_to_ticks(2_u32.seconds());

        assert!(timer.period_complete().unwrap());
        assert!(timer.period_complete().unwrap());
    }

    #[test]
    fn read_timer() {
        init_ticks();
        let clock = Clock;

        let timer = clock.new_timer(2_u32.seconds()).start().unwrap();

        add_to_ticks(1_u32.milliseconds());

        assert_eq!(timer.elapsed(), Ok(0_u32.seconds()));
        assert_eq!(timer.remaining(), Ok(1_u32.seconds()));

        add_to_ticks(1_u32.seconds());

        assert_eq!(timer.elapsed(), Ok(1_u32.seconds()));
        assert_eq!(timer.remaining(), Ok(0_u32.seconds()));

        add_to_ticks(1_u32.seconds());

        assert_eq!(timer.elapsed(), Ok(2_u32.seconds()));
        assert_eq!(timer.remaining(), Ok(0_u32.seconds()));

        add_to_ticks(1_u32.seconds());

        assert_eq!(timer.elapsed(), Ok(3_u32.seconds()));
        assert_eq!(timer.remaining(), Ok(0_u32.seconds()));
    }

    fn init_ticks() {}

    fn add_to_ticks<Dur: Duration>(duration: Dur)
    where
        Dur: FixedPoint,
        u64: From<Dur::T>,
    {
        let ticks = TICKS.load(Ordering::SeqCst);
        let ticks = ticks
            + duration
                .into_ticks::<<Clock as crate::Clock>::T>(Clock::SCALING_FACTOR)
                .unwrap();
        TICKS.store(ticks, Ordering::SeqCst);
    }
}
