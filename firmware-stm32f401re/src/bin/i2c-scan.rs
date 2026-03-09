#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::i2c::{Config, I2c};
use embassy_stm32::time::Hertz;
use embassy_time::Timer;

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut cfg = Config::default();
    cfg.frequency = Hertz(100_000);

    // Arduino header / Grove I²C
    let mut i2c = I2c::new_blocking(
        p.I2C1,
        p.PB8, // SCL  (Arduino D15 / Grove SCL)
        p.PB9, // SDA  (Arduino D14 / Grove SDA)
        cfg,
    );

    info!("Starting I2C scan");

    let mut buf = [0u8; 1];

    loop {
        for addr in 0x08..0x78 {
            if i2c.blocking_write(addr, &[]).is_ok() {
                info!("Found device at 0x{:02x}", addr);
            }
        }


        info!("Scan done");
        Timer::after_secs(2).await;
    }
}