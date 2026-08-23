#![no_std]
#![no_main]

use core::{default, ops::DerefMut};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut led = Output::new(p.PIN_22, Level::Low);

    loop {
        led.set_high();
        Timer::after_millis(500).await; // 500ミリ秒待つ
        led.set_low();
        Timer::after_millis(500).await; // 500ミリ秒待つ
        info!("LED toggled");
    }
}
