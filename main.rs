#![no_std]
#![no_main]

mod fmt;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World");
    let pin = p.PС13;
    let mut led = Output::new(pin, Level::High, Speed::Low);

    let delay = Duration::from_secs(1);

    loop {
        info!("On");
        led.set_high();
        Timer::after(delay).await;

        info!("Off");
        led.set_low();
        Timer::after(delay).await;
    }
}
