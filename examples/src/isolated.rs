#![no_std]
#![no_main]

use core::convert::Infallible;
use cortex_m_rt::entry;
use embedded_time::{self as time};
use nrf52832_hal as _;
use panic_never as _;

pub struct SysClock;
impl time::Clock for SysClock {
    type T = u64;
    type ImplError = Infallible;
    const SCALING_FACTOR: time::fraction::Fraction = <time::fraction::Fraction>::new(1, 1_000_000);

    fn try_now(&self) -> Result<time::Instant<Self>, time::clock::Error<Self::ImplError>> {
        Ok(time::Instant::new(23 as Self::T))
    }
}

#[entry]
fn main() -> ! {
    let _clock = SysClock;

    // let _timer = clock.new_timer(23_u32.milliseconds()).start();
    loop {}
}
