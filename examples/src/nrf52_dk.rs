#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use cortex_m_rt::entry;
use embedded_time::{self as time};
use panic_rtt as _;
use time::{duration::*, Clock as _};

pub mod nrf52 {
    pub use nrf52832_hal::{
        gpio,
        pac::{self, Peripherals},
        prelude::*,
    };
}

pub struct SysClock {
    low: nrf52::pac::TIMER0,
    high: nrf52::pac::TIMER1,
    capture_task: nrf52::pac::EGU0,
}

impl SysClock {
    pub fn take(
        low: nrf52::pac::TIMER0,
        high: nrf52::pac::TIMER1,
        capture_task: nrf52::pac::EGU0,
    ) -> Self {
        Self {
            low,
            high,
            capture_task,
        }
    }
}

impl time::Clock for SysClock {
    type T = u64;
    const SCALING_FACTOR: time::fraction::Fraction = <time::fraction::Fraction>::new(1, 16_000_000);

    fn try_now(&self) -> Result<time::Instant<Self>, time::clock::Error> {
        self.capture_task.tasks_trigger[0].write(|write| unsafe { write.bits(1) });

        let ticks =
            self.low.cc[0].read().bits() as u64 | ((self.high.cc[0].read().bits() as u64) << 32);

        Ok(time::Instant::new(ticks as Self::T))
    }
}

#[entry]
fn main() -> ! {
    let device = nrf52::pac::Peripherals::take().unwrap();

    device.TIMER0.mode.write(|w| w.mode().timer());
    device.TIMER0.bitmode.write(|w| w.bitmode()._32bit());
    device
        .TIMER0
        .prescaler
        .write(|w| unsafe { w.prescaler().bits(0) });
    device.TIMER0.cc[1].write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    device.TIMER1.mode.write(|w| w.mode().low_power_counter());
    device.TIMER1.bitmode.write(|w| w.bitmode()._32bit());
    device
        .TIMER1
        .prescaler
        .write(|w| unsafe { w.prescaler().bits(0) });

    unsafe {
        device.PPI.ch[0].eep.write(|w| {
            w.bits(&device.TIMER0.events_compare[1] as *const nrf52::pac::generic::Reg<_, _> as u32)
        });
        device.PPI.ch[0].tep.write(|w| {
            w.bits(&device.TIMER1.tasks_count as *const nrf52::pac::generic::Reg<_, _> as u32)
        });
        device.PPI.chen.modify(|_, w| w.ch0().enabled());

        device.PPI.ch[1].eep.write(|w| {
            w.bits(&device.EGU0.events_triggered[0] as *const nrf52::pac::generic::Reg<_, _> as u32)
        });
        device.PPI.ch[1].tep.write(|w| {
            w.bits(&device.TIMER0.tasks_capture[0] as *const nrf52::pac::generic::Reg<_, _> as u32)
        });
        device.PPI.fork[1].tep.write(|w| {
            w.bits(&device.TIMER1.tasks_capture[0] as *const nrf52::pac::generic::Reg<_, _> as u32)
        });
        device.PPI.chen.modify(|_, w| w.ch1().enabled());
    }

    device
        .TIMER0
        .tasks_start
        .write(|write| unsafe { write.bits(1) });
    device
        .TIMER1
        .tasks_start
        .write(|write| unsafe { write.bits(1) });

    // This moves these peripherals to prevent conflicting usage, however not the entire EGU0 is
    // used. A ref to EGU0 could be sent instead, although that provides no protection for the
    // fields that are being used by the clock.
    let clock = SysClock::take(device.TIMER0, device.TIMER1, device.EGU0);

    let port0 = nrf52::gpio::p0::Parts::new(device.P0);

    let led1 = port0.p0_17.into_open_drain_output(
        nrf52::gpio::OpenDrainConfig::Standard0Disconnect1,
        nrf52::gpio::Level::High,
    );

    let led2 = port0.p0_18.into_open_drain_output(
        nrf52::gpio::OpenDrainConfig::Standard0Disconnect1,
        nrf52::gpio::Level::High,
    );

    let led3 = port0.p0_19.into_open_drain_output(
        nrf52::gpio::OpenDrainConfig::Standard0Disconnect1,
        nrf52::gpio::Level::High,
    );

    let led4 = port0.p0_20.into_open_drain_output(
        nrf52::gpio::OpenDrainConfig::Standard0Disconnect1,
        nrf52::gpio::Level::High,
    );

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

    run(
        &mut led1.degrade(),
        &mut led2.degrade(),
        &mut led3.degrade(),
        &mut led4.degrade(),
        &clock,
    )
    .ok()
    .unwrap();

    loop {}
}

fn run<Led>(
    led1: &mut Led,
    led2: &mut Led,
    led3: &mut Led,
    led4: &mut Led,
    clock: &SysClock,
) -> Result<(), <Led as nrf52::OutputPin>::Error>
where
    Led: nrf52::OutputPin,
{
    loop {
        led1.set_low()?;
        led2.set_high()?;
        led3.set_high()?;
        led4.set_low()?;
        clock
            .new_timer(250_u32.milliseconds())
            .start()
            .ok()
            .unwrap()
            .wait()
            .ok()
            .unwrap();

        led1.set_high()?;
        led2.set_low()?;
        led3.set_low()?;
        led4.set_high()?;
        clock
            .new_timer(250_u32.milliseconds())
            .start()
            .ok()
            .unwrap()
            .wait()
            .ok()
            .unwrap();
    }
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

    pub fn try_from_generic() {
        assert_eq!(
            Seconds::try_from(duration::Generic::new(246_u32, Fraction::new(1, 2))),
            Ok(Seconds(123_u32))
        );

        let seconds: Result<Seconds<u32>, _> =
            duration::Generic::new(246_u32, Fraction::new(1, 2)).try_into();
        assert_eq!(seconds, Ok(Seconds(123_u32)));

        // Error
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

    pub fn to_generic() {
        assert_eq!(
            Seconds(123_u32).to_generic(Fraction::new(1, 2)),
            Ok(duration::Generic::new(246_u32, Fraction::new(1, 2)))
        );

        // Error
        assert_eq!(
            Seconds(u32::MAX).to_generic::<u32>(Fraction::new(1, 2)),
            Err(ConversionError::Unspecified)
        );
    }

    pub fn get_generic_integer() {
        let generic = duration::Generic::new(246_u32, Fraction::new(1, 2));
        assert_eq!(generic.integer(), 246_u32);
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

        assert_eq!(1_000_u32.nanoseconds(), 1_u32.microseconds());
        assert_eq!(1_000_000_u32.nanoseconds(), 1_u32.milliseconds());
        assert_eq!(1_000_000_000_u32.nanoseconds(), 1_u32.seconds());
    }

    pub fn error_try_from() {
        assert_eq!(
            Milliseconds::<u32>::try_from(Nanoseconds(u64::MAX)),
            Err(ConversionError::ConversionFailure)
        );
        assert_eq!(
            Milliseconds::<u32>::try_from(Seconds(u64::MAX)),
            Err(ConversionError::Unspecified)
        );
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
            Err(ConversionError::Unspecified)
        );
    }

    pub fn get_generic_integer() {
        let generic = rate::Generic::new(246_u32, Fraction::new(1, 2));
        assert_eq!(generic.integer(), 246_u32);
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
