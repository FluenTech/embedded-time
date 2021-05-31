use crossbeam_utils::thread;
use embedded_time::{
    self as time, duration::*, fixed_point, fraction::Fraction, Clock as _, Instant,
};
use std::sync::atomic::{AtomicU64, Ordering};

static TICKS: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct Clock;
impl time::Clock for Clock {
    type T = u64;
    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000);

    fn try_now(&self) -> Result<Instant<Self>, time::clock::Error> {
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
    Dur: fixed_point::FixedPoint,
    u64: From<Dur::T>,
{
    let ticks = TICKS.load(Ordering::SeqCst);
    let ticks = ticks
        + duration
            .to_generic::<u64>(Clock::SCALING_FACTOR)
            .unwrap()
            .integer();
    TICKS.store(ticks, Ordering::SeqCst);
}
