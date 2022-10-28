use crate::cpu;
use self::environment::Environment;

mod mtrap;
mod strap;
mod exception;
pub mod timer;
pub mod environment;

pub fn init(hartid : usize) {
    unsafe {
        let adrress = (&mut environment::ENVS[hartid] as *mut Environment) as usize;
        cpu::write_mscratch(adrress);
    }
}