#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::i2c::master::Config;
use esp_hal::{clock::CpuClock, i2c::master::I2c};
use esp_hal::delay::Delay;
use esp_hal::main;
use lcd1602_diver::LCD1602;
use log::info;
#[main]
fn main() -> ! {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();

    let mut delay = Delay::new();
    let blocking_i2c = I2c::new(
        peripherals.I2C0, Config::default());
    let i2c = match blocking_i2c {
        Ok(i2c) => i2c,
        Err(e) => {
            panic!("Failed to initialize I2C: {:?}", e);
        }
    }.with_scl(peripherals.GPIO5)
    .with_sda(peripherals.GPIO4);
    let mut lcd = match LCD1602::new_i2c(i2c, 0x27, &mut delay) {
        Ok(lcd) => lcd,
        Err(e) => {
            panic!("Failed to initialize LCD1602: {:?}", e);
        }
    };
    match lcd.write_str("Hello world", &mut delay) {
        Ok(_) => info!("Hello world complete"),
        Err(e) => {
            panic!("Failed to write to LCD: {:?}", e);
        }
    };
    loop {
        delay.delay_millis(500);
    }
}
