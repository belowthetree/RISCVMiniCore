use crate::cpu;
use self::environment::Environment;
pub mod mtrap;
pub mod environment;
mod exception;

pub fn init(hartid : usize) {
    unsafe {
        let adrress = (&mut environment::ENVS[hartid] as *mut Environment) as usize;
        cpu::write_mscratch(adrress);
    }
}