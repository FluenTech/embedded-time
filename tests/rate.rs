#![allow(non_snake_case)]

use core::convert::{TryFrom, TryInto};
use embedded_time::{
    duration::*,
    fraction::Fraction,
    rate::{self, *},
    ConversionError,
};

#[test]
fn construction() {
    assert_eq!(<Hertz>::new(5), Hertz(5_u32));
    assert_eq!(Hertz::new(5_u32), Hertz(5_u32));

    assert_eq!(5_u32.MiHz(), Mebihertz(5_u32));
    assert_eq!(5_u32.MHz(), Megahertz(5_u32));
    assert_eq!(5_u32.KiHz(), Kibihertz(5_u32));
    assert_eq!(5_u32.kHz(), Kilohertz(5_u32));
    assert_eq!(5_u32.Hz(), Hertz(5_u32));

    assert_eq!(5_u32.MiBps(), MebibytesPerSecond(5_u32));
    assert_eq!(5_u32.MBps(), MegabytesPerSecond(5_u32));
    assert_eq!(5_u32.KiBps(), KibibytesPerSecond(5_u32));
    assert_eq!(5_u32.kBps(), KilobytesPerSecond(5_u32));
    assert_eq!(5_u32.Bps(), BytesPerSecond(5_u32));

    assert_eq!(5_u32.Mibps(), MebibitsPerSecond(5_u32));
    assert_eq!(5_u32.Mbps(), MegabitsPerSecond(5_u32));
    assert_eq!(5_u32.Kibps(), KibibitsPerSecond(5_u32));
    assert_eq!(5_u32.kbps(), KilobitsPerSecond(5_u32));
    assert_eq!(5_u32.bps(), BitsPerSecond(5_u32));

    assert_eq!(5_u32.MiBd(), Mebibaud(5_u32));
    assert_eq!(5_u32.MBd(), Megabaud(5_u32));
    assert_eq!(5_u32.KiBd(), Kibibaud(5_u32));
    assert_eq!(5_u32.kBd(), Kilobaud(5_u32));
    assert_eq!(5_u32.Bd(), Baud(5_u32));
}

#[test]
fn comparisons() {
    assert_ne!(2_001.Hz(), 2_u32.kHz());

    assert_eq!(8_u32.Kibps(), 1_u32.KiBps());

    assert!(Kilohertz(5_u32) < Kilohertz(u64::MAX));
    assert!(Kilohertz(u64::MAX) > Kilohertz(5_u32));

    assert!(Hertz(5_u32) < Kilohertz(u64::MAX));
    assert!(Kilohertz(u64::MAX) > Hertz(5_u32));

    assert!(Kilohertz(5_u32) < Hertz(u64::MAX));
    assert!(Hertz(u64::MAX) > Kilohertz(5_u32));

    assert_ne!(Kilohertz(5_u32), Kilohertz(u64::MAX));
    assert_ne!(Kilohertz(u64::MAX), Kilohertz(5_u32));
}

#[test]
fn add() {
    assert_eq!((Kilohertz(1_u32) + Megahertz(1_u32)), Kilohertz(1_001_u32));
}

#[test]
fn sub() {
    assert_eq!(
        (Kilohertz(2_001_u32) - Megahertz(1_u32)),
        Kilohertz(1_001_u32)
    );

    assert_eq!(
        (Kibihertz(u32::MAX) - Mebihertz(1_u32)),
        Kibihertz(u32::MAX - 1_024)
    );
}

#[test]
fn mul() {
    assert_eq!((Kilohertz(2_001_u32) * 2), Kilohertz(4_002_u32));
}
#[test]
#[should_panic]
fn mul_overflow() {
    let _ = Kilohertz(u32::MAX) * 2;
}
#[test]
fn checked_mul() {
    assert_eq!(
        Kilohertz(2_001_u32).checked_mul(&2),
        Some(Kilohertz(4_002_u32))
    );

    assert_eq!(Kilohertz(u32::MAX).checked_mul(&2), None);
}

#[test]
fn div() {
    assert_eq!((Kilohertz(2_002_u32) / 2), Kilohertz(1_001_u32));
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
    assert_eq!(100_u32.bps() % u32::MAX.MiBps(), 100_u32.bps());
    assert_eq!(10_020_u32.Bps() % 1.kBps(), 20_u32.Bps());

    assert_eq!(Hertz(456_u32) % Hertz(100_u32), Hertz(56_u32));
    assert_eq!(Hertz(2_003_u32) % Kilohertz(1_u32), Hertz(3_u32));
    assert_eq!(Kilohertz(40_u32) % Hertz(100_u32), Kilohertz(0_u32));
}

#[test]
fn from_generic() {
    assert_eq!(
        Hertz::try_from(rate::Generic::new(246_u32, Fraction::new(1, 2))),
        Ok(Hertz(123_u32))
    );
}

#[test]
fn to_generic() {
    assert_eq!(
        Hertz(123_u32).to_generic(Fraction::new(1, 2)),
        Ok(rate::Generic::new(246_u32, Fraction::new(1, 2)))
    );

    // Overflow error
    assert_eq!(
        Hertz(u32::MAX).to_generic::<u32>(Fraction::new(1, 2)),
        Err(ConversionError::Unspecified)
    );

    // From named
    let generic: rate::Generic<u32> = 123_u32.Kibps().into();
    assert_eq!(
        generic,
        rate::Generic::new(123_u32, Fraction::new(1_024, 1))
    );
}

#[test]
fn get_generic_integer() {
    let generic = rate::Generic::new(246_u32, Fraction::new(1, 2));
    assert_eq!(generic.integer(), &246_u32);
}

#[test]
fn convert_to_duration() {
    assert_eq!(Hertz(500_u32).to_duration(), Ok(Milliseconds(2_u32)));
    assert_eq!(Kilohertz(500_u32).to_duration(), Ok(Microseconds(2_u32)));

    // Errors
    assert_eq!(
        Megahertz(u32::MAX).to_duration::<Hours<u32>>(),
        Err(ConversionError::Overflow)
    );
    assert_eq!(
        Hertz(0_u32).to_duration::<Seconds<u32>>(),
        Err(ConversionError::DivByZero)
    );
    assert_eq!(
        Hertz(0_u32).to_duration::<Seconds<u64>>(),
        Err(ConversionError::DivByZero)
    );
}

#[test]
fn frequency_scaling() {
    assert_eq!(1_u32.Hz(), 1_u32.Hz());
    assert_eq!(1_u32.kHz(), 1_000_u32.Hz());
    assert_eq!(1_u32.KiHz(), 1_024_u32.Hz());
    assert_eq!(1_u32.MHz(), 1_000_000_u32.Hz());
    assert_eq!(1_u32.MiHz(), 1_048_576_u32.Hz());
}

#[test]
fn bytes_per_second_scaling() {
    assert_eq!(1_u32.Bps(), 1_u32.Bps());
    assert_eq!(1_u32.kBps(), 1_000_u32.Bps());
    assert_eq!(1_u32.KiBps(), 1_024_u32.Bps());
    assert_eq!(1_u32.MBps(), 1_000_000_u32.Bps());
    assert_eq!(1_u32.MiBps(), 1_048_576_u32.Bps());
}

#[test]
fn bits_per_second_scaling() {
    assert_eq!(1_u32.bps(), 1_u32.bps());
    assert_eq!(1_u32.kbps(), 1_000_u32.bps());
    assert_eq!(1_u32.Kibps(), 1_024_u32.bps());
    assert_eq!(1_u32.Mbps(), 1_000_000_u32.bps());
    assert_eq!(1_u32.Mibps(), 1_048_576_u32.bps());

    assert_eq!(1_u32.Bps(), 8_u32.bps());
}

#[test]
fn baud_scaling() {
    assert_eq!(1_u32.Bd(), 1_u32.Bd());
    assert_eq!(1_u32.kBd(), 1_000_u32.Bd());
    assert_eq!(1_u32.KiBd(), 1_024_u32.Bd());
    assert_eq!(1_u32.MBd(), 1_000_000_u32.Bd());
    assert_eq!(1_u32.MiBd(), 1_048_576_u32.Bd());
}

#[test]
fn into_bigger() {
    macro_rules! test_into_bigger {
        ($into:ident) => {};
        ($into:ident, $($small:ident),+) => {
            $(
                assert_eq!(
                    $into::<u32>::from($small(u32::MAX)),
                    $into((u32::MAX as u64 * *$small::<u32>::SCALING_FACTOR.numerator() as u64 / *$into::<u32>::SCALING_FACTOR.numerator() as u64) as u32)
                );

                let rate: $into<u32> = $small(u32::MAX).into();
                assert_eq!(rate, $into((u32::MAX as u64 * *$small::<u32>::SCALING_FACTOR.numerator() as u64 / *$into::<u32>::SCALING_FACTOR.numerator() as u64) as u32));

                assert_eq!(
                    $into::<u64>::from($small(u32::MAX)),
                    $into((u32::MAX as u64 * *$small::<u32>::SCALING_FACTOR.numerator() as u64 / *$into::<u64>::SCALING_FACTOR.numerator() as u64) as u32)
                );

                let rate: $into<u64> = $small(u32::MAX).into();
                assert_eq!(rate, $into((u32::MAX as u64 * *$small::<u64>::SCALING_FACTOR.numerator() as u64 / *$into::<u64>::SCALING_FACTOR.numerator() as u64) as u32));

                assert_eq!(
                    $into::<u32>::try_from($small(u32::MAX as u64)),
                    Ok($into((u32::MAX as u64 * *$small::<u32>::SCALING_FACTOR.numerator() as u64 / *$into::<u32>::SCALING_FACTOR.numerator() as u64) as u32))
                );

                let rate: Result<$into<u32>, _> = $small(u32::MAX as u64).try_into();
                assert_eq!(rate, Ok($into((u32::MAX as u64 * *$small::<u64>::SCALING_FACTOR.numerator() as u64 / *$into::<u32>::SCALING_FACTOR.numerator() as u64) as u32)));
            )+

            test_into_bigger!($($small),+);
        };
    }
    test_into_bigger![Mebihertz, Kibihertz, Hertz];
    test_into_bigger![Megahertz, Kilohertz, Hertz];

    test_into_bigger![
        MebibytesPerSecond,
        MebibitsPerSecond,
        KibibytesPerSecond,
        KibibitsPerSecond,
        BytesPerSecond
    ];
    test_into_bigger![
        MegabytesPerSecond,
        MegabitsPerSecond,
        KilobytesPerSecond,
        KilobitsPerSecond,
        BytesPerSecond
    ];
    test_into_bigger![MebibytesPerSecond, BitsPerSecond];
    test_into_bigger![MebibitsPerSecond, BitsPerSecond];
    test_into_bigger![KibibytesPerSecond, BitsPerSecond];
    test_into_bigger![KibibitsPerSecond, BitsPerSecond];
    test_into_bigger![BytesPerSecond, BitsPerSecond];

    test_into_bigger![MegabytesPerSecond, BitsPerSecond];
    test_into_bigger![MegabitsPerSecond, BitsPerSecond];
    test_into_bigger![KilobytesPerSecond, BitsPerSecond];
    test_into_bigger![KilobitsPerSecond, BitsPerSecond];

    test_into_bigger![Mebibaud, Kibibaud, Baud];
    test_into_bigger![Megabaud, Kilobaud, Baud];
}

#[test]
fn into_same() {
    macro_rules! test_into_same {
        ($name:ident) => {
            assert_eq!($name::<u64>::from($name(500_u32)), $name(500_u64));
            let rate: $name<u64> = $name(500_u32).into();
            assert_eq!(rate, $name(500_u64));

            assert_eq!($name::<u32>::try_from($name(500_u64)), Ok($name(500_u32)));
        };
    }
    test_into_same![Mebihertz];
    test_into_same![Megahertz];
    test_into_same![Kibihertz];
    test_into_same![Kilohertz];
    test_into_same![Hertz];
    test_into_same![MebibytesPerSecond];
    test_into_same![MegabytesPerSecond];
    test_into_same![KibibytesPerSecond];
    test_into_same![KilobytesPerSecond];
    test_into_same![BytesPerSecond];
    test_into_same![MebibitsPerSecond];
    test_into_same![MegabitsPerSecond];
    test_into_same![KibibitsPerSecond];
    test_into_same![KilobitsPerSecond];
    test_into_same![BitsPerSecond];
    test_into_same![Mebibaud];
    test_into_same![Megabaud];
    test_into_same![Kibibaud];
    test_into_same![Kilobaud];
    test_into_same![Baud];
}

#[test]
fn into_smaller() {
    macro_rules! test_into_smaller {
        ($into:ident) => {};
        ($into:ident, $($big:ident),+) => {
            $(
                assert_eq!(
                    $into::<u64>::from($big(u32::MAX)),
                    $into(u32::MAX as u64 * *$big::<u32>::SCALING_FACTOR.numerator() as u64 / *$into::<u64>::SCALING_FACTOR.numerator() as u64)
                );

                let rate: $into<u64> = $big(u32::MAX).into();
                assert_eq!(rate, $into(u32::MAX as u64 * *$big::<u32>::SCALING_FACTOR.numerator() as u64 / *$into::<u64>::SCALING_FACTOR.numerator() as u64));

                // big<u64> to small<u32>
                assert_eq!(
                    $into::<u32>::try_from($big(500 as u64)),
                    Ok($into((500 as u64 * *$big::<u32>::SCALING_FACTOR.numerator() as u64 / *$into::<u32>::SCALING_FACTOR.numerator() as u64) as u32))
                );

                let rate: Result<$into<u32>, _> = $big(500 as u64).try_into();
                assert_eq!(rate, Ok($into((500 as u64 * *$big::<u64>::SCALING_FACTOR.numerator() as u64 / *$into::<u32>::SCALING_FACTOR.numerator() as u64) as u32)));

                // big<u64> to small<u64>
                assert_eq!(
                    $into::<u64>::try_from($big(500 as u64)),
                    Ok($into((500 as u64 * *$big::<u32>::SCALING_FACTOR.numerator() as u64 / *$into::<u32>::SCALING_FACTOR.numerator() as u64) as u64))
                );

                let rate: Result<$into<u64>, _> = $big(500 as u64).try_into();
                assert_eq!(rate, Ok($into((500 as u64 * *$big::<u64>::SCALING_FACTOR.numerator() as u64 / *$into::<u32>::SCALING_FACTOR.numerator() as u64) as u64)));

                // big<u32> to small<u32>
                assert_eq!(
                    $into::<u32>::try_from($big(500 as u32)),
                    Ok($into((500 as u64 * *$big::<u32>::SCALING_FACTOR.numerator() as u64 / *$into::<u32>::SCALING_FACTOR.numerator() as u64) as u32))
                );

                let rate: Result<$into<u32>, _> = $big(500 as u32).try_into();
                assert_eq!(rate, Ok($into((500 as u64 * *$big::<u64>::SCALING_FACTOR.numerator() as u64 / *$into::<u32>::SCALING_FACTOR.numerator() as u64) as u32)));
            )+

            test_into_smaller!($($big),+);
        };
    }
    test_into_smaller!(Hertz, Kilohertz, Megahertz);
    test_into_smaller!(Hertz, Kibihertz, Mebihertz);
    test_into_smaller![
        BitsPerSecond,
        BytesPerSecond,
        KilobitsPerSecond,
        KilobytesPerSecond,
        MegabitsPerSecond,
        MegabytesPerSecond
    ];
    test_into_smaller![
        BitsPerSecond,
        BytesPerSecond,
        KibibitsPerSecond,
        KibibytesPerSecond,
        MebibitsPerSecond,
        MebibytesPerSecond
    ];
    test_into_smaller![Baud, Kilobaud, Megabaud];
    test_into_smaller![Baud, Kibibaud, Mebibaud];
}
