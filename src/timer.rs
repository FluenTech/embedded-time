use crate::duration::TryConvertFrom;
use crate::timer::param::*;
use crate::{prelude::*, units::*, Duration, Instant, TimeInt};
use core::{convert::TryFrom, marker::PhantomData, ops::Add, prelude::v1::*};

pub(crate) mod param {
    #[derive(Debug)]
    pub struct None;

    #[derive(Debug)]
    pub struct Disarmed;

    #[derive(Debug)]
    pub struct Armed;

    #[derive(Debug)]
    pub struct Running;

    #[derive(Debug)]
    pub struct Periodic;

    #[derive(Debug)]
    pub struct OneShot;
}

/// A `Timer` counts toward an expiration, can be polled for elapsed and remaining time, as
/// well as optionally execute a task upon expiration.
#[derive(Debug)]
pub struct Timer<Type, State, Clock: crate::Clock, Dur: Duration> {
    duration: Option<Dur>,
    expiration: Option<Instant<Clock>>,
    _type: PhantomData<Type>,
    _state: PhantomData<State>,
}

impl<Clock: crate::Clock, Dur: Duration> Timer<param::None, param::None, Clock, Dur> {
    /// Construct a new, `OneShot` `Timer`
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Timer<OneShot, Disarmed, Clock, Dur> {
        Timer::<OneShot, Disarmed, Clock, Dur> {
            duration: Option::None,
            expiration: Option::None,
            _type: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<Type, State, Clock: crate::Clock, Dur: Duration> Timer<Type, State, Clock, Dur> {
    /// Change timer type to one-shot
    pub fn into_oneshot(self) -> Timer<OneShot, State, Clock, Dur> {
        Timer::<OneShot, State, Clock, Dur> {
            duration: self.duration,
            expiration: self.expiration,
            _type: PhantomData,
            _state: PhantomData,
        }
    }

    /// Change timer type into periodic
    pub fn into_periodic(self) -> Timer<Periodic, State, Clock, Dur> {
        Timer::<Periodic, State, Clock, Dur> {
            duration: self.duration,
            expiration: self.expiration,
            _type: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<Type, Clock: crate::Clock, Dur: Duration> Timer<Type, Disarmed, Clock, Dur> {
    /// Set the [`Duration`](trait.Duration.html) of the timer
    ///
    /// This _arms_ the timer (makes it ready to run).
    pub fn set_duration(self, duration: Dur) -> Timer<Type, Armed, Clock, Dur> {
        Timer::<Type, Armed, Clock, Dur> {
            duration: Some(duration),
            expiration: Option::None,
            _type: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<Type, Clock: crate::Clock, Dur: Duration> Timer<Type, Armed, Clock, Dur> {
    /// Start the _armed_ timer from this instant
    pub fn start(self) -> Timer<Type, Running, Clock, Dur>
    where
        Instant<Clock>: Add<Dur, Output = Instant<Clock>>,
    {
        Timer::<Type, Running, Clock, Dur> {
            duration: self.duration,
            expiration: Some(Clock::now() + self.duration.unwrap()),
            _type: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<Type, Clock: crate::Clock, Dur: Duration> Timer<Type, Running, Clock, Dur> {
    fn _is_expired(&self) -> bool {
        Clock::now() >= self.expiration.unwrap()
    }

    /// Returns the [`Duration`](trait.Duration.html) of time elapsed since it was started
    ///
    /// The units of the [`Duration`](trait.Duration.html) are the same as that used with
    /// [`set_duration()`](struct.Timer.html#method.set_duration).
    pub fn elapsed(&self) -> Dur
    where
        Dur::Rep: TryFrom<Clock::Rep>,
        Clock::Rep: TryFrom<Dur::Rep>,
    {
        Clock::now()
            .duration_since(&(self.expiration.unwrap() - self.duration.unwrap()))
            .unwrap()
    }

    /// Returns the [`Duration`](trait.Duration.html) until the expiration of the timer
    ///
    /// The units of the [`Duration`](trait.Duration.html) are the same as that used with
    /// [`set_duration()`](struct.Timer.html#method.set_duration).
    pub fn remaining(&self) -> Dur
    where
        Dur::Rep: TryFrom<Clock::Rep>,
        Clock::Rep: TryFrom<Dur::Rep>,
        Dur: TryConvertFrom<Seconds<i32>>,
    {
        if let Some(duration) = self.expiration.unwrap().duration_since(&Clock::now()) {
            duration
        } else {
            0.seconds().try_convert_into().unwrap()
        }
    }
}

impl<Clock: crate::Clock, Dur: Duration> Timer<OneShot, Running, Clock, Dur> {
    /// Block until the timer has expired
    pub fn wait(self) {
        // since the timer is running, _is_expired() will return a value
        while !self._is_expired() {}
    }
}

impl<Clock: crate::Clock, Dur: Duration> Timer<OneShot, Running, Clock, Dur> {
    /// Check whether the timer has expired
    ///
    /// The timer is not restarted
    pub fn is_expired(&self) -> bool {
        self._is_expired()
    }
}

impl<Clock: crate::Clock, Dur: Duration> Timer<Periodic, Running, Clock, Dur> {
    /// Block until the timer has expired
    ///
    /// The timer is restarted
    pub fn wait(self) -> Self
    where
        Instant<Clock>: Add<Dur, Output = Instant<Clock>>,
    {
        // since the timer is running, _is_expired() will return a value
        while !self._is_expired() {}

        Self {
            duration: self.duration,
            expiration: self
                .expiration
                .map(|expiration| expiration + self.duration.unwrap()),
            _type: PhantomData,
            _state: PhantomData,
        }
    }

    /// Check whether a _periodic_ timer has elapsed
    ///
    /// The timer is restarted if it has elapsed.
    pub fn period_complete(&mut self) -> bool
    where
        Instant<Clock>: Add<Dur, Output = Instant<Clock>>,
    {
        // since the timer is running, _is_expired() will return a value
        if self._is_expired() {
            self.expiration = Some(self.expiration.unwrap() + self.duration.unwrap());

            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(unsafe_code)]

    use crate::{units::*, Clock as _, Duration, Instant, Period, TimeInt};
    use std::convert::TryFrom;

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct Clock;

    static mut START: Option<std::time::Instant> = None;

    impl crate::Clock for Clock {
        type Rep = i64;
        const PERIOD: Period = Period::new(1, 64_000_000);

        fn now() -> Instant<Self> {
            let since_start = unsafe { START.unwrap() }.elapsed();
            let ticks = Nanoseconds::<i64>::try_from(since_start)
                .unwrap()
                .into_ticks(Self::PERIOD)
                .unwrap();
            Instant::new(ticks)
        }
    }

    fn init_start_time() {
        unsafe {
            START = Some(std::time::Instant::now());
        }
    }

    #[test]
    fn oneshot_wait() {
        init_start_time();

        // WHEN blocking on a timer
        Clock::new_timer().set_duration(1.seconds()).start().wait();

        // THEN the block occurs for _at least_ the given duration
        unsafe {
            assert!(Seconds::<i32>::try_from(START.unwrap().elapsed()).unwrap() >= 1.seconds());
        }
    }

    #[test]
    fn periodic_wait() {
        init_start_time();

        let timer = Clock::new_timer()
            .into_periodic()
            .set_duration(1.seconds())
            .start()
            .wait();

        unsafe {
            assert!(Seconds::<i32>::try_from(START.unwrap().elapsed()).unwrap() == 1.seconds());
        }

        let timer = timer.wait();
        unsafe {
            assert!(Seconds::<i32>::try_from(START.unwrap().elapsed()).unwrap() == 2.seconds());
        }

        timer.wait();
        unsafe {
            assert!(Seconds::<i32>::try_from(START.unwrap().elapsed()).unwrap() == 3.seconds());
        }
    }

    #[test]
    fn periodic_expiration() {
        init_start_time();

        let mut timer = Clock::new_timer()
            .into_periodic()
            .set_duration(1.seconds())
            .start();

        std::thread::sleep(std::time::Duration::from_secs(2));

        assert!(timer.period_complete());
        assert!(timer.period_complete());
    }

    #[test]
    fn read_timer() {
        init_start_time();

        let timer = Clock::new_timer().set_duration(2.seconds()).start();

        assert_eq!(timer.elapsed(), 0.seconds());
        assert_eq!(timer.remaining(), 1.seconds());

        std::thread::sleep(std::time::Duration::from_secs(1));

        assert_eq!(timer.elapsed(), 1.seconds());
        assert_eq!(timer.remaining(), 0.seconds());

        std::thread::sleep(std::time::Duration::from_secs(1));

        assert_eq!(timer.elapsed(), 2.seconds());
        assert_eq!(timer.remaining(), (0).seconds());

        std::thread::sleep(std::time::Duration::from_secs(1));

        assert_eq!(timer.elapsed(), 3.seconds());
        assert_eq!(timer.remaining(), (-1).seconds());
    }
}
