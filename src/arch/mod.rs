use crate::{print, println};

pub mod boot;
pub mod trap;
pub mod process;
pub mod interrupt;
pub mod timer;
pub mod memory;
pub mod traits;
pub mod driver;

#[cfg(feature="qemu_riscv")]
mod qemu_riscv;

#[cfg(feature="qemu_opensbi")]
mod qemu_opensbi;
#[cfg(feature="qemu_opensbi")]
use qemu_opensbi::sbi::*;

pub fn debug_arch_info() {
    if cfg!(feature = "qemu_opensbi") {
        let mut t = sbi_get_spec_version();
        let sub = t.value & 0xffffff;
        let main = (t.value >> 24) & 0xf7;
        println!("sbi version {} {}.{}", t.error, main, sub);
        t = sbi_get_impl_id();
        println!("implement id {} {:?}", t.error, ESBIImplementationId::from(t.value));
        t = sbi_get_impl_version();
        println!("implement version {} {:x}", t.error, t.value);
        for eid in 0 .. 0x10 {
            t = sbi_probe_extension(eid);
            if t.error == 0 {
                println!("0x{:x} extension implemented", eid);
            } else {
                println!("0x{:x} extension not implemented", eid);
            }
        }
        if has_sbi_time() {
            println!("time implemented");
        }
        if has_sbi_ipi() {
            println!("ipi implemented");
        }
        if has_sbi_rfence() {
            println!("rfence implemented");
        }
        if has_sbi_hsm() {
            println!("hart state manager implemented");
        }
        if has_sbi_srst() {
            println!("system reset implemented");
        }
        if has_sbi_pmu() {
            println!("performence supervise implemented");
        }
        if has_sbi_dbcn() {
            println!("debug console implemented");
        }
        if has_sbi_susp() {
            println!("system suspend implemented");
        }
        if has_sbi_cppc() {
            println!("cppc implemented");
        }
        if has_sbi_nacl() {
            println!("nacl implemented");
        }
        if has_sbi_sta() {
            println!("steal time implemented");
        }
    }
}