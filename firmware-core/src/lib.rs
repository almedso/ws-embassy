#![cfg_attr(not(feature = "std"), no_std)]

pub trait Delay {
    async fn delay_ms(&mut self, ms: u64);
}

pub trait Led {
    fn on(&mut self);
    fn off(&mut self);
}

pub async fn app<L, D>(led: &mut L, delay: &mut D)
where
    L: Led,
    D: Delay,
{
    loop {
        led.on();
        delay.delay_ms(1000).await;
        led.off();
        delay.delay_ms(1000).await;
    }
}