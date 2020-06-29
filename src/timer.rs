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
    pub struct Scheduled;

    #[derive(Debug)]
    pub struct Periodic;

    #[derive(Debug)]
    pub struct OneShot;
}

/// A builder for configuring a new [`Timer`] before starting/scheduling
#[derive(Debug)]
pub struct TimerBuilder<Type, State, Clock: crate::Clock, Dur: Duration> {
    duration: Option<Dur>,
    expiration: Option<Instant<Clock>>,
    _type: PhantomData<Type>,
    _state: PhantomData<State>,
}

impl<Clock: crate::Clock, Dur: Duration> TimerBuilder<param::None, param::None, Clock, Dur> {
    /// Construct a new, `OneShot` timer builder
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> TimerBuilder<OneShot, Disarmed, Clock, Dur> {
        TimerBuilder::<OneShot, Disarmed, Clock, Dur> {
            duration: Option::None,
            expiration: Option::None,
            _type: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<Type, State, Clock: crate::Clock, Dur: Duration> TimerBuilder<Type, State, Clock, Dur> {
    /// Change timer type to one-shot
    pub fn into_oneshot(self) -> TimerBuilder<OneShot, State, Clock, Dur> {
        TimerBuilder::<OneShot, State, Clock, Dur> {
            duration: self.duration,
            expiration: self.expiration,
            _type: PhantomData,
            _state: PhantomData,
        }
    }

    /// Change timer type into periodic
    pub fn into_periodic(self) -> TimerBuilder<Periodic, State, Clock, Dur> {
        TimerBuilder::<Periodic, State, Clock, Dur> {
            duration: self.duration,
            expiration: self.expiration,
            _type: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<Type, Clock: crate::Clock, Dur: Duration> TimerBuilder<Type, Disarmed, Clock, Dur> {
    /// Set the [`Duration`](trait.Duration.html) of the timer
    ///
    /// This _arms_ the timer (makes it ready to run).
    pub fn set_duration(self, duration: Dur) -> TimerBuilder<Type, Armed, Clock, Dur> {
        TimerBuilder::<Type, Armed, Clock, Dur> {
            duration: Some(duration),
            expiration: Option::None,
            _type: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<Type, Clock: crate::Clock, Dur: Duration> TimerBuilder<Type, Armed, Clock, Dur> {
    /// Start the _armed_ timer from this instant
    pub fn start(self) -> Timer<Type, Running, Clock, Dur, impl FnMut()>
    where
        Instant<Clock>: Add<Dur, Output = Instant<Clock>>,
    {
        Timer::<Type, Running, Clock, Dur, _> {
            duration: self.duration,
            expiration: Some(Clock::now() + self.duration.unwrap()),
            task: || {},
            _type: PhantomData,
            _state: PhantomData,
        }
    }

    /// Start the timer while setting a task to be executed upon expiration
    pub fn schedule(self, task: impl FnMut()) -> Timer<Type, Scheduled, Clock, Dur, impl FnMut()>
    where
        Instant<Clock>: Add<Dur, Output = Instant<Clock>>,
    {
        Timer::<Type, Scheduled, Clock, Dur, _> {
            duration: self.duration,
            expiration: Some(Clock::now() + self.duration.unwrap()),
            task,
            _type: PhantomData,
            _state: PhantomData,
        }
    }
}

/// A `Timer` counts toward an expiration, can be polled for elapsed and remaining time, as
/// well as optionally execute a task upon expiration.
#[derive(Debug)]
pub struct Timer<Type, State, Clock: crate::Clock, Dur: Duration, Task: FnMut()> {
    duration: Option<Dur>,
    expiration: Option<Instant<Clock>>,
    task: Task,
    _type: PhantomData<Type>,
    _state: PhantomData<State>,
}

impl<Type, Clock: crate::Clock, Dur: Duration, Task: FnMut()>
    Timer<Type, Running, Clock, Dur, Task>
{
    fn _is_expired(&self) -> bool {
        Clock::now() >= self.expiration.unwrap()
    }

    /// Returns the [`Duration`](trait.Duration.html) of time elapsed since it was started
    ///
    /// The units of the [`Duration`](trait.Duration.html) are the same as that used with
    /// [`set_duration()`](struct.TimerBuilder.html#method.set_duration).
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
    /// [`set_duration()`](struct.TimerBuilder.html#method.set_duration).
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

impl<Clock: crate::Clock, Dur: Duration, Task: FnMut()> Timer<OneShot, Running, Clock, Dur, Task> {
    /// Block until the timer has expired
    pub fn wait(self) -> TimerBuilder<OneShot, Armed, Clock, Dur> {
        // since the timer is running, _is_expired() will return a value
        while !self._is_expired() {}

        TimerBuilder::<param::None, param::None, Clock, Dur>::new()
            .set_duration(self.duration.unwrap())
    }
}

impl<Clock: crate::Clock, Dur: Duration, Task: FnMut()> Timer<OneShot, Running, Clock, Dur, Task> {
    /// Check whether the timer has expired
    ///
    /// The timer is not restarted
    pub fn is_expired(&self) -> bool {
        self._is_expired()
    }
}

impl<Clock: crate::Clock, Dur: Duration, Task: FnMut()> Timer<Periodic, Running, Clock, Dur, Task> {
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
            task: self.task,
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
    use crate::{units::*, Clock as _, Duration, Instant, Period, TimeInt};
    use std::convert::TryFrom;
    use std::sync::atomic::{AtomicI64, Ordering};
    use std::thread;

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct Clock;

    static TICKS: AtomicI64 = AtomicI64::new(0);

    impl crate::Clock for Clock {
        type Rep = i64;
        const PERIOD: Period = Period::new(1, 1_000);

        fn now() -> Instant<Self> {
            Instant::new(TICKS.load(Ordering::Acquire))
        }
    }

    #[test]
    fn oneshot_wait() {
        init_ticks();

        let timer = Clock::new_timer().set_duration(1.seconds()).start();
        let timer_handle = thread::spawn(move || timer.wait());

        add_to_ticks(1.seconds());

        let result = timer_handle.join();

        assert!(result.is_ok());

        let timer = result.unwrap().start();
        let timer_handle = thread::spawn(move || timer.wait());

        add_to_ticks(1.seconds());

        assert!(timer_handle.join().is_ok());
    }

    #[test]
    fn periodic_wait() {
        init_ticks();

        let timer = Clock::new_timer()
            .into_periodic()
            .set_duration(1.seconds())
            .start();
        let timer_handle = thread::spawn(move || timer.wait());

        add_to_ticks(1.seconds());

        let result = timer_handle.join();

        assert!(result.is_ok());

        let timer = result.unwrap();

        // WHEN blocking on a timer
        let timer_handle = thread::spawn(move || timer.wait());

        add_to_ticks(1.seconds());

        assert!(timer_handle.join().is_ok());
    }

    #[test]
    fn periodic_expiration() {
        init_ticks();

        let mut timer = Clock::new_timer()
            .into_periodic()
            .set_duration(1.seconds())
            .start();

        add_to_ticks(2.seconds());

        assert!(timer.period_complete());
        assert!(timer.period_complete());
    }

    #[test]
    fn read_timer() {
        init_ticks();

        let timer = Clock::new_timer().set_duration(2.seconds()).start();

        add_to_ticks(1.milliseconds());

        assert_eq!(timer.elapsed(), 0.seconds());
        assert_eq!(timer.remaining(), 1.seconds());

        add_to_ticks(1.seconds());

        assert_eq!(timer.elapsed(), 1.seconds());
        assert_eq!(timer.remaining(), 0.seconds());

        add_to_ticks(1.seconds());

        assert_eq!(timer.elapsed(), 2.seconds());
        assert_eq!(timer.remaining(), (0).seconds());

        add_to_ticks(1.seconds());

        assert_eq!(timer.elapsed(), 3.seconds());
        assert_eq!(timer.remaining(), (-1).seconds());
    }

    #[test]
    fn expiration_task() {
        init_ticks();

        let mut x = 1;

        (Clock::new_timer()
            .set_duration(1.seconds())
            .schedule(|| x += 1)
            .task)();

        assert_eq!(x, 2);
    }

    fn init_ticks() {}

    fn add_to_ticks<Dur: Duration>(duration: Dur) {
        let ticks = TICKS.load(Ordering::Acquire);
        let ticks = ticks
            + duration
                .into_ticks::<<Clock as crate::Clock>::Rep>(Clock::PERIOD)
                .unwrap();
        TICKS.store(ticks, Ordering::Release);
    }
}
