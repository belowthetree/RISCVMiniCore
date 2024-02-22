#![allow(unused)]
//! # SBI v2.0 接口函数
//! 2024年2月22日

use core::{arch::asm};

pub enum EStandardSBIRet {
    SBISuccess = 0,
    SBIErrFailed = 1,
}

pub struct SBIRet
{
    pub error: usize,
    pub value: usize,
}

pub fn sbi_debug_console_write_byte(byte : u8)->SBIRet {
    let error : usize;
    let value : usize;
    unsafe {
        asm!(
            "mv a0, a3",
            "li a7, 0x4442434E",
            "li a6, 2",
            "ecall",
            in("a3") byte, out("a0") error, out("a1") value,
        )
    }
    SBIRet {
        error,
        value,
    }
}