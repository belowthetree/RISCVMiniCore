use core::panic;
use crate::{cpu, println, print};
use super::environment::Environment;

#[no_mangle]
extern "C" fn m_trap(hartid : usize, cause : usize, env : &mut Environment, mtval : usize)->usize {
    let is_interrupt = (cause >> 63) != 0;
    let code = cause & 0xfff;
    println!("into trap hartid: {:x}, mtval: {:x} cause {:x} epc {:x} satp {:x}", hartid, mtval, cause, env.epc, env.satp);
    cpu::dead();
    if is_interrupt {
        match code {
            _ => panic!("unhandle sync number: {:016x}", code),
        }
    } else {
        match code {
            _ => panic!("unhandle interrupt number: {:016x}", code),
        }
    }
}