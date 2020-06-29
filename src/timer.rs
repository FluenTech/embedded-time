use crate::{duration::TryConvertFrom, timer::param::*, traits::*, units::*, Duration, Instant};
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
pub struct Timer<'a, Type, State, Clock: crate::Clock, Dur: Duration> {
    clock: &'a Clock,
    duration: Option<Dur>,
    expiration: Option<Instant<Clock>>,
    _type: PhantomData<Type>,
    _state: PhantomData<State>,
}

impl<'a, Clock: crate::Clock, Dur: Duration> Timer<'_, param::None, param::None, Clock, Dur> {
    /// Construct a new, `OneShot` `Timer`
    #[allow(clippy::new_ret_no_self)]
    pub fn new(clock: &Clock) -> Timer<OneShot, Disarmed, Clock, Dur> {
        Timer::<OneShot, Disarmed, Clock, Dur> {
            clock,
            duration: Option::None,
            expiration: Option::None,
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

impl<'a, Type, Clock: crate::Clock, Dur: Duration> Timer<'a, Type, Disarmed, Clock, Dur> {
    /// Set the [`Duration`](trait.Duration.html) of the timer
    ///
    /// This _arms_ the timer (makes it ready to run).
    pub fn set_duration(self, duration: Dur) -> Timer<'a, Type, Armed, Clock, Dur> {
        Timer::<Type, Armed, Clock, Dur> {
            clock: self.clock,
            duration: Some(duration),
            expiration: Option::None,
            _type: PhantomData,
            _state: PhantomData,
        }
    }
}
impl<'a, Type, Clock: crate::Clock, Dur: Duration> Timer<'a, Type, Armed, Clock, Dur> {
    /// Start the _armed_ timer from this instant
    pub fn start(self) -> Timer<'a, Type, Running, Clock, Dur>
    where
        Instant<Clock>: Add<Dur, Output = Instant<Clock>>,
    {
        Timer::<Type, Running, Clock, Dur> {
            clock: self.clock,
            duration: self.duration,
            expiration: Some(self.clock.now().unwrap() + self.duration.unwrap()),
            _type: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<Type, Clock: crate::Clock, Dur: Duration> Timer<'_, Type, Running, Clock, Dur> {
    fn _is_expired(&self) -> bool {
        self.clock.now().unwrap() >= self.expiration.unwrap()
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
        self.clock
            .now()
            .unwrap()
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
        Dur: TryConvertFrom<Seconds<u32>>,
    {
        if let Ok(duration) = self
            .expiration
            .unwrap()
            .duration_since(&self.clock.now().unwrap())
        {
            duration
        } else {
            0.seconds().try_convert_into().unwrap()
        }
    }
}

impl<'a, Clock: crate::Clock, Dur: Duration> Timer<'a, OneShot, Running, Clock, Dur> {
    /// Block until the timer has expired
    pub fn wait(self) -> Timer<'a, OneShot, Armed, Clock, Dur> {
        // since the timer is running, _is_expired() will return a value
        while !self._is_expired() {}

        Timer::<param::None, param::None, Clock, Dur>::new(self.clock)
            .set_duration(self.duration.unwrap())
    }

    /// Check whether the timer has expired
    ///
    /// The timer is not restarted
    pub fn is_expired(&self) -> bool {
        self._is_expired()
    }
}

impl<Clock: crate::Clock, Dur: Duration> Timer<'_, Periodic, Running, Clock, Dur> {
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
            clock: self.clock,
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
#[allow(unused_imports)]
#[allow(unsafe_code)]
mod test {
    use crate::{traits::*, units::*, Duration, Error, Instant, Period};
    use std::convert::{Infallible, TryFrom, TryInto};

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct Clock;

    static mut START: Option<std::time::Instant> = None;

    impl crate::Clock for Clock {
        type Rep = u64;
        const PERIOD: Period = <Period>::new(1, 64_000_000);
        type ImplError = Infallible;

        fn now(&self) -> Result<Instant<Self>, crate::clock::Error<Self::ImplError>> {
            let since_start = unsafe { START.unwrap() }.elapsed();
            let ticks = Nanoseconds::<u64>::try_from(since_start)
                .unwrap()
                .into_ticks(Self::PERIOD)
                .unwrap();
            Ok(Instant::new(ticks))
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
        let clock = Clock;

        let timer = clock
            .new_timer()
            .set_duration(1_u32.seconds())
            .start()
            .wait();

        unsafe {
            assert!(Seconds::<u32>::try_from(START.unwrap().elapsed()).unwrap() >= 1_u32.seconds());
        }

        // WHEN blocking on a timer
        timer.start().wait();

        // THEN the block occurs for _at least_ the given duration
        unsafe {
            assert!(Seconds::<u32>::try_from(START.unwrap().elapsed()).unwrap() >= 2_u32.seconds());
        }
    }

    #[test]
    fn periodic_wait() {
        init_start_time();
        let clock = Clock;

        let timer = clock
            .new_timer()
            .into_periodic()
            .set_duration(1_u32.seconds())
            .start()
            .wait();

        unsafe {
            assert!(Seconds::<u32>::try_from(START.unwrap().elapsed()).unwrap() >= 1_u32.seconds());
        }

        let timer = timer.wait();
        unsafe {
            assert!(Seconds::<u32>::try_from(START.unwrap().elapsed()).unwrap() >= 2_u32.seconds());
        }

        timer.wait();
        unsafe {
            assert!(Seconds::<u32>::try_from(START.unwrap().elapsed()).unwrap() >= 3_u32.seconds());
        }
    }

    #[test]
    fn periodic_expiration() {
        init_start_time();
        let clock = Clock;

        let mut timer = clock
            .new_timer()
            .into_periodic()
            .set_duration(1_u32.seconds())
            .start();

        std::thread::sleep(std::time::Duration::from_secs(2));

        assert!(timer.period_complete());
        assert!(timer.period_complete());
    }

    #[test]
    fn read_timer() {
        init_start_time();
        let clock = Clock;

        let timer = clock.new_timer().set_duration(2_u32.seconds()).start();

        assert_eq!(timer.elapsed(), 0_u32.seconds());
        assert_eq!(timer.remaining(), 1_u32.seconds());

        std::thread::sleep(std::time::Duration::from_secs(1));

        assert_eq!(timer.elapsed(), 1_u32.seconds());
        assert_eq!(timer.remaining(), 0_u32.seconds());

        std::thread::sleep(std::time::Duration::from_secs(1));

        assert_eq!(timer.elapsed(), 2_u32.seconds());
        assert_eq!(timer.remaining(), 0_u32.seconds());

        std::thread::sleep(std::time::Duration::from_secs(1));

        assert_eq!(timer.elapsed(), 3_u32.seconds());
        assert_eq!(timer.remaining(), 0_u32.seconds());
    }
}
