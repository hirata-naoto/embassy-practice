#![no_std]
#![no_main]

use core::{default, ops::DerefMut};
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use embassy_nrf::gpio::{Flex, Level, Input, Output, Pull, OutputDrive};
use embassy_executor::Spawner;
use embassy_time::Timer;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Program start!");

    let config = embassy_nrf::config::Config::default();
    let p = embassy_nrf::init(config);

    let mut led = Output::new(p.P2_00, Level::High, OutputDrive::Standard);

    let button = Input::new(p.P0_00, Pull::Up);

    loop {
        info!("LED on");
        led.set_high();

        Timer::after_millis(500).await;
        info!("LED off");
        led.set_low();
    }
}
