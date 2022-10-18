#![no_std]
#![no_main]
#![feature(
    panic_info_message,
    lang_items,
    core_intrinsics,
    const_fn_trait_bound,
)]

global_asm!(include_str!("asm/boot.S"));
global_asm!(include_str!("asm/mem.S"));
global_asm!(include_str!("asm/trap.S"));
//global_asm!(include_str!("asm/strap.S"));

#[no_mangle]
extern "C" fn kernel_start(hartid : usize) {
    interrupt::init(hartid);
}

#[no_mangle]
extern "C" fn kernel_init() {
    interrupt::init(0);
    println!("hello");
    cpu::dead();
}

#[macro_use]
mod driver;
mod panic;
mod interrupt;
mod util;
pub use util::cpu;
pub use driver::uart;
use core::arch::global_asm;