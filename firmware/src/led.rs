const LED_PIN : u8 = 6;

pub fn init() {
    unsafe{
        let p = steal_peripheral!();
        p.PORT_9.p9dir.write(|w| w.bits(1 << LED_PIN));
    }
}

pub fn toggle() {
    unsafe{
        let p = steal_peripheral!();
        p.PORT_9.p9out.modify(|r,w| w.bits(r.bits() ^ (1 << LED_PIN)));
    }
}
