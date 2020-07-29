#![allow(non_snake_case)]

use core::convert::{TryFrom, TryInto};
use embedded_time::{duration::*, rate, rate::*, ConversionError, Fraction};

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
fn comparison() {
    assert_ne!(2_001_u32.Hz(), 2_u32.kHz());
    assert_ne!(2_001_u32.Hz(), 2_u64.kHz());
    assert_ne!(2_001_u64.Hz(), 2_u32.kHz());
    assert_ne!(2_001_u64.Hz(), 2_u64.kHz());

    assert!(5_u32.KiBps() > 5_u32.kBps());
    assert!(5_u32.KiBps() > 40_u32.kbps());
    assert_eq!(8_u32.Kibps(), 1_u32.KiBps());
}

#[test]
fn try_from_generic() {
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
}

#[test]
fn try_into_generic_err() {
    assert_eq!(
        Hertz(u32::MAX).to_generic::<u32>(Fraction::new(1, 2)),
        Err(ConversionError::Overflow)
    );
}

#[test]
fn get_generic_integer() {
    let generic = rate::Generic::new(246_u32, Fraction::new(1, 2));
    assert_eq!(generic.integer(), &246_u32);
}

#[test]
fn remainder() {
    assert_eq!(Hertz(456_u32) % Hertz(100_u32), Hertz(56_u32));
    assert_eq!(Hertz(2_003_u32) % Kilohertz(1_u32), Hertz(3_u32));
    assert_eq!(Kilohertz(40_u32) % Hertz(100_u32), Kilohertz(0_u32));
}

#[test]
fn convert_to_duration() {
    assert_eq!(Hertz(500_u32).to_duration(), Ok(Milliseconds(2_u32)));
    assert_eq!(Kilohertz(500_u32).to_duration(), Ok(Microseconds(2_u32)));
}

#[test]
fn frequency_scaling() {
    assert_eq!(1_u32.Hz(), 1_u32.Hz());
    assert_eq!(1_u32.kHz(), 1_000_u32.Hz());
    assert_eq!(1_u32.MHz(), 1_000_000_u32.Hz());
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
    test_into_bigger![Mebihertz, Megahertz, Kibihertz, Kilohertz, Hertz];
    test_into_bigger![
        MebibytesPerSecond,
        MegabytesPerSecond,
        MebibitsPerSecond,
        MegabitsPerSecond,
        KibibytesPerSecond,
        KilobytesPerSecond,
        KibibitsPerSecond,
        KilobitsPerSecond,
        BytesPerSecond,
        BitsPerSecond
    ];
    test_into_bigger![Mebibaud, Megabaud, Kibibaud, Kilobaud, Baud];
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
    test_widen_integer![Mebihertz];
    test_widen_integer![Megahertz];
    test_widen_integer![Kibihertz];
    test_widen_integer![Kilohertz];
    test_widen_integer![Hertz];
    test_widen_integer![MebibytesPerSecond];
    test_widen_integer![MegabytesPerSecond];
    test_widen_integer![KibibytesPerSecond];
    test_widen_integer![KilobytesPerSecond];
    test_widen_integer![BytesPerSecond];
    test_widen_integer![MebibitsPerSecond];
    test_widen_integer![MegabitsPerSecond];
    test_widen_integer![KibibitsPerSecond];
    test_widen_integer![KilobitsPerSecond];
    test_widen_integer![BitsPerSecond];
    test_widen_integer![Mebibaud];
    test_widen_integer![Megabaud];
    test_widen_integer![Kibibaud];
    test_widen_integer![Kilobaud];
    test_widen_integer![Baud];
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
    test_into_smaller!(Hertz, Kilohertz, Kibihertz, Megahertz, Mebihertz);
    test_into_smaller![
        BitsPerSecond,
        BytesPerSecond,
        KilobitsPerSecond,
        KibibitsPerSecond,
        KilobytesPerSecond,
        KibibytesPerSecond,
        MegabitsPerSecond,
        MebibitsPerSecond,
        MegabytesPerSecond,
        MebibytesPerSecond
    ];
    test_into_smaller![Baud, Kilobaud, Kibibaud, Megabaud, Mebibaud];
}
