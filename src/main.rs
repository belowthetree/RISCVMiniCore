#![no_std]
#![no_main]
#![feature(
    panic_info_message,
    lang_items,
    core_intrinsics,
    alloc_error_handler,
)]

pub mod arch;
pub use arch::boot;

const OSSIGN : &str =
"
                          ..
                          ....                                                        ....
                          ......                                                      ......
                          ....                                                        ......
                          ....                                                          ..  ............
                          ....                                            ..............................
                          ....                                            ................
                          ....                                                  ....................
                          ....              ..                                ......................
                          ......................                                        ..      ..............
          ......................................                        ......................................
          ......................                                    ......................    ..
                        ........                                      ....        ................
                      ......  ....                                            ..................
                      ....    ....                                          ....................
                      ....      ....                                          ..  ......    ........
                    ......      ......                                      ............................
                    ..........    ....                                  ....................      ......
                  ....    ......  ......                                  ..........  ......        ....
                ......      ....    ....                                      ......    ..    ......
              ......          ..    ......                                    ......    ..      ........
            ......                    ......                              ........      ..        ........
          ......                      .......                           ......  ..........          ......
        ......                          .......                       ......      ........            ....
      ......                            .............             ....              ....
      ..                                  .....                                      ..
";

// 非 0 核心从此处执行初始化
#[cfg(feature = "qemu_riscv")]
#[no_mangle]
extern "C" fn m_mode_kernel_start(hartid : usize) {
    interrupt::init(hartid);
}

// 0 号核心在此处执行主要初始化内容
#[cfg(feature = "qemu_riscv")]
#[no_mangle]
extern "C" fn kernel_init() {
    println!("{}", OSSIGN);
    interrupt::init(0);
    memory::init();
    task::init(kernel_task as usize);
    cpu::shutdown();
	println!("start kernel task fail");
    cpu::dead();
}

#[cfg(feature = "qemu_opensbi")]
#[no_mangle]
extern "C" fn kernel_init() {
    use crate::arch::debug_arch_info;

    debug_arch_info();
    interrupt::init(0);
	println!("finish int");
    cpu::dead();
    println!("{}", OSSIGN);
}

pub fn kernel_task() {
	println!("start kernel task");
    timer::set_next_timer();
    cpu::shutdown();
	cpu::dead();
}


#[macro_use]
mod driver;
mod panic;
mod util;
mod memory;
mod task;
//mod filesystem;

use arch::interrupt;
pub use util::cpu;
pub use driver::uart;
pub use alloc::string::ToString;

use crate::arch::timer;

extern crate alloc;