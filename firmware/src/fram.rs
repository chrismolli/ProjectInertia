const FRAM_PASSWORD : u16 = 0xA500;

pub fn set_waitstate(n_wait : u16) {
    unsafe{
        let p = steal_peripheral!();
        /* unlock fram ctrl */
        p.FRAM.frctl0.modify(|r,w| w.bits(FRAM_PASSWORD) );
        /* set wait state to 1 for 16 MHz operation */
        p.FRAM.frctl0.modify(|r,w| w.bits( r.bits() | (n_wait<<4)) );
    }
}
