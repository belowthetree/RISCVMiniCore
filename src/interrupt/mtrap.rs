use core::panic;

use super::{environment::Environment, exception::Exception};


#[no_mangle]
extern "C" fn m_trap(hartid : usize, cause : usize, env : &mut Environment, mtval : usize)->usize {
    let exp = Exception::new(cause);
    println!("into trap {:x}", hartid);
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