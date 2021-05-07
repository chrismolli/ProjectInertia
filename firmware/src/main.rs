#![no_main]
#![no_std]
#![feature(abi_msp430_interrupt)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate panic_msp430;

use msp430_rt::entry;
use msp430fr6972::interrupt;

use heapless::String;

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

    /*
        Init String Buffer
    */
    let mut msg1 : String<32> = String::from("ADC Value: ");
    let mut msg2 : String<32>;

    loop {

        match adc::read(){
            Ok(val) => msg2 = String::from(val),
            Err(_) => msg2 = String::from("None"),
        };

        uart::write(uart::UartNum::Uart1, &msg1.as_bytes());
        uart::write(uart::UartNum::Uart1, &msg2.as_bytes());
        uart::write(uart::UartNum::Uart1, &"\n\r".as_bytes());

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
