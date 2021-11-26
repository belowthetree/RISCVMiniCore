#![allow(dead_code)]
pub const INSTRUCTION_ADDRESS_MISALIGNED    : usize = 0;
pub const INSTRUCTION_ACCESS_FAULT          : usize = 1;
pub const ILLEGAL_INSTRUCTION               : usize = 2;
pub const BREAKPOINT                        : usize = 3;
pub const LOAD_ACCESS_FAULT                 : usize = 5;
pub const STORE_ADDRESS_MISALIGNED          : usize = 6;
pub const STORE_ACCESS_FAULT                : usize = 7;
pub const USER_ENVIRONMENT_CALL             : usize = 8;
pub const SUPERVISOR_ENVIRONMENT_CALL       : usize = 9;
pub const MACHINE_ENVIRONMENT_CALL          : usize = 11;
pub const INSTRUCTION_PAGE_FAULT            : usize = 12;
pub const LOAD_PAGE_FAULT                   : usize = 13;
pub const STORE_PAGE_FAULT                  : usize = 15;
pub const MACHINE_SOFTWARE                  : usize = 3;
pub const SUPERVISOR_SOFTWARE               : usize = 1;
pub const SUPERVISOR_TIMER                  : usize = 5;
pub const MACHINE_TIMER                     : usize = 7;
pub const SUPERVISOR_EXTERNAL               : usize = 9;
pub const MACHINE_EXTERNAL                  : usize = 11;

pub struct Exception {
    pub is_sync : bool,
    pub code : usize,
}

impl Exception {
    pub fn new(cause : usize)->Self {
        Self {
            is_sync : (cause >> 63) != 0,
            code : cause & 0xfff,
        }
    }
}