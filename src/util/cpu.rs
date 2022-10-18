#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn write_mscratch(v : usize) {
    unsafe {
        asm!(
            "mv t0, {val}
            csrw mscratch, t0
            ",
            val = in(reg) v
        )
    }
}

pub fn read_mhartid()->usize {
    let mut res : usize = 0;
    unsafe {
        asm!(
            "csrr {val}, mhartid",
            val = out(reg) res
        )
    }
    return res;
}

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