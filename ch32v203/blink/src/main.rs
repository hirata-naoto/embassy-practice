#![no_std]
#![no_main]


use defmt::*;
use defmt_rtt as _;
// use panic_probe as _;
use panic_halt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use ch32_hal as hal;
use hal::gpio::{Level, Output, Input, Pull};
use hal::usart::{Config as UsartConfig, UartTx};


#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(_spawner: Spawner) {
    let p = hal::init(hal::Config::default());

    info!("Program start!");


    // User LED (PB2)
    let mut led = Output::new(p.PB2, Level::Low, Default::default());

    // ボタン (PA0) 内部プルアップ、押したらLow想定
    let button = Input::new(p.PA0, Pull::Up);

    // シリアル送信のみ (USART1 TX=PA9)
    let mut usart_config = UsartConfig::default();
    usart_config.baudrate = 115200;
    let mut tx = UartTx::new_blocking(p.USART1, p.PA9, usart_config).unwrap();


    loop {
        led.set_high();
        Timer::after_millis(500).await; // 500ミリ秒待つ
        led.set_low();
        Timer::after_millis(500).await; // 500ミリ秒待つ
    }
}