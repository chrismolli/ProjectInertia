pub fn init(baud : u32){
    /*
        TODO
    */
}

fn write_byte(b : u16){
    unsafe{
        let p = steal_peripheral!();
        p.USCI_A1_UART_MODE.uca1txbuf.write(|w| w.bits(b));
    }
}

pub fn write(dat : & [u8]){
    for i in 1..dat.len() {
        write_byte(dat[i] as u16);
    }
}

pub fn read() -> u16{
    unsafe{
        let p = steal_peripheral!();
        p.USCI_A1_UART_MODE.uca1rxbuf.read().bits()
    }
}
