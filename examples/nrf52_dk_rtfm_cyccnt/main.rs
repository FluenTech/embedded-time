#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate panic_rtt;

use core::prelude::v1::*;
use cortex_m::peripheral::DWT;
use nrf52::prelude::*;
use num::rational::Ratio;
use rtfm::time::time_units::*;

pub mod nrf52 {
    pub use nrf52832_hal::gpio;
    pub use nrf52832_hal::prelude;
    pub use nrf52832_hal::target as pac;
}

const LED_ON_TIME: Milliseconds<i32> = Milliseconds(250);

#[rtfm::app(device = nrf52832_hal::pac, peripherals = true, monotonic = rtfm::cyccnt::CYCCNT, sys_timer_freq = 64_000_000)]
const APP: () = {
    struct Resources {
        led1: nrf52::gpio::p0::P0_17<nrf52::gpio::Output<nrf52::gpio::OpenDrain>>,
        led2: nrf52::gpio::p0::P0_18<nrf52::gpio::Output<nrf52::gpio::OpenDrain>>,
        led3: nrf52::gpio::p0::P0_19<nrf52::gpio::Output<nrf52::gpio::OpenDrain>>,
        led4: nrf52::gpio::p0::P0_20<nrf52::gpio::Output<nrf52::gpio::OpenDrain>>,
    }

    #[init(spawn = [turn_on_led1])]
    fn init(mut cx: init::Context) -> init::LateResources {
        // Initialize (enable) the monotonic timer (CYCCNT)
        cx.core.DCB.enable_trace();
        // required on Cortex-M7 devices that software lock the DWT (e.g. STM32F7)
        DWT::unlock();
        cx.core.DWT.enable_cycle_counter();

        cx.spawn.turn_on_led1().unwrap();

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

    #[task(resources = [led1], spawn = [turn_on_led3])]
    fn turn_off_led1(cx: turn_off_led1::Context) {
        let led1 = cx.resources.led1;
        led1.set_high().unwrap();

        cx.spawn.turn_on_led3().unwrap();
    }

    #[task(resources = [led2], schedule = [turn_off_led2])]
    fn turn_on_led2(cx: turn_on_led2::Context) {
        cx.resources.led2.set_low().unwrap();

        cx.schedule
            .turn_off_led2(cx.scheduled + LED_ON_TIME)
            .unwrap();
    }

    #[task(resources = [led2], spawn = [turn_on_led1])]
    fn turn_off_led2(cx: turn_off_led2::Context) {
        cx.resources.led2.set_high().unwrap();

        cx.spawn.turn_on_led1().unwrap();
    }

    #[task(resources = [led3], schedule = [turn_off_led3])]
    fn turn_on_led3(cx: turn_on_led3::Context) {
        cx.resources.led3.set_low().unwrap();

        cx.schedule
            .turn_off_led3(cx.scheduled + LED_ON_TIME)
            .unwrap();
    }

    #[task(resources = [led3], spawn = [turn_on_led4])]
    fn turn_off_led3(cx: turn_off_led3::Context) {
        cx.resources.led3.set_high().unwrap();

        cx.spawn.turn_on_led4().unwrap();
    }

    #[task(resources = [led4], schedule = [turn_off_led4])]
    fn turn_on_led4(cx: turn_on_led4::Context) {
        cx.resources.led4.set_low().unwrap();

        cx.schedule
            .turn_off_led4(cx.scheduled + LED_ON_TIME)
            .unwrap();
    }

    #[task(resources = [led4], spawn = [turn_on_led2])]
    fn turn_off_led4(cx: turn_off_led4::Context) {
        cx.resources.led4.set_high().unwrap();

        cx.spawn.turn_on_led2().unwrap();
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn UARTE0_UART0();
        fn RTC0();
    }
};
