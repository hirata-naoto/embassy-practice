#![no_std]
#![no_main]


use defmt::*;
// use panic_probe as _;
use esp_backtrace as _;
use esp_println as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_hal as hal;
use esp_hal::interrupt::software::SoftwareInterruptControl;
use esp_hal::gpio::{Level, Output, Input, Pull, OutputConfig};
use esp_hal::timer::timg::TimerGroup;


#[esp_rtos::main]
async fn main(_spawner: Spawner) {
    info!("Program start!");

    let p = hal::init(hal::Config::default());

    // User LED 
    let mut led = Output::new(p.GPIO8, Level::Low, OutputConfig::default());

    // timerの初期化
    let timg0 = TimerGroup::new(p.TIMG0);
    let software_interrupt = SoftwareInterruptControl::new(p.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, software_interrupt.software_interrupt0);
    
    loop {
        led.set_high();
        Timer::after_millis(500).await; // 500ミリ秒待つ
        led.set_low();
        Timer::after_millis(500).await; // 500ミリ秒待つ
    }
}