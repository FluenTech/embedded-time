use core::{
    convert::{Infallible, TryFrom, TryInto},
    fmt::{self, Formatter},
};
use embedded_time::{self as time, duration::*, Clock as _};

struct MockClock64;
impl time::Clock for MockClock64 {
    type T = u64;
    type ImplError = Infallible;
    const SCALING_FACTOR: time::fraction::Fraction = <time::fraction::Fraction>::new(1, 64_000_000);

    fn try_now(&self) -> Result<time::Instant<Self>, time::clock::Error<Self::ImplError>> {
        Ok(time::Instant::new(128_000_000))
    }
}

#[derive(Debug)]
struct MockClock32;

impl time::Clock for MockClock32 {
    type T = u32;
    type ImplError = Infallible;
    const SCALING_FACTOR: time::fraction::Fraction = <time::fraction::Fraction>::new(1, 16_000_000);

    fn try_now(&self) -> Result<time::Instant<Self>, time::clock::Error<Self::ImplError>> {
        Ok(time::Instant::new(32_000_000))
    }
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum ClockImplError {
    NotStarted,
}

#[derive(Debug)]
struct BadClock;

impl time::Clock for BadClock {
    type T = u32;
    type ImplError = ClockImplError;
    const SCALING_FACTOR: time::fraction::Fraction = <time::fraction::Fraction>::new(1, 16_000_000);

    fn try_now(&self) -> Result<time::Instant<Self>, time::clock::Error<Self::ImplError>> {
        Err(time::clock::Error::Other(ClockImplError::NotStarted))
    }
}

fn get_time<Clock: time::Clock>(clock: &Clock)
where
    u32: TryFrom<Clock::T>,
{
    assert_eq!(
        clock
            .try_now()
            .ok()
            .unwrap()
            .duration_since_epoch()
            .try_into(),
        Ok(Seconds(2_u32))
    );
}

#[test]
fn common_types() {
    let then = MockClock32.try_now().unwrap();
    let now = MockClock32.try_now().unwrap();

    let clock64 = MockClock64 {};
    let clock32 = MockClock32 {};

    get_time(&clock64);
    get_time(&clock32);

    let then = then - Seconds(1_u32);
    assert_ne!(then, now);
    assert!(then < now);
}

#[test]
fn clock_error() {
    assert_eq!(
        BadClock.try_now(),
        Err(time::clock::Error::Other(ClockImplError::NotStarted))
    );
}

struct Timestamp<Clock>(time::Instant<Clock>)
where
    Clock: time::Clock;

impl<Clock> Timestamp<Clock>
where
    Clock: time::Clock,
{
    pub fn new(instant: time::Instant<Clock>) -> Self {
        Timestamp(instant)
    }
}

impl<Clock> fmt::Display for Timestamp<Clock>
where
    Clock: time::Clock,
    u64: From<Clock::T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let duration = Milliseconds::<u64>::try_from(self.0.duration_since_epoch())
            .map_err(|_| fmt::Error {})?;

        let hours = Hours::<u32>::try_from(duration).map_err(|_| fmt::Error {})?;
        let minutes = Minutes::<u32>::try_from(duration).map_err(|_| fmt::Error {})? % Hours(1_u32);
        let seconds =
            Seconds::<u32>::try_from(duration).map_err(|_| fmt::Error {})? % Minutes(1_u32);
        let milliseconds =
            Milliseconds::<u32>::try_from(duration).map_err(|_| fmt::Error {})? % Seconds(1_u32);

        f.write_fmt(format_args!(
            "{}:{:02}:{:02}.{:03}",
            hours, minutes, seconds, milliseconds
        ))
    }
}

#[test]
fn format() {
    let timestamp = Timestamp::new(time::Instant::<MockClock64>::new(321_643_392_000));
    let formatted_timestamp = timestamp.to_string();
    assert_eq!(formatted_timestamp, "1:23:45.678");
}
