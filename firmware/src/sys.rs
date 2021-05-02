#![macro_use]

macro_rules! steal_peripheral {
    () => {msp430fr6972::Peripherals::steal()}
}

pub fn init() {
    unsafe{
        let p = steal_peripheral!();
        /* stop watchdog */
        p.WATCHDOG_TIMER.wdtctl.write(|w| w.bits(0x5A00).wdthold().set_bit());
        /* disable brown out lock */
        p.PMM.pm5ctl0.write(|w| w.bits(0x00));
    }
}
