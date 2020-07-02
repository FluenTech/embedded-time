#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate panic_rtt;

use core::convert::Infallible;
use cortex_m_rt::entry;
use embedded_time::{self as time, traits::*};

pub mod nrf52 {
    pub use nrf52832_hal::{
        gpio,
        prelude::*,
        target::{self as pac, Peripherals},
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
    type Rep = u64;
    const PERIOD: time::Period = <time::Period>::new(1, 16_000_000);
    type ImplError = Infallible;

    fn now(&self) -> Result<time::Instant<Self>, time::clock::Error<Self::ImplError>> {
        self.capture_task.tasks_trigger[0].write(|write| unsafe { write.bits(1) });

        let ticks =
            self.low.cc[0].read().bits() as u64 | ((self.high.cc[0].read().bits() as u64) << 32);

        Ok(time::Instant::new(ticks as Self::Rep))
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
    // fields that are being used by Clock64.
    let mut clock = SysClock::take(device.TIMER0, device.TIMER1, device.EGU0);

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

    run(
        &mut led1.degrade(),
        &mut led2.degrade(),
        &mut led3.degrade(),
        &mut led4.degrade(),
        &mut clock,
    )
    .unwrap();

    loop {}
}

fn run<Led>(
    led1: &mut Led,
    led2: &mut Led,
    led3: &mut Led,
    led4: &mut Led,
    clock: &mut SysClock,
) -> Result<(), <Led as nrf52::OutputPin>::Error>
where
    Led: nrf52::OutputPin,
{
    loop {
        led1.set_low()?;
        led2.set_high()?;
        led3.set_high()?;
        led4.set_low()?;
        clock.delay(250_u32.milliseconds()).unwrap();

        led1.set_high()?;
        led2.set_low()?;
        led3.set_low()?;
        led4.set_high()?;
        clock.delay(250_u32.milliseconds()).unwrap();
    }
}
