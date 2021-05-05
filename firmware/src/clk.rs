/*
    Imports
*/
use crate::fram;
use crate::time;

/*
    Constants
*/
const CLK_KEY : u16 = 0xA500;

const DCOFSEL_0 : u16 = 0x0000;
const DCOFSEL_3 : u16 = 0x0006;
const DCOFSEL_4 : u16 = 0x0008;
const DCOFSEL_6 : u16 = 0x000C;
const DCORSEL : u16 = 0x0040;

const SELA_LFXTCLK : u16 = 0x0000;
const SELA_VLOCLK : u16 = 0x0100;
const SELS_DCOCLK : u16 = 0x0030;
const SELM_DCOCLK : u16 = 0x0003;

const DIVA_1 : u16 = 0x0000;
const DIVS_1 : u16 = 0x0000;
const DIVM_1 : u16 = 0x0000;
const DIVA_4 : u16 = 0x0200;
const DIVS_4 : u16 = 0x0020;
const DIVM_4 : u16 = 0x0002;

/*
    Functions
*/
pub fn init_dco() {
    /*
        initializes DCO at 8 MHz
    */
    unsafe{
        let p = steal_peripheral!();
        /* unlock CS settings */
        p.CS.csctl0.write(|w| w.bits(CLK_KEY));
        /* set dco to 1 MHz */
        p.CS.csctl1.write(|w| w.bits(DCOFSEL_6));
        /* Set SMCLK = MCLK = DCO, ACLK = LFXTCLK (VLOCLK if unavailable) */
        p.CS.csctl2.write(|w| w.bits( SELA_VLOCLK | SELS_DCOCLK | SELM_DCOCLK ));
        /* set all dividers to 1 */
        p.CS.csctl3.write(|w| w.bits( DIVA_1 | DIVS_1 | DIVM_1 ));
        /* wait for dco to settle */
        time::delay(100);
    }
}

pub fn get_clk_speeds() -> u32 {
    /*
        todo return clockspeeds
    */
    0
}
