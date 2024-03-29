#![allow(dead_code)]
#![allow(unused_assignments)]

extern crate macro_derive;
use macro_derive::generate_write_csr_fn;
use macro_derive::generate_read_csr_fn;
use core::arch::asm;

// write_mscratch
generate_write_csr_fn!(mscratch);
generate_write_csr_fn!(sscratch);

// read_hartid
generate_read_csr_fn!(hartid);
generate_read_csr_fn!(mscratch);
generate_read_csr_fn!(scause);

pub fn dead() {
    unsafe {
        loop {
            asm!(
                "wfi"
            )
        }
    }
}

pub fn shutdown() {
    unsafe {
        const VIRT_TEST: *mut u32 = 0x10_0000 as *mut u32;
        VIRT_TEST.write_volatile(0x5555);
    }
}

pub fn wait_for_int() {
    unsafe {
        asm!(
            "wfi"
        )
    }
}