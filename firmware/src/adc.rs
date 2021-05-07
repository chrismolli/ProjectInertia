// ADC PIN P1.3 (A3) FSEL=2
const ADC_PIN : u8 = 3;

// ctl0
const ADC12SHT0_2 : u16 = 0x0200;
const ADC12ON : u16 = 0x0010;
const ADC12ENC : u16 = 0x0002;
const ADC12SC : u16 = 0x0001;

// ctl1
const ADC12BUSY : u16 = 0x0001;
const ADC12SHP : u16 = 0x0200;

// ctl2
const ADC12RES_2 : u16 = 0x0020;

// mctl0
const ADC12INCH_3 : u16 = 0x0003;

/*
    Imports
*/
use crate::error_codes::ErrorCodes;
use msp430::asm;

/*
    Functions
*/
pub fn init() -> Result<(),ErrorCodes>{
    unsafe{
        let p = steal_peripheral!();
        /* put p1.3 into adc mode */
        p.PORT_1_2.p1sel0.modify(|r,w| w.bits( r.bits() | (1 << ADC_PIN) ));
        p.PORT_1_2.p1sel1.modify(|r,w| w.bits( r.bits() & !(1 << ADC_PIN) ));
        /* init sample time to 16 cycles and start adc */
        p.ADC12.adc12ctl0.write(|w| w.bits(ADC12SHT0_2|ADC12ON) );
        /* use sampling timer */
        p.ADC12.adc12ctl1.write(|w| w.bits(ADC12SHP) );
        /* use use 12 bit resolution */
        p.ADC12.adc12ctl2.modify(|r,w| w.bits(r.bits() | ADC12RES_2) );
        /* select analog input 3 */
        p.ADC12.adc12mctl0.modify(|r,w| w.bits(r.bits() | ADC12INCH_3) );
    }
    Ok(())
}

fn is_initialized() -> bool{
    unsafe{
        let p = steal_peripheral!();
        (p.ADC12.adc12ctl0.read().bits() & ADC12ON ) != 0
    }
}

fn is_busy() -> bool{
    unsafe{
        let p = steal_peripheral!();
        (p.ADC12.adc12ctl1.read().bits() & ADC12BUSY ) != 0
    }
}

pub fn read() -> Result<u16,ErrorCodes>{
    if !is_initialized(){
        return Err(ErrorCodes::NotInitialized)
    }
    unsafe{
        let p = steal_peripheral!();
        /* trigger conversion */
        p.ADC12.adc12ctl0.modify(|r,w| w.bits( r.bits()|ADC12ENC|ADC12SC ) );
    }
    while is_busy(){
        asm::nop();
    }
    unsafe{
        let p = steal_peripheral!();
        return Ok(p.ADC12.adc12mem3.read().bits())
    }
}
