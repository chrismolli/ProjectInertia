use msp430::asm;

pub fn delay(n : u32){
    let mut i = 0;
    loop {
        asm::nop();

        i += 1;

        if i == n {
            break;
        }
    }
}
