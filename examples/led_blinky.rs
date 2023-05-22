#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use cortex_m_rt as rt;
use stm32f4xx_hal as hal;
use hal::prelude::*; // needed for the GpioExt trait (-> .split)


#[rt::entry]
fn main() -> !
{
  if let Some(peripherals) = hal::stm32::Peripherals::take()
  {
    // + sets RCC->AHB1ENR GPIOA bit
    let gpioa = peripherals.GPIOA.split();

    // .into_push_pull_output performs three steps
    // 1) set PUPDR: 00 -> no pull-up, no pull-down
    // 2) set OTYPER: 0 -> output push-pull
    // 3) set MODER: 01 -> general purpose output mode
    let mut led = gpioa.pa5.into_push_pull_output();

    loop
    {
      led.set_low().unwrap();
      for _ in 0..50_000 {}
      led.set_high().unwrap();
      for _ in 0..50_000 {}
    }
  }

  loop {}
}
