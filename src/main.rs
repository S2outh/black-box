#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;

use embassy_stm32::{
    Config,
    gpio::{Level, Output, Speed},
    adc::{Adc, SampleTime},
    rcc
};

use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

/// config rcc
fn get_rcc_config() -> rcc::Config {
    let mut rcc_config = rcc::Config::default();
    rcc_config.hsi = Some(rcc::HSIPrescaler::DIV1); // 64 MHz

    rcc_config.pll1 = Some(rcc::Pll {
        source: rcc::PllSource::HSI,
        prediv: rcc::PllPreDiv::DIV4,  // 16 MHz
        mul: rcc::PllMul::MUL30,       // 480 MHz
        divp: Some(rcc::PllDiv::DIV2), // 240 MHz
        divq: Some(rcc::PllDiv::DIV4), // 120 MHz
        divr: Some(rcc::PllDiv::DIV8), // 60 MHz
    });
    rcc_config.sys = rcc::Sysclk::PLL1_P; // cpu runns with 240 MHz
    rcc_config.mux.fdcansel = rcc::mux::Fdcansel::PLL1_Q; // can runns with 120 MHz
    rcc_config.voltage_scale = rcc::VoltageScale::Scale2; // voltage scale for max 300 MHz Pll out

    rcc_config.pll2 = Some(rcc::Pll {
        source: rcc::PllSource::HSI,
        prediv: rcc::PllPreDiv::DIV4,  // 16 MHz
        mul: rcc::PllMul::MUL30,       // 480 MHz
        divp: Some(rcc::PllDiv::DIV2), // 240 MHz
        divq: Some(rcc::PllDiv::DIV4), // 120 MHz
        divr: Some(rcc::PllDiv::DIV8), // 60 MHz
    });

    rcc_config.ahb_pre = rcc::AHBPrescaler::DIV2;  // AHB runns at 120 MHz
    rcc_config.apb1_pre = rcc::APBPrescaler::DIV4; // APB 1-4 all run with 60 MHz
    rcc_config.apb2_pre = rcc::APBPrescaler::DIV4;
    rcc_config.apb3_pre = rcc::APBPrescaler::DIV4;
    rcc_config.apb4_pre = rcc::APBPrescaler::DIV4;
    rcc_config
}

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
        // info!("c: {}", counter);

        blue.set_level((counter & 1 == 1).into());
        red.set_level(((counter >> 1) & 1 == 1).into());
        yellow.set_level(((counter >> 2) & 1 == 1).into());
        green.set_level(((counter >> 3) & 1 == 1).into());

        // Timekeeping is globally available, no need to mess with hardware timers.
        Timer::after_millis(wait_time).await;
    }
}

// we are only using the internal clock
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    config.rcc = get_rcc_config();
    let p = embassy_stm32::init(config);

    let green = Output::new(p.PD12, Level::Low, Speed::Medium);
    let yellow = Output::new(p.PD13, Level::Low, Speed::Medium);
    let red = Output::new(p.PD14, Level::Low, Speed::Medium);
    let blue = Output::new(p.PD15, Level::Low, Speed::Medium);

    Spawner::spawn(&_spawner, blink_gyrb(green, yellow, red, blue)).unwrap();

    let mut adc = Adc::new(p.ADC1);
    let mut vrefint = adc.enable_vrefint();

    info!("Launching main loop");
    loop {
        // info!("hello from main");
        let reading = adc.blocking_read(&mut vrefint, SampleTime::CYCLES32_5 );
        info!("just read an adc value {}", reading);

        Timer::after_millis(500).await;
    }
}