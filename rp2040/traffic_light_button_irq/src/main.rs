#![no_std]
#![no_main]

use core::{default, ops::DerefMut};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output, Input, Pull};
use embassy_time::Timer;
use embassy_futures::select::{select, Either};


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Program start!");

    let p = embassy_rp::init(Default::default());

    let mut green_led = Output::new(p.PIN_22, Level::Low);
    let mut orange_led = Output::new(p.PIN_21, Level::Low);
    let mut red_led = Output::new(p.PIN_20, Level::Low);
    red_led.set_high();

    let mut button = Input::new(p.PIN_23, Pull::Up);

    loop {
        // 「2秒経過」または「ボタンのLow検知」を待ち受け
        match select(Timer::after_millis(2000), button.wait_for_low()).await {
            Either::First(_) => info!("timed out (during red)!"),
            Either::Second(_) => info!("button pressed (interrupted during red)!"),
        }

        red_led.set_low();

        info!("green");
        green_led.set_high();
        Timer::after_millis(2000).await;
        green_led.set_low();

        info!("orange");
        for _ in 1..4 {
            orange_led.set_high();
            Timer::after_millis(500).await;
            orange_led.set_low();
            Timer::after_millis(500).await;
        }

        info!("red");
        red_led.set_high();

        // ボタンが離されるまで少し待つ（チャタリング対策など）
        button.wait_for_high().await;
        Timer::after_millis(50).await;
    }
}
  