#[cfg(feature="qemu_riscv")]
use super::qemu_riscv::memory::SATP;
#[cfg(feature="qemu_opensbi")]
use super::qemu_opensbi::memory::SATP;

use super::traits::{PrivilegeType, IPageTable};

pub type PageTableInfo = SATP;

pub fn map_page<T : IPageTable>(va : usize, pa : usize, page_type : super::traits::PageType, privilege : PrivilegeType, info : &T) {
    info.map(va, pa, page_type, privilege)
}