#![no_std]
#![no_main]

use core::{default, ops::DerefMut};
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use embassy_stm32::gpio::{Flex, Level, Input, Output, Speed, Pull, OutputType};
use embassy_executor::Spawner;
use embassy_time::Timer;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Program start!");

    let p = embassy_stm32::init(Default::default());

    // let mut led_pin = Flex::new(p.PC13);
    // led_pin.set_as_output(Speed::High);
    // led_pin.set_output_type(OutputType::PushPull);
    // led_pin.set_pull(Pull::Up); // 内部プルアップを有効化

    let mut led = Output::new(p.PC13, Level::High, Speed::VeryHigh);

    let button = Input::new(p.PA0, Pull::Up);

    loop {
        info!("LED on");
        led.set_high();

        Timer::after_millis(500).await;
        info!("LED off");
        led.set_low();
    }
}
