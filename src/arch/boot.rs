use core::arch::global_asm;

#[cfg(feature="qemu_riscv")]
global_asm!(include_str!("qemu_riscv/asm/boot.S"));
#[cfg(feature="qemu_riscv")]
global_asm!(include_str!("qemu_riscv/asm/mem.S"));

#[cfg(feature="qemu_opensbi")]
global_asm!(include_str!("qemu_opensbi/asm/entry.S"));
#[cfg(feature="qemu_opensbi")]
global_asm!(include_str!("qemu_opensbi/asm/mem.S"));