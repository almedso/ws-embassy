#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{bind_interrupts, i2c};
use embassy_rp::peripherals::I2C0;

use embassy_stm32::i2c::{Config, I2c};
use embassy_stm32::time::Hertz;
use embassy_stm32::bind_interrupts;

use embassy_time::Timer;
use panic_halt as _;

// see https://github.com/eldruin/pwm-pca9685-rs/tree/master
use pwm_pca9685::{Address, Channel, Pca9685};

use defmt::*;
use defmt_rtt as _;

bind_interrupts!(struct Irqs {
    I2C1_IRQ => i2c::InterruptHandler<p.I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut cfg = Config::default();
    cfg.frequency = Hertz(100_000);

    // Arduino header / Grove I²C
    let mut i2c = I2c::new_async(
        p.I2C1,
        p.PB8, // SCL  (Arduino D15 / Grove SCL)
        p.PB9, // SDA  (Arduino D14 / Grove SDA)
        Irqs,
        cfg,
    );

    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).await.unwrap();

    // This corresponds to a frequency of 60 Hz.
    pwm.set_prescale(100).await.unwrap();

    // It is necessary to enable the device.
    pwm.enable().await.unwrap();

    // Turn on channel 0 at 0.
    info!("PWM is enabled");
    pwm.set_channel_on(Channel::C0, 0).await.unwrap();

    // Turn off channel 0 at 2047, which is 50% in
    // the range `[0..4095]`.
    info!("Set Channel 0x{:02x} to 0x{:04x}", Channel::C0, 2047);
    pwm.set_channel_off(Channel::C0, 2047).await.unwrap();

    let _dev = pwm.destroy(); // Get the I2C device back
}

