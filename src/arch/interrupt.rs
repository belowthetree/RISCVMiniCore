#[cfg(feature = "qemu_riscv")]
use super::qemu_riscv::interrupt;

pub fn init(hartid : usize) {
    interrupt::init(hartid);
}