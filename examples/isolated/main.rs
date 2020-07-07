#![no_std]
#![no_main]

use core::convert::Infallible;
use cortex_m_rt::entry;
use embedded_time::{self as time, traits::*};
use nrf52832_hal as _;
use panic_never as _;

pub struct SysClock;
impl time::Clock for SysClock {
    type Rep = u64;
    const PERIOD: time::Period = <time::Period>::new(1, 1_000_000);
    type ImplError = Infallible;

    fn now(&self) -> Result<time::Instant<Self>, time::clock::Error<Self::ImplError>> {
        Ok(time::Instant::new(23 as Self::Rep))
    }
}

#[entry]
fn main() -> ! {
    let clock = SysClock;

    let _timer = clock.new_timer(23_u32.milliseconds()).start();
    loop {}
}
