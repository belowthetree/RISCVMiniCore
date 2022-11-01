#![no_std]
#![no_main]
#![feature(
    panic_info_message,
    lang_items,
    core_intrinsics,
    alloc_error_handler,
)]

global_asm!(include_str!("asm/boot.S"));
global_asm!(include_str!("asm/mem.S"));
global_asm!(include_str!("asm/trap.S"));
global_asm!(include_str!("asm/strap.S"));

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


#[no_mangle]
extern "C" fn kernel_start(hartid : usize) {
    interrupt::init(hartid);
}

#[no_mangle]
extern "C" fn kernel_init() {
    println!("{}", OSSIGN);
    interrupt::init(0);
    memory::init();
    task::init(kernel_task as usize);
	println!("start kernel task fail");
    cpu::dead();
}

pub fn kernel_task() {
	println!("start kernel task");
  timer::set_next_timer();
	cpu::dead();
}


#[macro_use]
mod driver;
mod panic;
mod interrupt;
mod util;
mod memory;
mod task;
//mod filesystem;

use interrupt::environment::Environment;
pub use util::cpu;
pub use driver::uart;
pub use alloc::string::ToString;
use core::arch::global_asm;

use crate::interrupt::timer;

extern crate alloc;