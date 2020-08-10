#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_time::{self as time};
use nrf52832_hal as _;
use panic_halt as _;

pub struct SysClock;
impl time::Clock for SysClock {
    type T = u64;
    const SCALING_FACTOR: time::fraction::Fraction = <time::fraction::Fraction>::new(1, 1_000_000);

    fn try_now(&self) -> Result<time::Instant<Self>, time::clock::Error> {
        Ok(time::Instant::new(23 as Self::T))
    }
}

#[entry]
fn main() -> ! {
    duration::comparison();
    duration::construction();
    duration::convert_to_rate();
    duration::convert_from_core_duration();
    duration::convert_to_core_duration();
    duration::duration_scaling();
    duration::error_try_from();
    duration::get_generic_integer();
    duration::remainder();
    duration::to_generic();
    duration::try_from_generic();

    rate::try_from_generic();
    rate::to_generic();
    rate::remainder();
    rate::get_generic_integer();
    rate::comparison();
    rate::baud_scaling();
    rate::bits_per_second_scaling();
    rate::bytes_per_second_scaling();
    rate::convert_to_duration();
    rate::frequency_scaling();
    rate::try_into_generic_err();

    loop {}
}

mod duration {
    use super::time::{duration, duration::*, fraction::Fraction, rate::*, ConversionError};
    use core::convert::{TryFrom, TryInto};

    pub fn construction() {
        assert_eq!(<Seconds>::new(5), Seconds(5_u32));
        assert_eq!(Seconds::new(5_u32), Seconds(5_u32));

        assert_eq!(5_u32.nanoseconds(), Nanoseconds(5_u32));
        assert_eq!(5_u32.microseconds(), Microseconds(5_u32));
        assert_eq!(5_u32.milliseconds(), Milliseconds(5_u32));
        assert_eq!(5_u32.seconds(), Seconds(5_u32));
        assert_eq!(5_u32.minutes(), Minutes(5_u32));
        assert_eq!(5_u32.hours(), Hours(5_u32));
    }

    pub fn comparison() {
        // even though the value of 5 seconds cannot be expressed as Nanoseconds<u32>, it behaves as
        // expected.
        assert_ne!(Seconds(5_u32), Nanoseconds(u32::MAX));
        // assert_ne!(Seconds(5_u32), Nanoseconds(u32::MAX as u64));
        // assert_ne!(Seconds(5_u64), Nanoseconds(u32::MAX));
        // assert_ne!(Seconds(5_u64), Nanoseconds(u32::MAX as u64));

        assert_ne!(Nanoseconds(u32::MAX), Seconds(5_u32));
        // assert_ne!(Nanoseconds(u32::MAX as u64), Seconds(5_u32));
        // assert_ne!(Nanoseconds(u32::MAX), Seconds(5_u64));
        // assert_ne!(Nanoseconds(u32::MAX as u64), Seconds(5_u64));

        assert!(Seconds(5_u32) > Nanoseconds(u32::MAX));
        assert!(Nanoseconds(u32::MAX) < Seconds(5_u32));

        // assert!(Seconds(5_u32) < Nanoseconds(u64::MAX));
        // assert!(Nanoseconds(u64::MAX) > Seconds(5_u32));
        //
        // assert!(Seconds(5_u64) > Nanoseconds(u32::MAX));
        // assert!(Nanoseconds(u32::MAX) < Seconds(5_u64));
        //
        // assert!(Seconds(5_u64) < Nanoseconds(u64::MAX));
        // assert!(Nanoseconds(u64::MAX) > Seconds(5_u64));
    }

    pub fn try_from_generic() {
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
        // assert_eq!(
        //     Seconds::<u32>::try_from(duration::Generic::new(
        //         u32::MAX as u64 + 1,
        //         Fraction::new(1, 1)
        //     )),
        //     Err(ConversionError::ConversionFailure)
        // );
    }

    pub fn to_generic() {
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

    pub fn get_generic_integer() {
        let generic = duration::Generic::new(246_u32, Fraction::new(1, 2));
        assert_eq!(generic.integer(), &246_u32);
    }

    pub fn remainder() {
        assert_eq!(Minutes(62_u32) % Hours(1_u32), Minutes(2_u32));
        assert_eq!(Minutes(62_u32) % Milliseconds(1_u32), Minutes(0_u32));
        assert_eq!(Minutes(62_u32) % Minutes(60_u32), Minutes(2_u32));
    }

    pub fn convert_to_rate() {
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

    pub fn convert_from_core_duration() {
        let core_duration = core::time::Duration::from_nanos(5_025_678_901_234);
        // assert_eq!(
        //     core_duration.try_into(),
        //     Ok(Nanoseconds::<u64>(5_025_678_901_234))
        // );
        // assert_eq!(
        //     core_duration.try_into(),
        //     Ok(Microseconds::<u64>(5_025_678_901))
        // );
        assert_eq!(core_duration.try_into(), Ok(Milliseconds::<u32>(5_025_678)));
        assert_eq!(core_duration.try_into(), Ok(Seconds::<u32>(5_025)));
        assert_eq!(core_duration.try_into(), Ok(Minutes::<u32>(83)));
        assert_eq!(core_duration.try_into(), Ok(Hours::<u32>(1)));
    }

    pub fn convert_to_core_duration() {
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

    pub fn duration_scaling() {
        assert_eq!(1_u32.nanoseconds(), 1_u32.nanoseconds());
        assert_eq!(1_u32.microseconds(), 1_000_u32.nanoseconds());
        assert_eq!(1_u32.milliseconds(), 1_000_000_u32.nanoseconds());
        assert_eq!(1_u32.seconds(), 1_000_000_000_u32.nanoseconds());
        // assert_eq!(1_u64.minutes(), 60_000_000_000_u64.nanoseconds());
        // assert_eq!(1_u64.hours(), 3_600_000_000_000_u64.nanoseconds());

        assert_eq!(1_000_u32.nanoseconds(), 1_u32.microseconds());
        assert_eq!(1_000_000_u32.nanoseconds(), 1_u32.milliseconds());
        assert_eq!(1_000_000_000_u32.nanoseconds(), 1_u32.seconds());
        // assert_eq!(60_000_000_000_u64.nanoseconds(), 1_u64.minutes());
        // assert_eq!(3_600_000_000_000_u64.nanoseconds(), 1_u64.hours());
    }

    pub fn error_try_from() {
        // assert_eq!(
        //     Milliseconds::<u32>::try_from(Nanoseconds(u64::MAX)),
        //     Err(ConversionError::ConversionFailure)
        // );
        // assert_eq!(
        //     Milliseconds::<u32>::try_from(Seconds(u64::MAX)),
        //     Err(ConversionError::Overflow)
        // );
    }
}

mod rate {
    use super::time::{
        duration::{Microseconds, Milliseconds},
        fraction::Fraction,
        rate::{self, *},
        ConversionError,
    };
    use core::convert::TryFrom;

    pub fn comparison() {
        assert_ne!(2_001_u32.Hz(), 2_u32.kHz());
        // assert_ne!(2_001_u32.Hz(), 2_u64.kHz());
        // assert_ne!(2_001_u64.Hz(), 2_u32.kHz());
        // assert_ne!(2_001_u64.Hz(), 2_u64.kHz());

        assert!(5_u32.KiBps() > 5_u32.kBps());
        assert!(5_u32.KiBps() > 40_u32.kbps());
        assert_eq!(8_u32.Kibps(), 1_u32.KiBps());
    }

    pub fn try_from_generic() {
        assert_eq!(
            Hertz::try_from(rate::Generic::new(246_u32, Fraction::new(1, 2))),
            Ok(Hertz(123_u32))
        );
    }

    pub fn to_generic() {
        assert_eq!(
            Hertz(123_u32).to_generic(Fraction::new(1, 2)),
            Ok(rate::Generic::new(246_u32, Fraction::new(1, 2)))
        );
    }

    pub fn try_into_generic_err() {
        assert_eq!(
            Hertz(u32::MAX).to_generic::<u32>(Fraction::new(1, 2)),
            Err(ConversionError::Overflow)
        );
    }

    pub fn get_generic_integer() {
        let generic = rate::Generic::new(246_u32, Fraction::new(1, 2));
        assert_eq!(generic.integer(), &246_u32);
    }

    pub fn remainder() {
        assert_eq!(Hertz(456_u32) % Hertz(100_u32), Hertz(56_u32));
        assert_eq!(Hertz(2_003_u32) % Kilohertz(1_u32), Hertz(3_u32));
        assert_eq!(Kilohertz(40_u32) % Hertz(100_u32), Kilohertz(0_u32));
    }

    pub fn convert_to_duration() {
        assert_eq!(Hertz(500_u32).to_duration(), Ok(Milliseconds(2_u32)));
        assert_eq!(Kilohertz(500_u32).to_duration(), Ok(Microseconds(2_u32)));
    }

    pub fn frequency_scaling() {
        assert_eq!(1_u32.Hz(), 1_u32.Hz());
        assert_eq!(1_u32.kHz(), 1_000_u32.Hz());
        assert_eq!(1_u32.MHz(), 1_000_000_u32.Hz());
    }

    pub fn bytes_per_second_scaling() {
        assert_eq!(1_u32.Bps(), 1_u32.Bps());
        assert_eq!(1_u32.kBps(), 1_000_u32.Bps());
        assert_eq!(1_u32.KiBps(), 1_024_u32.Bps());
        assert_eq!(1_u32.MBps(), 1_000_000_u32.Bps());
        assert_eq!(1_u32.MiBps(), 1_048_576_u32.Bps());
    }

    pub fn bits_per_second_scaling() {
        assert_eq!(1_u32.bps(), 1_u32.bps());
        assert_eq!(1_u32.kbps(), 1_000_u32.bps());
        assert_eq!(1_u32.Kibps(), 1_024_u32.bps());
        assert_eq!(1_u32.Mbps(), 1_000_000_u32.bps());
        assert_eq!(1_u32.Mibps(), 1_048_576_u32.bps());
    }

    pub fn baud_scaling() {
        assert_eq!(1_u32.Bd(), 1_u32.Bd());
        assert_eq!(1_u32.kBd(), 1_000_u32.Bd());
        assert_eq!(1_u32.KiBd(), 1_024_u32.Bd());
        assert_eq!(1_u32.MBd(), 1_000_000_u32.Bd());
        assert_eq!(1_u32.MiBd(), 1_048_576_u32.Bd());
    }
}
