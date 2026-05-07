#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Launching info");

    let mut counter = 0;
    let mut green = Output::new(p.PD12, Level::Low, Speed::Medium);
    let mut yellow = Output::new(p.PD13, Level::Low, Speed::Medium);
    let mut red = Output::new(p.PD14, Level::Low, Speed::Medium);
    let mut blue = Output::new(p.PD15, Level::Low, Speed::Medium);

    let wait_time = 100;
    loop {
        counter = (counter + 1) % (1 << 4);
        info!("c: {}", counter);

        blue.set_level((counter & 1 == 1).into());
        red.set_level(((counter >> 1) & 1 == 1).into());
        yellow.set_level(((counter >> 2) & 1 == 1).into());
        green.set_level(((counter >> 3) & 1 == 1).into());

        Timer::after_millis(wait_time).await;
    }
}