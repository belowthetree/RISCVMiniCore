//! # 用户堆管理器
//! 用于处理用户的堆内存申请
//! 在虚拟地址维护一个向上增长的堆，同时管理对应的物理页面。堆内存以内存池方式管理
//!
//! 2021年4月29日 zg


use alloc::vec::Vec;
use crate::{memory::free_memory, arch::{traits::PrivilegeType, memory::PageTableInfo}};

pub const MAX_HEAP_SIZE : usize = 4096;

use super::memory_pool::MemoryPool;

/// ## 链接同一个进程的所有堆内存
/// 非底层结构，无需 Drop
#[allow(dead_code)]
pub struct Heap{
    /// 记录堆开始的虚拟地址
    virtual_heap_start : usize,
    virtual_heap_top : usize,
    memory_area : Vec<MemoryPool>,
    privilege : PrivilegeType,
}


/// 以虚拟地址交互，不涉及物理地址
impl Heap {
    /// 根据大小新建一个，结构体本身存放在内核堆内存里，用户所需在用户内存中申请
    pub fn new(virtual_heap_start : usize, privilege : PrivilegeType)->Self {
        Self {
            virtual_heap_start,
            virtual_heap_top : virtual_heap_start,
            memory_area: Vec::new(),
            privilege,
        }
    }

    pub fn alloc(&mut self, size : usize, page : &PageTableInfo)->(usize, usize) {
        if let Some(pool) = self.memory_area.iter_mut().find(|pool| {
            pool.block_size >= size && !pool.full()
        }) {
            pool.alloc().expect("task heap pool alloc err")
        }
        else {
            self.expand(size, page);
            let pool = self.memory_area.iter_mut().find(|pool| {
                pool.block_size >= size && !pool.full()
            }).expect("heap pool find err");
            pool.alloc().expect("task heap pool alloc err after expand")
        }
    }

    pub fn free(&mut self, va : usize) {
        let pool = self.memory_area.iter_mut().find(|pool| {
            pool.contain(va)
        });
        if pool.is_none() {
            panic!("free Error va {:x}", va);
        }
        pool.unwrap().free(va);
    }

    pub fn virt_to_phy(&self, va : usize)->usize {
        let pool = self.memory_area.iter().find(|pool| {
            pool.contain(va)
        }).unwrap();
        pool.virt_to_phy(va)
    }

    fn expand(&mut self, block_size : usize, page : &PageTableInfo) {
        let pool = MemoryPool::new(
            self.virtual_heap_top, block_size, self.privilege);
        pool.map(page);
        self.virtual_heap_top += pool.total_size;
        self.memory_area.push(pool);
        self.memory_area.sort_by(|a, b| {
            a.block_size.cmp(&b.block_size)
        });
        assert!(self.virtual_heap_top - self.virtual_heap_start <= MAX_HEAP_SIZE);
    }
}
