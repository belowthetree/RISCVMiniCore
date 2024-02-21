#[cfg(feature = "qemu_riscv")]
use super::qemu_riscv::interrupt;
#[cfg(feature = "qemu_opensbi")]
use super::qemu_opensbi::interrupt;

pub fn init(hartid : usize) {
    interrupt::init(hartid);
}