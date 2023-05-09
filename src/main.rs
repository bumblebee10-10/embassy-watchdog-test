#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

use embassy_time::{Duration, Timer};
use embassy_rp::gpio::{Level, Output};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {

    let p = embassy_rp::init(Default::default());

    use embassy_rp::watchdog::*;
    let mut watchdog = Watchdog::new(p.WATCHDOG);
    watchdog.start(Duration::from_millis(14_500));
    // also it seems that specifying anything more than 5_000ms causes it to be optimized


    let mut led = Output::new(p.PIN_1, Level::High);

    loop {
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
        led.set_high();
    }
    // watchdog is not feed, but it should blink 4sec
}
