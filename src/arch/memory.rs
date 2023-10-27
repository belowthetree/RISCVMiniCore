
use super::{qemu_riscv::memory::SATP, traits::{PrivilegeType, IPageTable}};

pub type PageTableInfo = SATP;

pub fn map_page<T : IPageTable>(va : usize, pa : usize, page_type : super::traits::PageType, privilege : PrivilegeType, info : &T) {
    info.map(va, pa, page_type, privilege)
}