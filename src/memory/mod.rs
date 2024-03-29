//! # 内存管理
//! 内存分为物理内存、虚拟内存管理
//! 基础内存分配使用外部的内存管理器（默认 tisu-memory），在此基础上实现 stack、heap、program_memory。同时约定，所有自定义的内存结构必须实现 Drop 接口
//! 程序的内存申请交由堆内存进行管理
//! 
//! 为了方便替换内存管理器，页、块管理器在 memory_manager 中初始化并抽象出接口
//! 系统最后通过 stack、heap、block 等结构使用内存
//!
//! 2021年1月25日 zg

#![allow(unused)]

use self::{
	config::{HEAP_START, KERNEL_PAGE_NUM, MEMORY_END, PAGE_SIZE},
};
use core::alloc::{GlobalAlloc, Layout};

mod memory_manager;
pub mod block;
pub mod heap;
pub mod config;
pub mod stack;

use tisu_memory::MemoryOp;

//pub use program_memory::*;

pub fn init(){
	memory_manager::init();
	// test();
}

#[inline]
pub fn kernel_page(page_num : usize) ->Option<*mut u8> {
	memory_manager::get_manager().kernel_page(page_num)
}

#[inline]
pub fn user_page(page_num : usize)->Option<*mut u8> {
	memory_manager::get_manager().user_page(page_num)
}

#[inline]
pub fn free_page(addr : *mut u8) {
	memory_manager::get_manager().free_page(addr)
}

#[inline]
pub fn alloc_kernel(size : usize)->Option<*mut u8> {
	memory_manager::get_manager().alloc_memory(size, true)
}

#[inline]
pub fn alloc_user(size : usize)->Option<*mut u8> {
	memory_manager::get_manager().alloc_memory(size, false)
}

#[inline]
pub fn free_memory(addr : *mut u8) {
	memory_manager::get_manager().free_memory(addr)
}

#[allow(dead_code)]
pub fn test() {
	memory_manager::test();
}

/// 为 rust 容器实现内存分配
struct OSGlobalAlloc;
unsafe impl GlobalAlloc for OSGlobalAlloc {
    unsafe fn alloc(&self, layout : Layout) -> *mut u8 {
        alloc_kernel(layout.size()).unwrap()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free_memory(ptr);
    }
}

#[global_allocator]
static GA: OSGlobalAlloc = OSGlobalAlloc{};

#[alloc_error_handler]
pub fn alloc_error(layout : Layout) -> !{
    panic!("Fail to alloc {} bytes with {} bytes alignment", layout.size(), layout.align());
}