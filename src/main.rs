#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

// Declare async tasks
#[embassy_executor::task]
async fn blink_gyrb(
    mut green: Output<'static>,
    mut yellow: Output<'static>,
    mut red: Output<'static>,
    mut blue: Output<'static>,
) {

    let mut counter = 0;
    let wait_time = 100;

    info!("Launching blink loop");
    loop {
        counter = (counter + 1) % (1 << 4);
        info!("c: {}", counter);

        blue.set_level((counter & 1 == 1).into());
        red.set_level(((counter >> 1) & 1 == 1).into());
        yellow.set_level(((counter >> 2) & 1 == 1).into());
        green.set_level(((counter >> 3) & 1 == 1).into());

        // Timekeeping is globally available, no need to mess with hardware timers.
        Timer::after_millis(wait_time).await;
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let green = Output::new(p.PD12, Level::Low, Speed::Medium);
    let yellow = Output::new(p.PD13, Level::Low, Speed::Medium);
    let red = Output::new(p.PD14, Level::Low, Speed::Medium);
    let blue = Output::new(p.PD15, Level::Low, Speed::Medium);

    Spawner::spawn(&_spawner, blink_gyrb(green, yellow, red, blue)).unwrap();

    info!("Launching main loop");
    loop {
        info!("hello from main");
        Timer::after_millis(500).await;
    }
}