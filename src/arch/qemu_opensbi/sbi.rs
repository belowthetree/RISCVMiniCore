#![allow(unused)]
//! # SBI v2.0 接口函数
//! 2024年2月22日

use core::{arch::asm};

extern crate macro_derive;
use macro_derive::{generate_sbi_extension_exist_fn, generate_sbi_fn};

#[derive(Debug, Clone, Copy)]
pub enum ESBIImplementationId {
    BerkeleyBootLoader = 0,
    OpenSBI = 1,
    Xvisor = 2,
    KVM = 3,
    RustSBI = 4,
    Diosix = 5,
    Coffer = 6,
    XenProject = 7,
    Unknown = 111,
}

impl ESBIImplementationId {
    pub fn from(v : usize)->Self {
        match v {
            0 => Self::BerkeleyBootLoader,
            1 => Self::OpenSBI,
            2 => Self::Xvisor,
            3 => Self::KVM,
            4 => Self::RustSBI,
            5 => Self::Diosix,
            6 => Self::Coffer,
            7 => Self::XenProject,
            _ => Self::Unknown,
        }
    }
}

pub enum EStandardSBIRet {
    SBISuccess = 0,
    SBIErrFailed = 1,
}

pub struct SBIRet
{
    pub error: usize,
    pub value: usize,
}

generate_sbi_fn!(sbi_debug_console_write_byte, 2, 0x4442434E, byte : u8);
generate_sbi_fn!(sbi_get_spec_version, 0, 0x10);
generate_sbi_fn!(sbi_get_impl_id, 1, 0x10);
generate_sbi_fn!(sbi_get_impl_version, 2, 0x10);
generate_sbi_fn!(sbi_probe_extension, 3, 0x10, extension_id : usize);
generate_sbi_fn!(sbi_get_mvendorid, 4, 0x10, extension_id : usize);
generate_sbi_fn!(sbi_get_marchid, 5, 0x10, extension_id : usize);
generate_sbi_fn!(sbi_get_mimpid, 6, 0x10, extension_id : usize);

generate_sbi_extension_exist_fn!(has_sbi_set_timer_implement, 0x00);
generate_sbi_extension_exist_fn!(has_sbi_console_putchar, 0x01);
generate_sbi_extension_exist_fn!(has_sbi_console_getchar, 0x02);
generate_sbi_extension_exist_fn!(has_sbi_clear_ipi, 0x03);
generate_sbi_extension_exist_fn!(has_sbi_send_ipi, 0x04);
generate_sbi_extension_exist_fn!(has_sbi_remote_fence_i, 0x05);
generate_sbi_extension_exist_fn!(has_sbi_remote_sfence_vma, 0x06);
generate_sbi_extension_exist_fn!(has_sbi_remote_sfence_vma_asid, 0x07);
generate_sbi_extension_exist_fn!(has_sbi_shutdown, 0x07);
/// 时钟扩展
generate_sbi_extension_exist_fn!(has_sbi_time, 0x54494D45);
/// IPI 扩展
generate_sbi_extension_exist_fn!(has_sbi_ipi, 0x735049);
// RFence 扩展
generate_sbi_extension_exist_fn!(has_sbi_rfence, 0x52464E43);
/// 控制 Hart 状态扩展
generate_sbi_extension_exist_fn!(has_sbi_hsm, 0x48534D);
/// 系统复位扩展
generate_sbi_extension_exist_fn!(has_sbi_srst, 0x53525354);
/// 性能监控单元扩展
generate_sbi_extension_exist_fn!(has_sbi_pmu, 0x504D55);
/// 调试控制台扩展
generate_sbi_extension_exist_fn!(has_sbi_dbcn, 0x4442434E);
/// 系统挂起扩展
generate_sbi_extension_exist_fn!(has_sbi_susp, 0x53555350);
/// CPPC 扩展
generate_sbi_extension_exist_fn!(has_sbi_cppc, 0x43505043);
/// 嵌套加速扩展
generate_sbi_extension_exist_fn!(has_sbi_nacl, 0x4E41434C);
/// Steal-time计算扩展
generate_sbi_extension_exist_fn!(has_sbi_sta, 0x535441);

// pub fn sbi_debug_console_write_byte(byte : u8)->SBIRet {
//     let error : usize;
//     let value : usize;
//     unsafe {
//         asm!(
//             "mv a0, a3",
//             "li a7, 0x4442434E",
//             "li a6, 2",
//             "ecall",
//             in("a0") byte, lateout("a0") error, lateout("a1") value,
//         )
//     }
//     SBIRet {
//         error,
//         value,
//     }
// }