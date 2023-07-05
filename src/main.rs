#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f3xx_hal as hal;
use hal::{
    pac, prelude::*
};

#[entry]
fn main() -> ! {
    hprintln!("Hello, World");
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let mut led = gpioa
        .pa0
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    loop {
        led.toggle().unwrap();
        asm::delay(8_000_000);
    }}
