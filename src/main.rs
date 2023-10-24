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


#[no_mangle]
extern "C" fn kernel_start(hartid : usize) {
    interrupt::init(hartid);
}

#[no_mangle]
extern "C" fn kernel_init() {
    println!("{}", OSSIGN);
    interrupt::init(0);
    memory::init();
    cpu::shutdown();
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