#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embassy_stm32::Config;
use embassy_stm32::gpio::{Level, Output, Speed};
use {defmt_rtt as _, panic_probe as _};

struct Stm32F401Led<'d> {
    led: Output<'d>,
}

impl<'d> Stm32F401Led<'d> {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        // PA5 = LD2 auf dem Nucleo-F401RE
        Stm32F401Led {
            led: Output::new(p.PA5, Level::Low, Speed::Low)
        }
    }
}

impl<'d> firmware_core::Led for Stm32F401Led<'d> {
    fn on(&mut self) {
        defmt::info!("Blink on");
        self.led.set_high();
    }

    fn off(&mut self) {
        defmt::info!("Blink off");
        self.led.set_low();
    }
}

pub struct Stm32F401Delay;

impl firmware_core::Delay for Stm32F401Delay {
    async fn delay_ms(&mut self, ms: u64) {
        Timer::after(Duration::from_millis(ms)).await;
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    config.rcc.hsi = true;
    let p = embassy_stm32::init(config);

    let mut delay = Stm32F401Delay;
    let mut led = Stm32F401Led::new(p);

    firmware_core::app(&mut led, &mut delay).await;
}

