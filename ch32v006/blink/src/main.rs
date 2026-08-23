#![no_std]
#![no_main]

use panic_halt as _;
use ch32_hal as hal;
use hal::gpio::{Level, Output};
use embassy_executor::Spawner;
use embassy_time::Timer;

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(_spawner: Spawner) {
    let mut config = hal::Config::default();
    config.rcc = hal::rcc::Config::SYSCLK_FREQ_48MHZ_HSI;
    let p = hal::init(config);

    let mut led = Output::new(p.PD6, Level::Low, Default::default());

    loop {
        led.set_high();
        Timer::after_millis(500).await;
        led.set_low();
        Timer::after_millis(500).await;
    }
}
