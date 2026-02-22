use embassy_executor::Spawner;
struct SimLed;

impl firmware_core::Led for SimLed {
    fn on(&mut self) {
        println!("LED ON");
    }

    fn off(&mut self) {
        println!("LED OFF");
    }
}

pub struct SimDelay;

impl firmware_core::Delay for SimDelay {
    async fn delay_ms(&mut self, ms: u64) {
        embassy_time::Timer::after_millis(ms).await;
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut led = SimLed;
    let mut delay = SimDelay;
    firmware_core::app(&mut led, &mut delay).await;
}