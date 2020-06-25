use crate::timer::param::{Armed, Disarmed, OneShot, Periodic, Running};
use crate::{Duration, Instant, TimeInt};
use core::convert::TryFrom;
use core::marker::PhantomData;
use core::ops::Add;

pub mod param {
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

#[derive(Debug)]
pub struct Timer<Type, State, Clock: crate::Clock, Dur: Duration> {
    duration: Option<Dur>,
    expiration: Option<Instant<Clock>>,
    _type: PhantomData<Type>,
    _state: PhantomData<State>,
}

impl<Clock: crate::Clock, Dur: Duration> Timer<param::None, param::None, Clock, Dur> {
    pub fn new() -> Timer<OneShot, Disarmed, Clock, Dur> {
        Timer::<OneShot, Disarmed, Clock, Dur> {
            duration: None,
            expiration: None,
            _type: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<Type, State, Clock: crate::Clock, Dur: Duration> Timer<Type, State, Clock, Dur> {
    pub fn into_oneshot(self) -> Timer<OneShot, State, Clock, Dur> {
        Timer::<OneShot, State, Clock, Dur> {
            duration: self.duration,
            expiration: self.expiration,
            _type: PhantomData,
            _state: PhantomData,
        }
    }

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
    pub fn set_duration(self, duration: Dur) -> Timer<Type, Armed, Clock, Dur> {
        Timer::<Type, Armed, Clock, Dur> {
            duration: Some(duration),
            expiration: None,
            _type: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<Type, Clock: crate::Clock, Dur: Duration> Timer<Type, Armed, Clock, Dur> {
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
}

impl<Clock: crate::Clock, Dur: Duration> Timer<OneShot, Running, Clock, Dur> {
    pub fn wait(self) {
        // since the timer is running, _is_expired() will return a value
        while !self._is_expired() {}
    }
}

impl<Clock: crate::Clock, Dur: Duration> Timer<OneShot, Running, Clock, Dur> {
    pub fn is_expired(&self) -> bool {
        self._is_expired()
    }
}

impl<Clock: crate::Clock, Dur: Duration> Timer<Periodic, Running, Clock, Dur> {
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

    use crate::{units::*, Clock as _, Duration, Instant, Period, TimeInt, Timer};
    use std::convert::{TryFrom, TryInto};

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

        Clock::new_timer().set_duration(1.seconds()).start().wait();

        unsafe {
            assert!(Seconds::<i32>::try_from(START.unwrap().elapsed()).unwrap() == 1.seconds());
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

        let timer = timer.wait();
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
}
