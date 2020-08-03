use embedded_time::{self as time, duration, fraction::Fraction, ConversionError, Instant};

#[derive(Debug)]
struct Clock;

impl time::Clock for Clock {
    type T = u32;
    type ImplError = ();
    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000);

    fn try_now(&self) -> Result<Instant<Self>, time::clock::Error<Self::ImplError>> {
        unimplemented!()
    }
}

#[test]
fn duration_since() {
    let diff = Instant::<Clock>::new(5).checked_duration_since(&Instant::<Clock>::new(3));
    assert_eq!(
        diff,
        Ok(duration::Generic::new(2_u32, Fraction::new(1, 1_000)))
    );

    let diff = Instant::<Clock>::new(5).checked_duration_since(&Instant::<Clock>::new(6));
    assert_eq!(diff, Err(ConversionError::NegDuration));
}
