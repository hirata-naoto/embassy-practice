#![no_std]
#![no_main]

use panic_halt as _;
use ch32_hal as hal;
use hal::gpio::{Level, Output};
use embassy_executor::Spawner;
use embassy_time::Timer;


#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(_spawner: Spawner) {
    let p = hal::init(hal::Config::default());

    let mut led = Output::new(p.PB2, Level::Low, Default::default());

    loop {
        led.set_high();
        Timer::after_millis(500).await; // 500ミリ秒待つ
        led.set_low();
        Timer::after_millis(500).await; // 500ミリ秒待つ
    }
}