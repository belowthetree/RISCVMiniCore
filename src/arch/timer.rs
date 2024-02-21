#[cfg(feature = "qemu_riscv")]
use super::qemu_riscv::interrupt::timer;
#[cfg(feature = "qemu_opensbi")]
use super::qemu_opensbi::interrupt::timer;

pub fn set_next_timer() {
    timer::set_next_timer()
}