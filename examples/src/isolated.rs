#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_time::{self as time};
use nrf52832_hal as _;
use panic_halt as _;

pub struct SysClock;
impl time::Clock for SysClock {
    type T = u32;
    const SCALING_FACTOR: time::fraction::Fraction = <time::fraction::Fraction>::new(1, 1_000_000);

    fn try_now(&self) -> Result<time::Instant<Self>, time::clock::Error> {
        Ok(time::Instant::new(23))
    }
}

#[entry]
fn main() -> ! {
    instant::run();
    duration::run();
    rate::run();

    loop {}
}

mod instant {
    use core::convert::TryInto;
    use embedded_time::{
        self as time,
        duration::{self, *},
        Instant,
    };

    pub fn run() {
        duration_since();
        duration_since_with_generic_type::<clock::Clock>(clock::Clock);
        duration_since_epoch();
    }

    mod clock {
        use embedded_time::Instant;
        use embedded_time::{self as time, fraction::Fraction};

        #[derive(Debug)]
        pub(crate) struct Clock;

        impl time::Clock for Clock {
            type T = u32;
            const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000);

            fn try_now(&self) -> Result<Instant<Self>, time::clock::Error> {
                static mut TICKS: u32 = 0;
                unsafe {
                    TICKS += 1;
                }
                Ok(Instant::new(unsafe { TICKS as Self::T }))
            }
        }
    }

    fn duration_since() {
        let diff = Instant::<clock::Clock>::new(5)
            .checked_duration_since(&Instant::<clock::Clock>::new(3));
        assert_eq!(
            diff,
            Some(duration::Generic::new(2_u32, Fraction::new(1, 1_000)))
        );

        let diff = Instant::<clock::Clock>::new(5)
            .checked_duration_since(&Instant::<clock::Clock>::new(6));
        assert_eq!(diff, None);
    }

    fn duration_since_with_generic_type<C: time::Clock>(clock: C) {
        let instant1 = clock.try_now().unwrap();
        let instant2 = clock.try_now().unwrap();
        let diff = instant2.checked_duration_since(&instant1).unwrap();

        let secs: Result<Seconds<C::T>, _> = diff.try_into();
        assert!(secs.unwrap() > Seconds(1));
    }

    fn duration_since_epoch() {
        assert_eq!(
            Instant::<clock::Clock>::new(u32::MAX).duration_since_epoch(),
            duration::Generic::from(Milliseconds(u32::MAX))
        );
    }
}

mod duration {
    use super::time::{duration, duration::*, rate::*, ConversionError};
    use core::convert::{TryFrom, TryInto};

    pub fn run() {
        comparison();
        construction();
        convert_to_rate();
        duration_scaling();
        get_generic_integer();
        add();
        sub();
        mul();
        div();
        remainder();
        to_generic();
        try_from_generic();
    }

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

    fn comparison() {
        // even though the value of 5 seconds cannot be expressed as Nanoseconds<u32>, it behaves as
        // expected.
        assert_ne!(Seconds(5_u32), Nanoseconds(u32::MAX));

        assert_ne!(Nanoseconds(u32::MAX), Seconds(5_u32));

        assert!(Seconds(5_u32) > Nanoseconds(u32::MAX));
        assert!(Nanoseconds(u32::MAX) < Seconds(5_u32));
    }

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
    }

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

    fn get_generic_integer() {
        let generic = duration::Generic::new(246_u32, Fraction::new(1, 2));
        assert_eq!(generic.integer(), 246_u32);
    }

    fn add() {
        assert_eq!(
            (Milliseconds(1_u32) + Seconds(1_u32)),
            Milliseconds(1_001_u32)
        );
    }

    fn sub() {
        assert_eq!(
            (Milliseconds(2_001_u32) - Seconds(1_u32)),
            Milliseconds(1_001_u32)
        );

        assert_eq!((Minutes(u32::MAX) - Hours(1_u32)), Minutes(u32::MAX - 60));
    }

    fn mul() {
        assert_eq!((Milliseconds(2_001_u32) * 2), Milliseconds(4_002_u32));

        assert_eq!(
            Milliseconds(2_001_u32).checked_mul(&2),
            Some(Milliseconds(4_002_u32))
        );

        assert_eq!(Milliseconds(u32::MAX).checked_mul(&2), None);
    }

    fn div() {
        assert_eq!((Milliseconds(2_002_u32) / 2), Milliseconds(1_001_u32));

        assert_eq!(
            Milliseconds(2_002_u32).checked_div(&2),
            Some(Milliseconds(1_001_u32))
        );

        assert_eq!(Milliseconds(u32::MAX).checked_div(&0), None);
    }

    fn remainder() {
        assert_eq!(Minutes(62_u32) % Hours(1_u32), Minutes(2_u32));
        assert_eq!(Minutes(62_u32) % Milliseconds(1_u32), Minutes(0_u32));
        assert_eq!(Minutes(62_u32) % Minutes(60_u32), Minutes(2_u32));
    }

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

    fn duration_scaling() {
        assert_eq!(1_u32.nanoseconds(), 1_u32.nanoseconds());
        assert_eq!(1_u32.microseconds(), 1_000_u32.nanoseconds());
        assert_eq!(1_u32.milliseconds(), 1_000_000_u32.nanoseconds());
        assert_eq!(1_u32.seconds(), 1_000_000_000_u32.nanoseconds());

        assert_eq!(1_000_u32.nanoseconds(), 1_u32.microseconds());
        assert_eq!(1_000_000_u32.nanoseconds(), 1_u32.milliseconds());
        assert_eq!(1_000_000_000_u32.nanoseconds(), 1_u32.seconds());
    }
}

mod rate {
    use super::time::{
        duration::*,
        rate::{self, *},
        ConversionError,
    };
    use core::convert::{TryFrom, TryInto};

    pub fn run() {
        try_from_generic();
        to_generic();
        add();
        sub();
        mul();
        div();
        remainder();
        get_generic_integer();
        comparison();
        baud_scaling();
        bits_per_second_scaling();
        bytes_per_second_scaling();
        convert_to_duration();
        frequency_scaling();
        try_into_generic_err();
        into_bigger();
    }

    fn comparison() {
        assert_ne!(2_001_u32.Hz(), 2_u32.kHz());

        assert_eq!(8_u32.Kibps(), 1_u32.KiBps());
    }

    fn try_from_generic() {
        assert_eq!(
            Hertz::try_from(rate::Generic::new(246_u32, Fraction::new(1, 2))),
            Ok(Hertz(123_u32))
        );
    }

    fn to_generic() {
        assert_eq!(
            Hertz(123_u32).to_generic(Fraction::new(1, 2)),
            Ok(rate::Generic::new(246_u32, Fraction::new(1, 2)))
        );
    }

    fn try_into_generic_err() {
        assert_eq!(
            Hertz(u32::MAX).to_generic::<u32>(Fraction::new(1, 2)),
            Err(ConversionError::Overflow)
        );
    }

    fn get_generic_integer() {
        let generic = rate::Generic::new(246_u32, Fraction::new(1, 2));
        assert_eq!(generic.integer(), 246_u32);
    }

    fn add() {
        assert_eq!((Kilohertz(1_u32) + Megahertz(1_u32)), Kilohertz(1_001_u32));
    }

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

    fn mul() {
        assert_eq!((Kilohertz(2_001_u32) * 2), Kilohertz(4_002_u32));

        assert_eq!(
            Kilohertz(2_001_u32).checked_mul(&2),
            Some(Kilohertz(4_002_u32))
        );

        assert_eq!(Kilohertz(u32::MAX).checked_mul(&2), None);
    }

    fn div() {
        assert_eq!((Kilohertz(2_002_u32) / 2), Kilohertz(1_001_u32));

        assert_eq!(
            Kilohertz(2_002_u32).checked_div(&2),
            Some(Kilohertz(1_001_u32))
        );

        assert_eq!(Kilohertz(u32::MAX).checked_div(&0), None);
    }

    fn remainder() {
        assert_eq!(Hertz(456_u32) % Hertz(100_u32), Hertz(56_u32));
        assert_eq!(Hertz(2_003_u32) % Kilohertz(1_u32), Hertz(3_u32));
        assert_eq!(Kilohertz(40_u32) % Hertz(100_u32), Kilohertz(0_u32));
    }

    fn convert_to_duration() {
        assert_eq!(Hertz(500_u32).to_duration(), Ok(Milliseconds(2_u32)));
        assert_eq!(Kilohertz(500_u32).to_duration(), Ok(Microseconds(2_u32)));
    }

    fn frequency_scaling() {
        assert_eq!(1_u32.Hz(), 1_u32.Hz());
        assert_eq!(1_u32.kHz(), 1_000_u32.Hz());
        assert_eq!(1_u32.MHz(), 1_000_000_u32.Hz());
    }

    fn bytes_per_second_scaling() {
        assert_eq!(1_u32.Bps(), 1_u32.Bps());
        assert_eq!(1_u32.kBps(), 1_000_u32.Bps());
        assert_eq!(1_u32.KiBps(), 1_024_u32.Bps());
        assert_eq!(1_u32.MBps(), 1_000_000_u32.Bps());
        assert_eq!(1_u32.MiBps(), 1_048_576_u32.Bps());
    }

    fn bits_per_second_scaling() {
        assert_eq!(1_u32.bps(), 1_u32.bps());
        assert_eq!(1_u32.kbps(), 1_000_u32.bps());
        assert_eq!(1_u32.Kibps(), 1_024_u32.bps());
        assert_eq!(1_u32.Mbps(), 1_000_000_u32.bps());
        assert_eq!(1_u32.Mibps(), 1_048_576_u32.bps());
    }

    fn baud_scaling() {
        assert_eq!(1_u32.Bd(), 1_u32.Bd());
        assert_eq!(1_u32.kBd(), 1_000_u32.Bd());
        assert_eq!(1_u32.KiBd(), 1_024_u32.Bd());
        assert_eq!(1_u32.MBd(), 1_000_000_u32.Bd());
        assert_eq!(1_u32.MiBd(), 1_048_576_u32.Bd());
    }

    fn into_bigger() {
        macro_rules! test_into_bigger {
            ($name:ident) => {
                // into same
                assert_eq!($name::<u32>::from($name(500_u32)), $name(500_u32));
                let rate: $name<u32> = $name(500_u32).into();
                assert_eq!(rate, $name(500_u32));

                assert_eq!($name::<u32>::try_from($name(500_u32)), Ok($name(500_u32)));
            };
            ($big:ident, $($small:ident),+) => {
                $(
                    // into bigger
                    assert_ne!(
                        $big::<u32>::from($small(u32::MAX)),
                        $big(1_u32)
                    );

                    let rate: $big<u32> = $small(u32::MAX).into();
                    assert_eq!(rate, $big(1_u32));

                    // into smaller
                    assert_eq!(
                        $small::<u32>::try_from($big(500 as u32)),
                        Ok($small((500)))
                    );

                    let rate: Result<$small<u32>, _> = $big(500 as u32).try_into();
                    assert_eq!(rate, Ok($small((500))));
                )+
                test_into_bigger!($big);
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
}
