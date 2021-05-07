#![macro_use]

/*
    Imports
*/
use crate::error_codes::ErrorCodes;
use msp430::asm;

/*
    Constants
*/
const UART1_TX_PIN : u8 = 4;
const UART1_RX_PIN : u8 = 5;

const UART_WRST : u8       = 0x01;
const UART_CSEL_SMCLK : u8 = 0x80;

const UCOS16 : u16 = 0x0001;
const UCBRF  : u16 = 0x0050;
const UCBRS  : u16 = 0x5500;

const UCTXIFG : u16 = 0x0002;
const UCRXIFG : u16 = 0x0001;

/*
    Macros
*/
macro_rules! dbg_println {
    // TODO
    () => {0
        let s = format!("{}",11);
        write(&s.as_bytes())
    }
}

/*
    Enums
*/
#[derive(Clone, Copy)]
pub enum UartNum{
    Uart1,
    Uart2,
}

/*
    Functions
*/
pub fn init(u : UartNum) -> Result<(),ErrorCodes>{
    /*
        initializes the uart using baudrate of 115200
        TODO add automatic baudrate calculation
    */
    match u {
        UartNum::Uart1 => {
            unsafe{
                let p = steal_peripheral!();
                /* put uart1 pins into uart mode */
                p.PORT_3_4.p3sel0.modify(|r,w| w.bits( r.bits() | ((1 << UART1_RX_PIN) | (1 << UART1_TX_PIN)) ));
                p.PORT_3_4.p3sel1.modify(|r,w| w.bits( r.bits() & !((1 << UART1_RX_PIN) | (1 << UART1_TX_PIN)) ));
                /* set uart 1 reset */
                p.USCI_A1_UART_MODE.uca1ctl1.write(|w| w.bits(UART_WRST) );
                /* set uart 1 clk selection */
                p.USCI_A1_UART_MODE.uca1ctl1.modify(|r,w| w.bits( r.bits() | UART_CSEL_SMCLK ));
                /* set uart 1 baudrate prescaler */
                p.USCI_A1_UART_MODE.uca1br0.write(|w| w.bits(0x04));
                /* set uart 1 modulation control */
                p.USCI_A1_UART_MODE.uca1mctlw.modify(|r,w| w.bits( r.bits() | (UCOS16 | UCBRF | UCBRS) ) );
                /* release uart 1 reset */
                p.USCI_A1_UART_MODE.uca1ctl1.modify(|r,w| w.bits( r.bits() & !UART_WRST) );
            }
            Ok(())
        },
        UartNum::Uart2 => {
            /*
                Todo
            */
            Err(ErrorCodes::NotImplemented)
        }
    }
}

fn is_initialized(u : UartNum) -> bool {
    match u {
        UartNum::Uart1 => {
            unsafe{
                let p = steal_peripheral!();
                (p.USCI_A1_UART_MODE.uca1ctl1.read().bits() & UART_WRST) == 0
            }
        },
        UartNum::Uart2 => {
            unsafe{
                let p = steal_peripheral!();
                (p.USCI_A0_UART_MODE.uca0ctl1.read().bits() & UART_WRST) == 0
            }
        }
    }
}

fn tx_buffer_full(u : UartNum) -> bool {
    match u {
        UartNum::Uart1 => {
            unsafe{
                let p = steal_peripheral!();
                (p.USCI_A1_UART_MODE.uca1ifg.read().bits() & UCTXIFG) == 0
            }
        },
        UartNum::Uart2 => {
            unsafe{
                let p = steal_peripheral!();
                (p.USCI_A0_UART_MODE.uca0ifg.read().bits() & UCTXIFG) == 0
            }
        }
    }
}

pub fn write_byte(u : UartNum, b : u8) -> Result<(),ErrorCodes>{
    if !is_initialized(u){
        return Err(ErrorCodes::NotInitialized);
    }
    while tx_buffer_full(u){
        asm::nop();
    }
    match u {
        UartNum::Uart1 => {
            unsafe{
                let p = steal_peripheral!();
                p.USCI_A1_UART_MODE.uca1txbuf.write(|w| w.bits(b as u16));
            }
        },
        UartNum::Uart2 => {
            unsafe{
                let p = steal_peripheral!();
                p.USCI_A0_UART_MODE.uca0txbuf.write(|w| w.bits(b as u16));
            }
        }
    };
    Ok(())
}

pub fn write(u : UartNum, dat : &[u8]) -> Result<(),ErrorCodes>{
    for i in 0..dat.len() {
        write_byte(u, dat[i])?;
    }
    Ok(())
}

fn rx_buffer_full(u : UartNum) -> bool {
    match u {
        UartNum::Uart1 => {
            unsafe{
                let p = steal_peripheral!();
                (p.USCI_A1_UART_MODE.uca1ifg.read().bits() & UCRXIFG) == 1
            }
        },
        UartNum::Uart2 => {
            unsafe{
                let p = steal_peripheral!();
                (p.USCI_A0_UART_MODE.uca0ifg.read().bits() & UCRXIFG) == 1
            }
        }
    }
}

pub fn read(u : UartNum) -> Result<u8,ErrorCodes>{
    if !is_initialized(u){
        return Err(ErrorCodes::NotInitialized);
    }
    if rx_buffer_full(u){
        match u {
            UartNum::Uart1 => {
                unsafe{
                    let p = steal_peripheral!();
                    return Ok(p.USCI_A1_UART_MODE.uca1rxbuf.read().bits() as u8)
                }
            },
            UartNum::Uart2 => {
                unsafe{
                    let p = steal_peripheral!();
                    return Ok(p.USCI_A1_UART_MODE.uca1rxbuf.read().bits() as u8)
                }
            }
        }
    }
    Err(ErrorCodes::BufferEmpty)
}
