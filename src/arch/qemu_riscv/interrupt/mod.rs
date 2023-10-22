use crate::cpu;
use super::trap::environment::Environment;
use super::trap::environment;

mod handler;
pub mod timer;

pub fn init(hartid : usize) {
    unsafe {
        let adrress = (&mut environment::ENVS[hartid] as *mut Environment) as usize;
        cpu::write_mscratch(adrress);
    }
}