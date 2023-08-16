// src/main.rs

#![no_std]
#![no_main]

use cortex_m_rt::entry; // The runtime
use stm32h7xx_hal::{pac, prelude::*};

use lcd1602_diver::{Cursor, CursorBlink, Display, DisplayMode, LCD1602};

// I2C address of the LCD1602
const I2C_ADDRESS: u8 = 0x27;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    // Configure the SCL and the SDA pin for our I2C bus
    let scl = gpiob.pb8.into_alternate_open_drain();
    let sda = gpiob.pb9.into_alternate_open_drain();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    let i2c = dp
        .I2C1
        .i2c((scl, sda), 100.kHz(), ccdr.peripheral.I2C1, &ccdr.clocks);

    let mut lcd = LCD1602::new_i2c(i2c, I2C_ADDRESS, &mut delay).expect("Init LCD failed");

    let _ = lcd.reset(&mut delay);
    let _ = lcd.clear(&mut delay);
    let _ = lcd.set_display_mode(
        DisplayMode {
            display: Display::On,
            cursor_visibility: Cursor::On,
            cursor_blink: CursorBlink::On,
        },
        &mut delay,
    );
    let _ = lcd.write_str("Hello, world!", &mut delay);
    // Move the cursor to the second line
    lcd.set_cursor_pos(40, &mut delay).expect("msg");

    // Display the following string on the second line
    lcd.write_str("Hello Imran!", &mut delay).expect("msg");

    loop {}
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
