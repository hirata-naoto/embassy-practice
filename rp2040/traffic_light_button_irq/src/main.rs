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

    let mut button = Input::new(p.PIN_23, Pull::Up);

    loop {
        info!("red");
        red_led.set_high();

        // 2秒間待つが、その間にボタンが押されたら即座にそちらを優先する
        // selectを使うことで「タイマー終了」と「ボタンのLow検知」を同時に待ち受けられます
        match select(Timer::after_millis(2000), button.wait_for_low()).await {
            Either::First(_) => {
                // 2秒経った場合（通常ルート）
                red_led.set_low();
            }
            Either::Second(_) => {
                // 2秒経つ前にボタンが押された場合（割り込み発生！）

                // ここに割り込み時に実行したいシーケンスを記述
                run_sequence(&mut green_led, &mut orange_led, &mut red_led).await;

                // ボタンが離されるまで少し待つ（チャタリング対策など）
                button.wait_for_high().await;
                Timer::after_millis(50).await;
            }
        }
    }
}


async fn run_sequence(green_led: &mut Output<'_>, orange_led: &mut Output<'_>, red_led: &mut Output<'_>) {
    info!("button pressed (interrupted during red)!");
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
    orange_led.set_low();
}