use core::convert::{TryFrom, TryInto};
use embedded_time::{duration, duration::*, fraction::Fraction, rate::*, ConversionError};

#[test]
fn construction() {
    assert_eq!(<Seconds>::new(5), Seconds(5_u32));
    assert_eq!(Seconds::new(5_u32), Seconds(5_u32));

    assert_eq!(5_u32.nanoseconds(), Nanoseconds(5_u32));
    assert_eq!(5_u32.microseconds(), Microseconds(5_u32));
    assert_eq!(5_u32.milliseconds(), Milliseconds(5_u32));
    assert_eq!(5_u32.seconds(), Seconds(5_u32));
    assert_eq!(5_u32.minutes(), Minutes(5_u32));
    assert_eq!(5_u32.hours(), Hours(5_u32));
}

#[test]
fn comparison() {
    // even though the value of 5 seconds cannot be expressed as Nanoseconds<u32>, it behaves as
    // expected.
    assert_ne!(Seconds(5_u32), Nanoseconds(u32::MAX));
    assert_ne!(Seconds(5_u32), Nanoseconds(u32::MAX as u64));
    assert_ne!(Seconds(5_u64), Nanoseconds(u32::MAX));
    assert_ne!(Seconds(5_u64), Nanoseconds(u32::MAX as u64));

    assert_ne!(Nanoseconds(u32::MAX), Seconds(5_u32));
    assert_ne!(Nanoseconds(u32::MAX as u64), Seconds(5_u32));
    assert_ne!(Nanoseconds(u32::MAX), Seconds(5_u64));
    assert_ne!(Nanoseconds(u32::MAX as u64), Seconds(5_u64));

    assert!(Seconds(5_u32) > Nanoseconds(u32::MAX));
    assert!(Nanoseconds(u32::MAX) < Seconds(5_u32));

    assert!(Seconds(5_u32) < Nanoseconds(u64::MAX));
    assert!(Nanoseconds(u64::MAX) > Seconds(5_u32));

    assert!(Seconds(5_u64) > Nanoseconds(u32::MAX));
    assert!(Nanoseconds(u32::MAX) < Seconds(5_u64));

    assert!(Seconds(5_u64) < Nanoseconds(u64::MAX));
    assert!(Nanoseconds(u64::MAX) > Seconds(5_u64));
}

#[test]
fn try_from_generic() {
    assert_eq!(
        Seconds::try_from(duration::Generic::new(246_u32, Fraction::new(1, 2))),
        Ok(Seconds(123_u32))
    );

    let seconds: Result<Seconds<u32>, _> =
        duration::Generic::new(246_u32, Fraction::new(1, 2)).try_into();
    assert_eq!(seconds, Ok(Seconds(123_u32)));

    // Overflow
    assert_eq!(
        Seconds::<u32>::try_from(duration::Generic::new(u32::MAX, Fraction::new(10, 1))),
        Err(ConversionError::Overflow)
    );

    // ConversionFailure (type)
    assert_eq!(
        Seconds::<u32>::try_from(duration::Generic::new(
            u32::MAX as u64 + 1,
            Fraction::new(1, 1)
        )),
        Err(ConversionError::ConversionFailure)
    );
}

#[test]
fn to_generic() {
    assert_eq!(
        Seconds(123_u32).to_generic(Fraction::new(1, 2)),
        Ok(duration::Generic::new(246_u32, Fraction::new(1, 2)))
    );

    // Overflow error
    assert_eq!(
        Seconds(u32::MAX).to_generic::<u32>(Fraction::new(1, 2)),
        Err(ConversionError::Overflow)
    );
}

#[test]
fn get_generic_integer() {
    let generic = duration::Generic::new(246_u32, Fraction::new(1, 2));
    assert_eq!(generic.integer(), &246_u32);
}

#[test]
fn remainder() {
    assert_eq!(Minutes(62_u32) % Hours(1_u32), Minutes(2_u32));
    assert_eq!(Minutes(62_u32) % Milliseconds(1_u32), Minutes(0_u32));
    assert_eq!(Minutes(62_u32) % Minutes(60_u32), Minutes(2_u32));
}

#[test]
fn convert_to_rate() {
    assert_eq!(Milliseconds(500_u32).to_rate(), Ok(Hertz(2_u32)));

    assert_eq!(Microseconds(500_u32).to_rate(), Ok(Kilohertz(2_u32)));

    // Errors
    assert_eq!(
        Hours(u32::MAX).to_rate::<Megahertz<u32>>(),
        Err(ConversionError::Overflow)
    );
    assert_eq!(
        Seconds(0_u32).to_rate::<Hertz<u32>>(),
        Err(ConversionError::DivByZero)
    );
}

#[test]
fn convert_from_core_duration() {
    let core_duration = core::time::Duration::from_nanos(5_025_678_901_234);
    assert_eq!(
        core_duration.try_into(),
        Ok(Nanoseconds::<u64>(5_025_678_901_234))
    );
    assert_eq!(
        core_duration.try_into(),
        Ok(Microseconds::<u64>(5_025_678_901))
    );
    assert_eq!(core_duration.try_into(), Ok(Milliseconds::<u32>(5_025_678)));
    assert_eq!(core_duration.try_into(), Ok(Seconds::<u32>(5_025)));
    assert_eq!(core_duration.try_into(), Ok(Minutes::<u32>(83)));
    assert_eq!(core_duration.try_into(), Ok(Hours::<u32>(1)));
}

#[test]
fn convert_to_core_duration() {
    assert_eq!(
        Nanoseconds(123_u32).try_into(),
        Ok(core::time::Duration::from_nanos(123))
    );
    assert_eq!(
        Microseconds(123_u32).try_into(),
        Ok(core::time::Duration::from_micros(123))
    );
    assert_eq!(
        Milliseconds(123_u32).try_into(),
        Ok(core::time::Duration::from_millis(123))
    );
    assert_eq!(
        Seconds(123_u32).try_into(),
        Ok(core::time::Duration::from_secs(123))
    );
    assert_eq!(
        Minutes(123_u32).try_into(),
        Ok(core::time::Duration::from_secs(123 * 60))
    );
    assert_eq!(
        Hours(123_u32).try_into(),
        Ok(core::time::Duration::from_secs(123 * 3600))
    );
}

#[test]
fn duration_scaling() {
    assert_eq!(1_u32.nanoseconds(), 1_u32.nanoseconds());
    assert_eq!(1_u32.microseconds(), 1_000_u32.nanoseconds());
    assert_eq!(1_u32.milliseconds(), 1_000_000_u32.nanoseconds());
    assert_eq!(1_u32.seconds(), 1_000_000_000_u32.nanoseconds());
    assert_eq!(1_u64.minutes(), 60_000_000_000_u64.nanoseconds());
    assert_eq!(1_u64.hours(), 3_600_000_000_000_u64.nanoseconds());

    assert_eq!(1_000_u32.nanoseconds(), 1_u32.microseconds());
    assert_eq!(1_000_000_u32.nanoseconds(), 1_u32.milliseconds());
    assert_eq!(1_000_000_000_u32.nanoseconds(), 1_u32.seconds());
    assert_eq!(60_000_000_000_u64.nanoseconds(), 1_u64.minutes());
    assert_eq!(3_600_000_000_000_u64.nanoseconds(), 1_u64.hours());
}

mod into_bigger {
    use super::*;

    #[test]
    fn into_hours() {
        // From
        assert_eq!(Hours::<u32>::from(Minutes(u32::MAX)), Hours(u32::MAX / 60));
        assert_eq!(Hours::<u32>::from(Seconds(3_601_u32)), Hours(1_u32));
        assert_eq!(
            Hours::<u32>::from(Milliseconds(3_600_001_u32)),
            Hours(1_u32)
        );
        assert_eq!(
            Hours::<u32>::from(Microseconds(3_600_000_001_u32)),
            Hours(1_u32)
        );
        assert_eq!(
            Hours::<u64>::from(Nanoseconds(3_600_000_000_001_u64)),
            Hours(1_u64)
        );

        // Into
        let hours: Hours<u32> = Minutes(62_u32).into();
        assert_eq!(hours, Hours(1_u32));

        let hours: Hours<u32> = Seconds(3_601_u32).into();
        assert_eq!(hours, Hours(1_u32));

        let hours: Hours<u32> = Milliseconds(3_600_001_u32).into();
        assert_eq!(hours, Hours(1_u32));

        let hours: Hours<u32> = Microseconds(3_600_000_001_u32).into();
        assert_eq!(hours, Hours(1_u32));

        let hours: Hours<u64> = Nanoseconds(3_600_000_000_001_u64).into();
        assert_eq!(hours, Hours(1_u64));
    }

    #[test]
    fn into_minutes() {
        // From
        assert_eq!(Minutes::<u32>::from(Seconds(3_601_u32)), Minutes(60_u32));
        assert_eq!(
            Minutes::<u32>::from(Milliseconds(3_600_001_u32)),
            Minutes(60_u32)
        );
        assert_eq!(
            Minutes::<u32>::from(Microseconds(3_600_000_001_u32)),
            Minutes(60_u32)
        );
        assert_eq!(
            Minutes::<u64>::from(Nanoseconds(3_600_000_000_001_u64)),
            Minutes(60_u64)
        );

        // Into
        let minutes: Minutes<u32> = Seconds(3_601_u32).into();
        assert_eq!(minutes, Minutes(60_u32));

        let minutes: Minutes<u32> = Milliseconds(3_600_001_u32).into();
        assert_eq!(minutes, Minutes(60_u32));

        let minutes: Minutes<u32> = Microseconds(3_600_000_001_u32).into();
        assert_eq!(minutes, Minutes(60_u32));

        let minutes: Minutes<u64> = Nanoseconds(3_600_000_000_001_u64).into();
        assert_eq!(minutes, Minutes(60_u32));
    }

    #[test]
    fn into_seconds() {
        // From
        assert_eq!(
            Seconds::<u32>::from(Milliseconds(3_600_001_u32)),
            Seconds(3_600_u32)
        );
        assert_eq!(
            Seconds::<u32>::from(Microseconds(3_600_000_001_u32)),
            Seconds(3_600_u32)
        );
        assert_eq!(
            Seconds::<u64>::from(Nanoseconds(3_600_000_000_001_u64)),
            Seconds(3_600_u32)
        );

        // Into
        let seconds: Seconds<u32> = Milliseconds(3_600_001_u32).into();
        assert_eq!(seconds, Seconds(3_600_u32));

        let seconds: Seconds<u32> = Microseconds(3_600_000_001_u32).into();
        assert_eq!(seconds, Seconds(3_600_u32));

        let seconds: Seconds<u64> = Nanoseconds(3_600_000_000_001_u64).into();
        assert_eq!(seconds, Seconds(3_600_u32));
    }

    #[test]
    fn into_milliseconds() {
        // From
        assert_eq!(
            Milliseconds::<u32>::from(Microseconds(3_600_000_001_u32)),
            Milliseconds(3_600_000_u32)
        );
        assert_eq!(
            Milliseconds::<u64>::from(Nanoseconds(3_600_000_000_001_u64)),
            Milliseconds(3_600_000_u32)
        );

        // Into
        let milliseconds: Milliseconds<u32> = Microseconds(3_600_000_001_u32).into();
        assert_eq!(milliseconds, Milliseconds(3_600_000_u32));

        let milliseconds: Milliseconds<u64> = Nanoseconds(3_600_000_000_001_u64).into();
        assert_eq!(milliseconds, Milliseconds(3_600_000_u32));
    }

    #[test]
    fn into_microseconds() {
        // From
        assert_eq!(
            Microseconds::<u64>::from(Nanoseconds(3_600_000_000_001_u64)),
            Microseconds(3_600_000_000_u32)
        );

        // Into
        let microseconds: Microseconds<u64> = Nanoseconds(3_600_000_000_001_u64).into();
        assert_eq!(microseconds, Microseconds(3_600_000_000_u32));
    }
}

#[test]
fn widen_integer() {
    assert_eq!(Hours::<u64>::from(Hours(500_u32)), Hours(500_u64));

    let hours: Hours<u64> = Hours(500_u32).into();
    assert_eq!(hours, Hours(500_u64));

    assert_eq!(Minutes::<u64>::from(Minutes(500_u32)), Minutes(500_u64));

    let minutes: Minutes<u64> = Minutes(500_u32).into();
    assert_eq!(minutes, Minutes(500_u64));

    assert_eq!(Seconds::<u64>::from(Seconds(500_u32)), Seconds(500_u64));

    let seconds: Seconds<u64> = Seconds(500_u32).into();
    assert_eq!(seconds, Seconds(500_u64));

    assert_eq!(
        Milliseconds::<u64>::from(Milliseconds(500_u32)),
        Milliseconds(500_u64)
    );

    let milliseconds: Milliseconds<u64> = Milliseconds(500_u32).into();
    assert_eq!(milliseconds, Milliseconds(500_u64));

    assert_eq!(
        Microseconds::<u64>::from(Microseconds(500_u32)),
        Microseconds(500_u64)
    );

    let microseconds: Microseconds<u64> = Microseconds(500_u32).into();
    assert_eq!(microseconds, Microseconds(500_u64));

    assert_eq!(
        Nanoseconds::<u64>::from(Nanoseconds(500_u32)),
        Nanoseconds(500_u64)
    );

    let nanoseconds: Nanoseconds<u64> = Nanoseconds(500_u32).into();
    assert_eq!(nanoseconds, Nanoseconds(500_u64));
}

mod into_smaller {
    use super::*;

    #[test]
    fn into_minutes() {
        // From
        assert_eq!(Minutes::<u64>::from(Hours(1_u32)), Minutes(60_u64));

        // Into
        let minutes: Minutes<u64> = Hours(1_u32).into();
        assert_eq!(minutes, Minutes(60_u64));
    }

    #[test]
    fn into_seconds() {
        // From
        assert_eq!(Seconds::<u64>::from(Minutes(60_u32)), Seconds(3_600_u64));
        assert_eq!(Seconds::<u64>::from(Hours(1_u32)), Seconds(3_600_u64));

        // Into
        let seconds: Seconds<u64> = Hours(1_u32).into();
        assert_eq!(seconds, Seconds(3_600_u64));

        let seconds: Seconds<u64> = Minutes(60_u32).into();
        assert_eq!(seconds, Seconds(3_600_u64));
    }

    #[test]
    fn into_milliseconds() {
        // From
        assert_eq!(
            Milliseconds::<u64>::from(Seconds(3_600_u32)),
            Milliseconds(3_600_000_u64)
        );
        assert_eq!(
            Milliseconds::<u64>::from(Minutes(60_u32)),
            Milliseconds(3_600_000_u64)
        );
        assert_eq!(
            Milliseconds::<u64>::from(Hours(1_u32)),
            Milliseconds(3_600_000_u64)
        );

        // Into
        let milliseconds: Milliseconds<u64> = Hours(1_u32).into();
        assert_eq!(milliseconds, Milliseconds(3_600_000_u64));

        let milliseconds: Milliseconds<u64> = Minutes(60_u32).into();
        assert_eq!(milliseconds, Milliseconds(3_600_000_u64));

        let milliseconds: Milliseconds<u64> = Seconds(3_600_u32).into();
        assert_eq!(milliseconds, Milliseconds(3_600_000_u64));
    }

    #[test]
    fn into_microseconds() {
        // From
        assert_eq!(
            Microseconds::<u64>::from(Milliseconds(3_600_000_u32)),
            Microseconds(3_600_000_000_u64)
        );
        assert_eq!(
            Microseconds::<u64>::from(Seconds(3_600_u32)),
            Microseconds(3_600_000_000_u64)
        );
        assert_eq!(
            Microseconds::<u64>::from(Minutes(60_u32)),
            Microseconds(3_600_000_000_u64)
        );
        assert_eq!(
            Microseconds::<u64>::from(Hours(1_u32)),
            Microseconds(3_600_000_000_u64)
        );

        // Into
        let microseconds: Microseconds<u64> = Hours(1_u32).into();
        assert_eq!(microseconds, Microseconds(3_600_000_000_u64));

        let microseconds: Microseconds<u64> = Minutes(60_u32).into();
        assert_eq!(microseconds, Microseconds(3_600_000_000_u64));

        let microseconds: Microseconds<u64> = Seconds(3_600_u32).into();
        assert_eq!(microseconds, Microseconds(3_600_000_000_u64));

        let microseconds: Microseconds<u64> = Milliseconds(3_600_000_u32).into();
        assert_eq!(microseconds, Microseconds(3_600_000_000_u64));
    }

    #[test]
    fn into_nanoseconds() {
        // From
        assert_eq!(
            Nanoseconds::<u64>::from(Microseconds(1_000_000_u32)),
            Nanoseconds(1_000_000_000_u64)
        );
        assert_eq!(
            Nanoseconds::<u64>::from(Milliseconds(1_000_u32)),
            Nanoseconds(1_000_000_000_u64)
        );
        assert_eq!(
            Nanoseconds::<u64>::from(Seconds(1_u32)),
            Nanoseconds(1_000_000_000_u64)
        );
        assert_eq!(
            Nanoseconds::<u64>::from(Minutes(1_u32)),
            Nanoseconds(60_000_000_000_u64)
        );
        assert_eq!(
            Nanoseconds::<u64>::from(Hours(1_u32)),
            Nanoseconds(3_600_000_000_000_u64)
        );

        // Into
        let nanoseconds: Nanoseconds<u64> = Hours(1_u32).into();
        assert_eq!(nanoseconds, Nanoseconds(3_600_000_000_000_u64));

        let nanoseconds: Nanoseconds<u64> = Minutes(60_u32).into();
        assert_eq!(nanoseconds, Nanoseconds(3_600_000_000_000_u64));

        let nanoseconds: Nanoseconds<u64> = Seconds(3_600_u32).into();
        assert_eq!(nanoseconds, Nanoseconds(3_600_000_000_000_u64));

        let nanoseconds: Nanoseconds<u64> = Milliseconds(3_600_000_u32).into();
        assert_eq!(nanoseconds, Nanoseconds(3_600_000_000_000_u64));

        let nanoseconds: Nanoseconds<u64> = Microseconds(3_600_000_000_u32).into();
        assert_eq!(nanoseconds, Nanoseconds(3_600_000_000_000_u64));
    }
}

#[test]
fn error_try_from() {
    assert_eq!(
        Milliseconds::<u32>::try_from(Nanoseconds(u64::MAX)),
        Err(ConversionError::ConversionFailure)
    );
    assert_eq!(
        Milliseconds::<u32>::try_from(Seconds(u64::MAX)),
        Err(ConversionError::Overflow)
    );
}
