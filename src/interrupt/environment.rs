//! 中断过程中的环境

#[allow(dead_code)]
enum Register{
    RA = 1,
    SP = 2,
    A0 = 10,
    A1 = 11,
    A2 = 12,
    A3 = 13,
    A4 = 14,
    A5 = 15,
}

impl Register {
    fn val(self)->usize {
        self as usize
    }
}

/// 保存需要恢复的环境
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct Environment{
    regs    :     [usize;32], // 0 ~ 255
    fregs   :     [usize;32], // 256 ~ 511
    pub satp    :     usize,      // 512
    pub epc     :     usize,      // 520
    pub hartid  :     usize,      // 528
}

impl Environment {
    pub const fn new() -> Self {
        Self {
            regs: [0;32],
            fregs: [0;32],
            satp: 0,
            epc: 0,
            hartid: 0,
        }
    }

    pub fn ra(&self)->usize {
        self.regs[Register::RA.val()]
    }
}

pub static mut ENVS : [Environment;4] = [Environment::new();4];
