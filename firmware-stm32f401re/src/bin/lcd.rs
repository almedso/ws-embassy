#![no_std]
#![no_main]

use cortex_m::asm;

use embassy_executor::Spawner;
use embassy_stm32::i2c::{Config, I2c};
use embassy_stm32::peripherals::I2C1;
use embassy_stm32::{bind_interrupts, time::Hertz};
use embassy_stm32::i2c::EventInterruptHandler;
use embassy_stm32::i2c::ErrorInterruptHandler;
use embedded_hal::delay::DelayNs;
use embassy_time::{Instant, Duration};

use {defmt_rtt as _, panic_probe as _};
use defmt::info;


use grove_lcd_rgb::GroveLcd;

/// Synchronous delay implementation using embassy's monotonic clock.
/// This provides accurate timing without busy-wait loops.
pub struct EmbassyDelay;

impl embedded_hal::delay::DelayNs for EmbassyDelay {
    fn delay_ns(&mut self, ns: u32) {
        let target = Instant::now() + Duration::from_nanos(ns as u64);
        while Instant::now() < target {}
    }
}

bind_interrupts!(struct Irqs {
    I2C1_EV => EventInterruptHandler<I2C1>;
    I2C1_ER => ErrorInterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {

    let p = embassy_stm32::init(Default::default());

    let mut config = Config::default();
    config.frequency = Hertz(100_000);

    info!("Init I2c");
    let i2c = I2c::new(
        p.I2C1,
        p.PB8, // SCL - arduino D15 on Nucleo-F401RE
        p.PB9, // SDA - arduino D14 on Nucleo-F401RE
        Irqs,
        p.DMA1_CH6, // TX
        p.DMA1_CH0, // RX
        config,
    );
    info!("Delays");

    let delay = EmbassyDelay;

    info!("I2C LCD start");
    // Create LCD instance with your I2C peripheral and delay provider
    let mut lcd = GroveLcd::new(i2c, delay);

    info!("I2C LCD initialized");
    
    // Initialize LCD (16 columns, 2 rows)
    info!("[STEP 1] About to call lcd.begin(16, 2)...");
    lcd.begin(16, 2).unwrap();
    info!("[STEP 1] lcd.begin() completed");
    
    embassy_time::Timer::after(Duration::from_micros(5)).await;

    // Set backlight color to green
    info!("[STEP 2] About to call lcd.set_rgb(0, 255, 0)...");
    lcd.set_rgb(0, 255, 0).unwrap();
    info!("[STEP 2] lcd.set_rgb() completed");
    
    embassy_time::Timer::after(Duration::from_micros(5)).await;

    // Display text
    info!("[STEP 3] About to call lcd.print('Hello, World!')...");
    lcd.print("Hello, World!").unwrap();
    info!("[STEP 3] lcd.print() completed");
    
    embassy_time::Timer::after(Duration::from_millis(5)).await;

    // Set cursor to second line
    info!("[STEP 4] About to call lcd.set_cursor(0, 1)...");
    lcd.set_cursor(0, 1).unwrap();
    info!("[STEP 4] lcd.set_cursor() completed");
    
    info!("[STEP 5] About to call lcd.print('Grove LCD')...");
    lcd.print("Grove LCD").unwrap();
    info!("[STEP 5] lcd.print() completed");
    
    info!("All LCD operations completed successfully");

    // keep main running indefinitely
    loop {
        asm::nop();
    }
}

// #[panic_handler]
// fn panic(_info: &core::panic::PanicInfo) -> ! {
//     // panic_probe will handle the panic reporting
//     loop {}
// }

