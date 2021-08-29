use core::convert::TryInto;
use embedded_time::{
    self as time,
    duration::{self, *},
    Clock as _, Instant,
};
use test_case::test_case;

#[derive(Debug)]
struct Clock;

impl time::Clock for Clock {
    type T = u32;
    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000);

    fn try_now(&self) -> Result<Instant<Self>, time::clock::Error> {
        static mut TICKS: u32 = 0;
        unsafe {
            TICKS += 1;
        }
        Ok(Instant::new(unsafe { TICKS }))
    }
}

#[test]
fn duration_since() {
    let clock = Clock;
    let instant1 = clock.try_now().unwrap();
    let instant2 = clock.try_now().unwrap();
    let diff = instant2.checked_duration_since(&instant1);
    assert_eq!(
        diff,
        Some(duration::Generic::new(1_u32, Fraction::new(1, 1_000)))
    );

    let micros: Result<Microseconds<<Clock as time::Clock>::T>, _> = diff.unwrap().try_into();
    assert_eq!(micros, Ok(Microseconds(1_000_u32)));

    let diff = instant1.checked_duration_since(&instant2);
    assert_eq!(diff, None);
}

#[test]
fn duration_until() {
    let clock = Clock;
    let instant1 = clock.try_now().unwrap();
    let instant2 = clock.try_now().unwrap();
    let diff = instant1.checked_duration_until(&instant2);
    assert_eq!(
        diff,
        Some(duration::Generic::new(1_u32, Fraction::new(1, 1_000)))
    );

    let micros: Result<Microseconds<<Clock as time::Clock>::T>, _> = diff.unwrap().try_into();
    assert_eq!(micros, Ok(Microseconds(1_000_u32)));

    let diff = instant2.checked_duration_until(&instant1);
    assert_eq!(diff, None);
}

#[test]
fn duration_since_epoch() {
    assert_eq!(
        Instant::<Clock>::new(u32::MAX).duration_since_epoch(),
        duration::Generic::from(Milliseconds(u32::MAX))
    );
}

#[test_case(0, u32::MAX/2 => Instant::<Clock>::new(u32::MAX / 2) ; "Add the maximum allowed duration")]
fn instant_add_duration(base: u32, addition: u32) -> Instant<Clock> {
    Instant::<Clock>::new(base) + Milliseconds(addition)
}

#[test]
#[should_panic]
fn add_panic() {
    let _ = Instant::<Clock>::new(0) + Milliseconds(u32::MAX / 2 + 1);
}

#[test_case(u32::MAX/2, 0 => Instant::<Clock>::new(u32::MAX / 2) ; "Add the maximum allowed duration")]
fn duration_add_instant(base: u32, addition: u32) -> Instant<Clock> {
    Milliseconds(base) + Instant::<Clock>::new(addition)
}

#[test_case(0, Milliseconds(u32::MAX/2) => Some(Instant::<Clock>::new(u32::MAX / 2)) ; "Add the maximum allowed duration")]
#[test_case(0, Milliseconds(u32::MAX/2 + 1) => None ; "Overflow due to the duration being too large")]
fn checked_add<Dur: Duration>(base: u32, addition: Dur) -> Option<Instant<Clock>>
where
    Dur::T: Into<u32>,
{
    Instant::<Clock>::new(base).checked_add(addition)
}

#[test_case(u32::MAX, Milliseconds(u32::MAX/2) => Some(Instant::<Clock>::new(u32::MAX - (u32::MAX / 2))) ; "Subtract the maximum allowed duration")]
#[test_case(u32::MAX, Milliseconds(u32::MAX/2 + 1) => None ; "Overflow due to the duration being too large")]
fn checked_sub<Dur: Duration>(base: u32, subtrahend: Dur) -> Option<Instant<Clock>>
where
    Dur::T: Into<u32>,
{
    Instant::<Clock>::new(base).checked_sub(subtrahend)
}
