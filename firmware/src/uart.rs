const UART1_TX_PIN : u8 = 4;
const UART1_RX_PIN : u8 = 5;

const UART_WRST : u8       = 0x01;
const UART_CSEL_SMCLK : u8 = 0x80;

const UCOS16 : u16 = 0x0001;
const UCBRF  : u16 = 0x0050;
const UCBRS  : u16 = 0x5500;

use crate::time;

pub fn init_uart1(){
    /*
        initializes the uart using baudrate of 115200
    */
    unsafe{
        let p = steal_peripheral!();
        /* put uart1 pins into uart mode */
        p.PORT_3_4.p3sel0.modify(|r,w| w.bits( r.bits() | ((1 << UART1_RX_PIN) | (1 << UART1_TX_PIN)) ));
        p.PORT_3_4.p3sel1.modify(|r,w| w.bits( r.bits() & !((1 << UART1_RX_PIN) | (1 << UART1_TX_PIN)) ));
        /* set uart 1 reset */
        p.USCI_A1_UART_MODE.uca1ctl1.modify(|r,w| w.bits(UART_WRST) );
        /* set uart 1 clk selection */
        p.USCI_A1_UART_MODE.uca1ctl1.modify(|r,w| w.bits( r.bits() | UART_CSEL_SMCLK ));
        /* set uart 1 baudrate prescaler */
        p.USCI_A1_UART_MODE.uca1br0.write(|w| w.bits(0x04));
        /* set uart 1 modulation control */
        p.USCI_A1_UART_MODE.uca1mctlw.modify(|r,w| w.bits( r.bits() | (UCOS16 | UCBRF | UCBRS) ) );
        /* release uart 1 reset */
        p.USCI_A1_UART_MODE.uca1ctl1.modify(|r,w| w.bits( r.bits() & !UART_WRST) );
    }
}

pub fn init_uart2(){
    /*
        initializes the uart using baudrate of 115200
    */
    unsafe{
        let p = steal_peripheral!();
        /* put uart 2 pins into uart mode */
        p.PORT_3_4.p4sel1.write(|w| w.bits( (1 << 3) ));
        p.PORT_1_2.p2sel1.write(|w| w.bits( (1 << 0) ));
        /* set uart 2 reset */
        p.USCI_A0_UART_MODE.uca0ctl1.modify(|r,w| w.bits(UART_WRST) );
        /* set uart 2 clk selection */
        p.USCI_A0_UART_MODE.uca0ctl1.modify(|r,w| w.bits( r.bits() | UART_CSEL_SMCLK ));
        /* set uart 2 baudrate prescaler */
        p.USCI_A0_UART_MODE.uca0br0.write(|w| w.bits(0x04));
        /* set uart 2 modulation control */
        p.USCI_A0_UART_MODE.uca0mctlw.modify(|r,w| w.bits( r.bits() | (UCOS16 | UCBRF | UCBRS) ) );
        /* release uart 2 reset */
        p.USCI_A0_UART_MODE.uca0ctl1.modify(|r,w| w.bits(r.bits() & !UART_WRST) );
    }
}

pub fn write_byte(b : u16){
    unsafe{
        let p = steal_peripheral!();
        p.USCI_A1_UART_MODE.uca1txbuf.write(|w| w.bits(b));
    }
}

pub fn write(dat : &[u8]){
    for i in 0..dat.len() {
        write_byte(dat[i] as u16);
        time::delay(100);
        /*
            TODO add check when uart available again for writing
        */
    }
}

pub fn read() -> u16{
    unsafe{
        let p = steal_peripheral!();
        p.USCI_A1_UART_MODE.uca1rxbuf.read().bits()
    }
}
