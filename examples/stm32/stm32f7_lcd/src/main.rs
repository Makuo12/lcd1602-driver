// src/main.rs

#![no_std]
#![no_main]

use cortex_m_rt::entry; // The runtime
use stm32f7xx_hal::{self as hal, gpio::GpioExt, pac, prelude::*};

use lcd1602_diver::{Cursor, CursorBlink, Display, DisplayMode, LCD1602};

// I2C address of the LCD1602
const I2C_ADDRESS: u8 = 0x27;

#[entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpiob = dp.GPIOB.split();

    // Configure I2C1
    let scl = gpiob.pb8.into_alternate_open_drain::<4>();
    let sda = gpiob.pb7.into_alternate_open_drain::<4>();

    let i2c = hal::i2c::BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        hal::i2c::Mode::fast(100_000.Hz()),
        &clocks,
        &mut rcc.apb1,
        50_000,
    );

    // Create a delay abstraction based on general-pupose 32-bit timer TIM5
    let mut delay = dp.TIM5.delay_us(&clocks);

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
