use core::arch::global_asm;

#[cfg(feature="qemu_riscv")]
global_asm!(include_str!("qemu_riscv/asm/trap.S"));

#[cfg(feature="qemu_riscv")]
global_asm!(include_str!("qemu_riscv/asm/strap.S"));

#[cfg(feature = "qemu_riscv")]
pub use super::qemu_riscv::trap::environment::{Environment, Register};