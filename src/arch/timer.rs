#[cfg(feature = "qemu_riscv")]
use super::qemu_riscv::interrupt::timer;

pub fn set_next_timer() {
    timer::set_next_timer()
}