#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate panic_rtt;

use core::prelude::v1::*;
use cortex_m::mutex::CriticalSectionMutex as Mutex;
use mutex_trait::Mutex as _Mutex;
use num::rational::Ratio;
use rtfm::embedded_time::{self, instant::Instant, time_units::*, Period};

pub mod nrf52 {
    pub use nrf52832_hal::gpio;
    pub use nrf52832_hal::prelude;
    pub use nrf52832_hal::target as pac;
}

pub struct SystemTime {
    low: nrf52::pac::TIMER0,
    high: nrf52::pac::TIMER1,
    capture_task: nrf52::pac::EGU0,
}

impl SystemTime {
    pub fn new(
        low: nrf52::pac::TIMER0,
        high: nrf52::pac::TIMER1,
        capture_task: nrf52::pac::EGU0,
    ) -> Self {
        low.tasks_start.write(|write| unsafe { write.bits(1) });
        high.tasks_start.write(|write| unsafe { write.bits(1) });
        Self {
            low,
            high,
            capture_task,
        }
    }

    fn read(&mut self) -> u64 {
        self.capture_task.tasks_trigger[0].write(|write| unsafe { write.bits(1) });
        self.low.cc[0].read().bits() as u64 | ((self.high.cc[0].read().bits() as u64) << 32)
    }
}

impl embedded_time::Clock for SystemTime {
    type Rep = i64;

    fn now() -> Instant<Self> {
        let ticks = (&SYSTEM_TICKS).lock(|system_ticks| match system_ticks {
            Some(system_ticks) => system_ticks.read(),
            None => 0,
        });

        Instant::new(ticks as Self::Rep)
    }
}

impl Period for SystemTime {
    const PERIOD: Ratio<i32> = Ratio::new_raw(1, 16_000_000);
}

impl rtfm::Monotonic for SystemTime {
    unsafe fn reset() {
        (&SYSTEM_TICKS).lock(|ticks| match ticks {
            Some(ticks) => {
                ticks.low.tasks_clear.write(|write| write.bits(1));
                ticks.high.tasks_clear.write(|write| write.bits(1));
            }
            None => (),
        });
    }
}

static SYSTEM_TICKS: Mutex<Option<SystemTime>> = Mutex::new(None);

const LED_ON_TIME: Milliseconds<i32> = Milliseconds(250);

#[rtfm::app(device = nrf52832_hal::pac, peripherals = true, monotonic = crate::SystemTime, sys_timer_freq = 64_000_000)]
const APP: () = {
    struct Resources {
        led1: nrf52::gpio::p0::P0_17<nrf52::gpio::Output<nrf52::gpio::OpenDrain>>,
        led2: nrf52::gpio::p0::P0_18<nrf52::gpio::Output<nrf52::gpio::OpenDrain>>,
        led3: nrf52::gpio::p0::P0_19<nrf52::gpio::Output<nrf52::gpio::OpenDrain>>,
        led4: nrf52::gpio::p0::P0_20<nrf52::gpio::Output<nrf52::gpio::OpenDrain>>,
    }

    #[init(spawn = [turn_on_led1])]
    fn init(cx: init::Context) -> init::LateResources {
        cx.spawn.turn_on_led1().unwrap();

        cx.device.TIMER0.mode.write(|w| w.mode().timer());
        cx.device.TIMER0.bitmode.write(|w| w.bitmode()._32bit());
        cx.device
            .TIMER0
            .prescaler
            .write(|w| unsafe { w.prescaler().bits(0) });
        cx.device.TIMER0.cc[1].write(|w| unsafe { w.bits(0xFFFF_FFFF) });
        cx.device
            .TIMER1
            .mode
            .write(|w| w.mode().low_power_counter());
        cx.device.TIMER1.bitmode.write(|w| w.bitmode()._32bit());
        cx.device
            .TIMER1
            .prescaler
            .write(|w| unsafe { w.prescaler().bits(0) });

        unsafe {
            cx.device.PPI.ch[0].eep.write(|w| {
                w.bits(cx.device.TIMER0.events_compare[1].borrow()
                    as *const nrf52::pac::generic::Reg<_, _> as u32)
            });
            cx.device.PPI.ch[0].tep.write(|w| {
                w.bits(cx.device.TIMER1.tasks_count.borrow()
                    as *const nrf52::pac::generic::Reg<_, _> as u32)
            });
            cx.device.PPI.chen.modify(|_, w| w.ch0().enabled());

            cx.device.PPI.ch[1].eep.write(|w| {
                w.bits(cx.device.EGU0.events_triggered[0].borrow()
                    as *const nrf52::pac::generic::Reg<_, _> as u32)
            });
            cx.device.PPI.ch[1].tep.write(|w| {
                w.bits(cx.device.TIMER0.tasks_capture[0].borrow()
                    as *const nrf52::pac::generic::Reg<_, _> as u32)
            });
            cx.device.PPI.fork[1].tep.write(|w| {
                w.bits(cx.device.TIMER1.tasks_capture[0].borrow()
                    as *const nrf52::pac::generic::Reg<_, _> as u32)
            });
            cx.device.PPI.chen.modify(|_, w| w.ch1().enabled());
        }

        let system_ticks = SystemTime::new(cx.device.TIMER0, cx.device.TIMER1, cx.device.EGU0);
        (&SYSTEM_TICKS).lock(|ticks| *ticks = Some(system_ticks));

        let port0 = nrf52::gpio::p0::Parts::new(cx.device.P0);

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

        init::LateResources {
            led1,
            led2,
            led3,
            led4,
        }
    }

    #[task(resources = [led1], schedule = [turn_off_led1])]
    fn turn_on_led1(cx: turn_on_led1::Context) {
        let led1 = cx.resources.led1;
        led1.set_low().unwrap();

        cx.schedule
            .turn_off_led1(cx.scheduled + LED_ON_TIME)
            .unwrap();
    }

    #[task(resources = [led1], spawn = [turn_on_led2])]
    fn turn_off_led1(cx: turn_off_led1::Context) {
        let led1 = cx.resources.led1;
        led1.set_high().unwrap();

        cx.spawn.turn_on_led2().unwrap();
    }

    #[task(resources = [led2], schedule = [turn_off_led2])]
    fn turn_on_led2(cx: turn_on_led2::Context) {
        cx.resources.led2.set_low().unwrap();

        cx.schedule
            .turn_off_led2(cx.scheduled + LED_ON_TIME)
            .unwrap();
    }

    #[task(resources = [led2], spawn = [turn_on_led4])]
    fn turn_off_led2(cx: turn_off_led2::Context) {
        cx.resources.led2.set_high().unwrap();

        cx.spawn.turn_on_led4().unwrap();
    }

    #[task(resources = [led3], schedule = [turn_off_led3])]
    fn turn_on_led3(cx: turn_on_led3::Context) {
        cx.resources.led3.set_low().unwrap();

        cx.schedule
            .turn_off_led3(cx.scheduled + LED_ON_TIME)
            .unwrap();
    }

    #[task(resources = [led3], spawn = [turn_on_led1])]
    fn turn_off_led3(cx: turn_off_led3::Context) {
        cx.resources.led3.set_high().unwrap();

        cx.spawn.turn_on_led1().unwrap();
    }

    #[task(resources = [led4], schedule = [turn_off_led4])]
    fn turn_on_led4(cx: turn_on_led4::Context) {
        cx.resources.led4.set_low().unwrap();

        cx.schedule
            .turn_off_led4(cx.scheduled + LED_ON_TIME)
            .unwrap();
    }

    #[task(resources = [led4], spawn = [turn_on_led3])]
    fn turn_off_led4(cx: turn_off_led4::Context) {
        cx.resources.led4.set_high().unwrap();

        cx.spawn.turn_on_led3().unwrap();
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn UARTE0_UART0();
        fn RTC0();
    }
};
