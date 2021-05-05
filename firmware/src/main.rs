#![no_main]
#![no_std]
#![feature(abi_msp430_interrupt)]

extern crate panic_msp430;

use msp430_rt::entry;
use msp430fr6972::interrupt;

mod sys;
mod led;
mod uart;
mod time;
mod clk;
mod fram;

#[entry]
fn main() -> ! {
    /*
        Init system by disabling wdog and bor lock
    */
    sys::init();

    /*
        Change Clock Source to HFX
    */
    clk::init_dco();

    /*
        Init peripherals
    */
    led::init();
    uart::init_uart1();

    let msg = "Hello World!\n\r";

    loop {
        uart::write(&msg.as_bytes());

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
