#![no_main]
#![no_std]
#![feature(abi_msp430_interrupt)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate panic_msp430;

use msp430_rt::entry;
use msp430fr6972::interrupt;

use crate::uart::dbg_println;

mod sys;
mod led;
mod uart;
mod time;
mod clk;
mod error_codes;
mod adc;

#[entry]
fn main() -> ! {
    /*
        Init system by disabling wdog and bor lock
    */
    sys::init();

    /*
        Change Clock Source to DOC
    */
    clk::init();

    /*
        Init peripherals
    */
    led::init();
    uart::init(uart::UartNum::Uart1);
    adc::init();

    loop {

        match adc::read(){
            Ok(val) => dbg_println("ADC reads ",Some(val)),
            Err(_) => dbg_println("Failed reading ADC",Some(1)),
        };

        led::toggle();
        time::delay(200_000);
    }
}

#[interrupt]
fn TIMER0_A0() {

}

#[no_mangle]
extern "C" fn abort() -> ! {
    panic!();
}
