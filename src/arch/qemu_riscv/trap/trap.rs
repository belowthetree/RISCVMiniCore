#![allow(dead_code)]
pub const INSTRUCTION_ADDRESS_MISALIGNED    : usize = 0;
pub const INSTRUCTION_ACCESS_FAULT          : usize = 1;
pub const ILLEGAL_INSTRUCTION               : usize = 2;
pub const BREAKPOINT                        : usize = 3;
pub const LOAD_ADDRESS_MISALIGNED           : usize = 4;
pub const LOAD_ACCESS_FAULT                 : usize = 5;
pub const STORE_ADDRESS_MISALIGNED          : usize = 6;
pub const STORE_ACCESS_FAULT                : usize = 7;
pub const USER_ENVIRONMENT_CALL             : usize = 8;
pub const SUPERVISOR_ENVIRONMENT_CALL       : usize = 9;
pub const MACHINE_ENVIRONMENT_CALL          : usize = 11;
pub const INSTRUCTION_PAGE_FAULT            : usize = 12;
pub const LOAD_PAGE_FAULT                   : usize = 13;
pub const STORE_PAGE_FAULT                  : usize = 15;

pub const SUPERVISOR_SOFTWARE               : usize = 1;
pub const MACHINE_SOFTWARE                  : usize = 3;
pub const SUPERVISOR_TIMER                  : usize = 5;
pub const MACHINE_TIMER                     : usize = 7;
pub const SUPERVISOR_EXTERNAL               : usize = 9;
pub const MACHINE_EXTERNAL                  : usize = 11;

#[derive(Clone, Copy)]
pub enum EInterrupt {
    SupervisorSoftware = 1,
    MachineSoftware = 3,
    SupervisorTimer = 5,
    MachineTimer = 7,
    SupervisorExternal = 9,
    MachineExternal = 11,
}

#[derive(Clone, Copy)]
pub enum EException {
    InstructionAddressMisaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadAddressMisaligned = 4,
    // 权限
    LoadAccessFault = 5,
    StoreAddressMisaligned = 6,
    // 权限
    StoreAccessFault = 7,
    UserEnvCall = 8,
    SupervisorEnvCall = 9,
    MachineEnvCall = 11,
    // 可能是页表映射的物理地址不存在
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 14,
}

pub union TrapResult {
    exception : EException,
    interrupt : EInterrupt,
    err : bool,
}

pub struct Trap {
    pub is_interrupt : bool,
    pub result : TrapResult,
}

impl Trap {
    pub fn new(cause : usize)->Self {
        let is_interrupt = (cause >> 63) != 0;
        let code = cause & 0xfff;
        let result;
        if is_interrupt {
            match code {
                SUPERVISOR_SOFTWARE => result = TrapResult {interrupt : EInterrupt::SupervisorSoftware},
                MACHINE_SOFTWARE => result = TrapResult {interrupt : EInterrupt::MachineExternal},
                SUPERVISOR_TIMER => result = TrapResult {interrupt : EInterrupt::SupervisorTimer},
                MACHINE_TIMER => result = TrapResult {interrupt : EInterrupt::MachineTimer},
                SUPERVISOR_EXTERNAL => result = TrapResult {interrupt : EInterrupt::SupervisorExternal},
                _ => result = TrapResult {err : true},
            }
        }
        else {
            match code {
                INSTRUCTION_ADDRESS_MISALIGNED => result = TrapResult {exception : EException::InstructionAddressMisaligned},
                INSTRUCTION_ACCESS_FAULT => result = TrapResult {exception : EException::InstructionAccessFault},
                ILLEGAL_INSTRUCTION => result = TrapResult {exception : EException::IllegalInstruction},
                BREAKPOINT => result = TrapResult {exception : EException::Breakpoint},
                LOAD_ADDRESS_MISALIGNED => result = TrapResult {exception : EException::LoadAddressMisaligned},
                LOAD_ACCESS_FAULT => result = TrapResult {exception : EException::LoadAccessFault},
                STORE_ADDRESS_MISALIGNED => result = TrapResult {exception : EException::StoreAddressMisaligned},
                STORE_ACCESS_FAULT => result = TrapResult {exception : EException::StoreAccessFault},
                USER_ENVIRONMENT_CALL => result = TrapResult {exception : EException::UserEnvCall},
                SUPERVISOR_ENVIRONMENT_CALL => result = TrapResult {exception : EException::SupervisorEnvCall},
                MACHINE_ENVIRONMENT_CALL => result = TrapResult {exception : EException::MachineEnvCall},
                INSTRUCTION_PAGE_FAULT => result = TrapResult {exception : EException::InstructionPageFault},
                LOAD_PAGE_FAULT => result = TrapResult {exception : EException::LoadPageFault},
                STORE_PAGE_FAULT => result = TrapResult {exception : EException::StorePageFault},
                _ => result = TrapResult {err : false},
            }
        }
        Self {
            is_interrupt,
            result
        }
    }
}