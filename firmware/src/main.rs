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

#[entry]
fn main() -> ! {
    /*
        Init system by disabling wdog and bor lock
    */
    sys::init();

    /*
        Init peripherals
    */
    led::init();
    uart::init(115_200);

    loop {
        led::toggle();
        time::delay(100_000);
    }
}

#[interrupt]
fn TIMER0_A0() {

}

#[no_mangle]
extern "C" fn abort() -> ! {
    panic!();
}
