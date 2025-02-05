#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{prelude::*, pac, delay::Delay};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut led = gpioe.pe13.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led.set_high().unwrap();
        delay.delay_ms(500_u16);
        led.set_low().unwrap();
        delay.delay_ms(500_u16);
    }
}
