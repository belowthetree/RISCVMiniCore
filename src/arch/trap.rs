use core::arch::global_asm;

#[cfg(feature="qemu_riscv")]
global_asm!(include_str!("qemu_riscv/asm/trap.S"));

#[cfg(feature="qemu_riscv")]
global_asm!(include_str!("qemu_riscv/asm/strap.S"));

#[cfg(feature = "qemu_riscv")]
pub type Environment = super::qemu_riscv::trap::environment::Environment;

#[cfg(feature = "qemu_riscv")]
pub type Register = super::qemu_riscv::trap::environment::Register;

#[cfg(feature = "qemu_opensbi")]
pub type Environment = super::qemu_opensbi::trap::environment::Environment;

#[cfg(feature = "qemu_opensbi")]
pub type Register = super::qemu_opensbi::trap::environment::Register;