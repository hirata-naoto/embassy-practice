#![no_std]
#![no_main]

use core::{default, ops::DerefMut};
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use embassy_stm32::gpio::{Level, Input, Output, Speed. Pull};
use embassy_executor::Spawner;
use embassy_time::Timer;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Program start!");

    let p = embassy_stm32::init(Default::default());

    let mut led = Output::new(p.PA5, Level::High, Speed::Low);

    let button = Input::new(p.PC13, Pull::Up);

    loop {
        info!("LED on");
        led.set_high();

        Timer::after_millis(500).await;
        info!("LED off");
        led.set_low();
    }
}
