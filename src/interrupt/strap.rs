#![allow(dead_code)]
use core::panic;

use super::{environment::Environment, exception::Exception};

#[no_mangle]
extern "C" fn s_trap(hartid : usize, cause : usize, env : &mut Environment, stval : usize)->usize {
    let exp = Exception::new(cause);
    println!("into trap hartid: {:x}, stval: {:x}\nenv: {:x?}", hartid, stval, env);
    if exp.is_sync {
        match exp.code {
            _ => panic!("unhandle sync number: {:016x}", exp.code),
        }
    } else {
        match exp.code {
            _ => panic!("unhandle interrupt number: {:016x}", exp.code),
        }
    }
}