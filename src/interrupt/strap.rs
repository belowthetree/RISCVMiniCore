#![allow(dead_code)]
use core::panic;

use super::{environment::Environment};

#[no_mangle]
extern "C" fn s_trap(hartid : usize, cause : usize, env : &mut Environment, stval : usize)->usize {
    let is_interrupt = (cause >> 63) != 0;
    let code = cause & 0xfff;

    println!("into trap hartid: {:x}, stval: {:x}\n{:x}", hartid, stval, cause);

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