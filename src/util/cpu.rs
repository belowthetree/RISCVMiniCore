#![allow(dead_code)]
#![allow(unused_assignments)]

extern crate macro_derive;
use macro_derive::generate_write_csr_fn;
use macro_derive::generate_read_csr_fn;

// write_mscratch
generate_write_csr_fn!(mscratch);
generate_write_csr_fn!(sscratch);

// write_hartid
generate_read_csr_fn!(hartid);
generate_read_csr_fn!(mscratch);

pub fn dead() {
    unsafe {
        loop {
            asm!(
                "wfi"
            )
        }
    }
}

pub fn wait_for_int() {
    unsafe {
        asm!(
            "wfi"
        )
    }
}
use core::arch::asm;