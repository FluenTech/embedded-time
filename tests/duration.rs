use core::convert::{TryFrom, TryInto};
use embedded_time::duration::Generic;
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
fn comparisons() {
    assert_ne!(Seconds(5_u32), Nanoseconds(u32::MAX));
    assert_ne!(Seconds(5_u32), Nanoseconds(u32::MAX as u64));

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

    assert_ne!(Nanoseconds(1_u32), Nanoseconds(u64::MAX));
    assert_ne!(Nanoseconds(u64::MAX), Nanoseconds(1_u32));

    assert!(Seconds(5_u32) > Nanoseconds(u32::MAX));
    assert!(Nanoseconds(u32::MAX) < Seconds(5_u32));

    assert!(Seconds(5_u32) < Nanoseconds(u64::MAX));
    assert!(Nanoseconds(u64::MAX) > Seconds(5_u32));

    assert!(Seconds(5_u64) > Nanoseconds(u32::MAX));
    assert!(Nanoseconds(u32::MAX) < Seconds(5_u64));

    assert!(Seconds(5_u64) < Nanoseconds(u64::MAX));
    assert!(Nanoseconds(u64::MAX) > Seconds(5_u64));

    assert!(Microseconds(5_u32) < Microseconds(u64::MAX));
    assert!(Microseconds(u64::MAX) > Microseconds(5_u32));

    assert!(Generic::new(32_768_u64, Fraction::new(1, 32_768)) < Milliseconds(10_000_u32));
    assert!(Generic::new(32_768_u32, Fraction::new(1, 32_768)) < Milliseconds(10_000_u32));
    assert!(Generic::new(20 * 32_768_u32, Fraction::new(1, 32_768)) > Milliseconds(10_000_u32));
    assert!(
        Generic::new(1_000u32, Fraction::new(1, 1_000))
            == Generic::new(2_000u32, Fraction::new(1, 2_000))
    );
}

#[test]
fn add() {
    assert_eq!(
        (Milliseconds(1_u32) + Seconds(1_u32)),
        Milliseconds(1_001_u32)
    );

    assert_eq!(
        (Generic::new(1_010u32, Fraction::new(1, 1_000)) + Generic::new(1u32, Fraction::new(1, 1))),
        Generic::new(2_010u32, Fraction::new(1, 1_000))
    );
}

#[test]
fn sub() {
    assert_eq!(
        (Milliseconds(2_001_u32) - Seconds(1_u32)),
        Milliseconds(1_001_u32)
    );

    assert_eq!(Minutes(u32::MAX) - Hours(1_u32), Minutes(u32::MAX - 60));

    assert_eq!(
        (Generic::new(1_010u32, Fraction::new(1, 1_000)) - Generic::new(1u32, Fraction::new(1, 1))),
        Generic::new(10u32, Fraction::new(1, 1_000))
    );
}

#[test]
fn mul() {
    assert_eq!(Milliseconds(2_001_u32) * 2, Milliseconds(4_002_u32));
}
#[test]
#[should_panic]
fn mul_overflow() {
    let _ = Milliseconds(u32::MAX) * 2;
}
#[test]
fn checked_mul() {
    assert_eq!(
        Milliseconds(2_001_u32).checked_mul(&2),
        Some(Milliseconds(4_002_u32))
    );

    assert_eq!(Milliseconds(u32::MAX).checked_mul(&2), None);
}

#[test]
fn div() {
    assert_eq!((Milliseconds(2_002_u32) / 2), Milliseconds(1_001_u32));
}
#[test]
#[should_panic]
fn div_div_by_zero() {
    let _ = Milliseconds(u32::MAX) / 0;
}

#[test]
fn checked_div() {
    assert_eq!(
        Milliseconds(2_002_u32).checked_div(&2),
        Some(Milliseconds(1_001_u32))
    );

    assert_eq!(Milliseconds(u32::MAX).checked_div(&0), None);
}

#[test]
fn rem() {
    assert_eq!(100_u32.minutes() % u32::MAX.hours(), 100_u32.minutes());
    assert_eq!(100_u32.minutes() % 1.hours(), 40_u32.minutes());

    assert_eq!(Minutes(62_u32) % Hours(1_u32), Minutes(2_u32));
    assert_eq!(Minutes(62_u32) % Milliseconds(1_u32), Minutes(0_u32));
    assert_eq!(Minutes(62_u32) % Minutes(60_u32), Minutes(2_u32));
}

#[test]
fn from_generic() {
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
        Err(ConversionError::Unspecified)
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
        Err(ConversionError::Unspecified)
    );

    // From named
    let generic: duration::Generic<u32> = 246_u32.milliseconds().into();
    assert_eq!(
        generic,
        duration::Generic::new(246_u32, Fraction::new(1, 1_000))
    );
}

#[test]
fn get_generic_integer() {
    let generic = duration::Generic::new(246_u32, Fraction::new(1, 2));
    assert_eq!(generic.integer(), 246_u32);
}

#[test]
fn to_rate() {
    assert_eq!(Microseconds(500_u32).to_rate(), Ok(Kilohertz(2_u32)));
    assert_eq!(Microseconds(500_u32).to_rate(), Ok(Kilohertz(2_u64)));
    assert_eq!(Microseconds(500_u64).to_rate(), Ok(Kilohertz(2_u32)));
    assert_eq!(Microseconds(500_u64).to_rate(), Ok(Kilohertz(2_u64)));

    assert_eq!(Milliseconds(500_u32).to_rate(), Ok(Hertz(2_u32)));

    // Errors
    assert_eq!(
        Hours(u32::MAX).to_rate::<Megahertz<u32>>(),
        Err(ConversionError::Overflow)
    );
    assert_eq!(
        Seconds(0_u32).to_rate::<Hertz<u32>>(),
        Err(ConversionError::DivByZero)
    );
    assert_eq!(
        Seconds(0_u32).to_rate::<Hertz<u64>>(),
        Err(ConversionError::DivByZero)
    );
}

#[test]
fn from_core_duration() {
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

    // From
    let duration: Seconds<u64> = core_duration.into();
    assert_eq!(duration, Seconds(5_025_u64));
}

#[test]
fn into_core_duration() {
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

    assert_eq!(1_000_u32.nanoseconds(), 1_u32.microseconds());
    assert_eq!(1_000_000_u32.nanoseconds(), 1_u32.milliseconds());
    assert_eq!(1_000_000_000_u32.nanoseconds(), 1_u32.seconds());
}

#[test]
fn into_bigger() {
    macro_rules! test_into_bigger {
        ($into:ident) => {};
        ($into:ident, $($small:ident),+) => {

            assert_eq!(
                $into::<u32>::from($into(u32::MAX)),
                $into(u32::MAX)
            );

            let rate: $into<u32> = $into(u32::MAX).into();
            assert_eq!(rate, $into(u32::MAX));

            $(
                assert_eq!(
                    $into::<u32>::from($small(u32::MAX)),
                    $into((u32::MAX as u64
                    * $small::<u32>::SCALING_FACTOR.numerator() as u64
                    * $into::<u32>::SCALING_FACTOR.denominator() as u64
                    / $into::<u32>::SCALING_FACTOR.numerator() as u64
                    / $small::<u32>::SCALING_FACTOR.denominator() as u64
                    ) as u32)
                );

                let rate: $into<u32> = $small(u32::MAX).into();
                assert_eq!(rate,
                    $into((u32::MAX as u64
                    * $small::<u32>::SCALING_FACTOR.numerator() as u64
                    * $into::<u32>::SCALING_FACTOR.denominator() as u64
                    / $into::<u32>::SCALING_FACTOR.numerator() as u64
                    / $small::<u32>::SCALING_FACTOR.denominator() as u64
                    ) as u32)
                );

                assert_eq!(
                    $into::<u64>::from($small(u32::MAX)),
                    $into((u32::MAX as u64
                    * $small::<u32>::SCALING_FACTOR.numerator() as u64
                    * $into::<u64>::SCALING_FACTOR.denominator() as u64
                    / $into::<u64>::SCALING_FACTOR.numerator() as u64
                    / $small::<u32>::SCALING_FACTOR.denominator() as u64
                    ) as u64)
                );

                let rate: $into<u64> = $small(u32::MAX).into();
                assert_eq!(rate,
                    $into((u32::MAX as u64
                    * $small::<u32>::SCALING_FACTOR.numerator() as u64
                    * $into::<u64>::SCALING_FACTOR.denominator() as u64
                    / $into::<u64>::SCALING_FACTOR.numerator() as u64
                    / $small::<u32>::SCALING_FACTOR.denominator() as u64
                    ) as u64)
                );

                assert_eq!(
                    $into::<u32>::try_from($small(u32::MAX as u64)),
                    Ok(
                        $into((u32::MAX as u64
                        * $small::<u64>::SCALING_FACTOR.numerator() as u64
                        * $into::<u32>::SCALING_FACTOR.denominator() as u64
                        / $into::<u32>::SCALING_FACTOR.numerator() as u64
                        / $small::<u64>::SCALING_FACTOR.denominator() as u64
                        ) as u32)
                    )
                );

                let rate: Result<$into<u32>, _> = $small(u32::MAX as u64).try_into();
                assert_eq!(
                    rate,
                    Ok(
                        $into((u32::MAX as u64
                        * $small::<u64>::SCALING_FACTOR.numerator() as u64
                        * $into::<u32>::SCALING_FACTOR.denominator() as u64
                        / $into::<u32>::SCALING_FACTOR.numerator() as u64
                        / $small::<u64>::SCALING_FACTOR.denominator() as u64
                        ) as u32)
                    )
                );
            )+

            test_into_bigger!($($small),+);
        };
    }
    test_into_bigger![
        Hours,
        Minutes,
        Seconds,
        Milliseconds,
        Microseconds,
        Nanoseconds
    ];
}

#[test]
fn widen_integer() {
    macro_rules! test_widen_integer {
        ($name:ident) => {
            assert_eq!($name::<u64>::from($name(500_u32)), $name(500_u64));
            let rate: $name<u64> = $name(500_u32).into();
            assert_eq!(rate, $name(500_u64));
        };
    }
    test_widen_integer![Hours];
    test_widen_integer![Minutes];
    test_widen_integer![Seconds];
    test_widen_integer![Milliseconds];
    test_widen_integer![Microseconds];
    test_widen_integer![Nanoseconds];
}

#[test]
fn into_smaller() {
    macro_rules! test_into_smaller {
        ($into:ident) => {};
        ($into:ident, $($big:ident),+) => {

            assert_eq!(
                $into::<u32>::from($into(u32::MAX)),
                $into(u32::MAX)
            );

            let rate: $into<u32> = $into(u32::MAX).into();
            assert_eq!(rate, $into(u32::MAX));

            $(
                assert_eq!(
                    $into::<u64>::from($big(u32::MAX)),
                    $into((u32::MAX as u64
                    * $big::<u32>::SCALING_FACTOR.numerator() as u64
                    * $into::<u64>::SCALING_FACTOR.denominator() as u64
                    / $into::<u64>::SCALING_FACTOR.numerator() as u64
                    / $big::<u32>::SCALING_FACTOR.denominator() as u64
                    ) as u64)
                );

                let rate: $into<u64> = $big(u32::MAX).into();
                assert_eq!(rate,
                    $into((u32::MAX as u64
                    * $big::<u32>::SCALING_FACTOR.numerator() as u64
                    * $into::<u64>::SCALING_FACTOR.denominator() as u64
                    / $into::<u64>::SCALING_FACTOR.numerator() as u64
                    / $big::<u32>::SCALING_FACTOR.denominator() as u64
                    ) as u64)
                );

                assert_eq!(
                    $into::<u32>::try_from($big(4 as u32)),
                    Ok(
                        $into((4 as u64
                        * $big::<u32>::SCALING_FACTOR.numerator() as u64
                        * $into::<u32>::SCALING_FACTOR.denominator() as u64
                        / $into::<u32>::SCALING_FACTOR.numerator() as u64
                        / $big::<u32>::SCALING_FACTOR.denominator() as u64
                        ) as u32)
                    )
                );

                let rate: Result<$into<u32>, _> = $big(4 as u32).try_into();
                assert_eq!(
                    rate,
                    Ok(
                        $into((4 as u64
                        * $big::<u32>::SCALING_FACTOR.numerator() as u64
                        * $into::<u32>::SCALING_FACTOR.denominator() as u64
                        / $into::<u32>::SCALING_FACTOR.numerator() as u64
                        / $big::<u32>::SCALING_FACTOR.denominator() as u64
                        ) as u32)
                    )
                );

                assert_eq!(
                    $into::<u32>::try_from($big(4 as u64)),
                    Ok(
                        $into((4 as u64
                        * $big::<u64>::SCALING_FACTOR.numerator() as u64
                        * $into::<u32>::SCALING_FACTOR.denominator() as u64
                        / $into::<u32>::SCALING_FACTOR.numerator() as u64
                        / $big::<u64>::SCALING_FACTOR.denominator() as u64
                        ) as u32)
                    )
                );

                let rate: Result<$into<u32>, _> = $big(4 as u64).try_into();
                assert_eq!(
                    rate,
                    Ok(
                        $into((4 as u64
                        * $big::<u64>::SCALING_FACTOR.numerator() as u64
                        * $into::<u32>::SCALING_FACTOR.denominator() as u64
                        / $into::<u32>::SCALING_FACTOR.numerator() as u64
                        / $big::<u64>::SCALING_FACTOR.denominator() as u64
                        ) as u32)
                    )
                );

                assert_eq!(
                    $into::<u64>::try_from($big(4 as u64)),
                    Ok(
                        $into((4 as u64
                        * $big::<u64>::SCALING_FACTOR.numerator() as u64
                        * $into::<u64>::SCALING_FACTOR.denominator() as u64
                        / $into::<u64>::SCALING_FACTOR.numerator() as u64
                        / $big::<u64>::SCALING_FACTOR.denominator() as u64
                        ) as u64)
                    )
                );

                let rate: Result<$into<u64>, _> = $big(4 as u64).try_into();
                assert_eq!(
                    rate,
                    Ok(
                        $into((4 as u64
                        * $big::<u64>::SCALING_FACTOR.numerator() as u64
                        * $into::<u64>::SCALING_FACTOR.denominator() as u64
                        / $into::<u64>::SCALING_FACTOR.numerator() as u64
                        / $big::<u64>::SCALING_FACTOR.denominator() as u64
                        ) as u64)
                    )
                );
            )+

            test_into_smaller!($($big),+);
        };
    }
    test_into_smaller![Milliseconds, Seconds, Minutes, Hours];
    test_into_smaller![Nanoseconds, Microseconds, Milliseconds, Seconds];
}

#[test]
fn error_try_from() {
    assert_eq!(
        Milliseconds::<u32>::try_from(Nanoseconds(u64::MAX)),
        Err(ConversionError::ConversionFailure)
    );
    assert_eq!(
        Milliseconds::<u32>::try_from(Seconds(u64::MAX)),
        Err(ConversionError::Unspecified)
    );
}
