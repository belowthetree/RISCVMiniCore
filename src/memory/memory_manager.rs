//! # 内存分配器
//! 负责整个内核所有的内存分配
//! 2022年10月19日

use tisu_memory::{MemoryOp, PageManager, Heap};
use super::config::*;

/// 内存分配用
type MemoryManager = tisu_memory::MemoryManager<PageManager, Heap<PageManager>>;

static mut USER_HEAP_START : usize = 0;
static mut MANAGER : Option<MemoryManager> = None;

/// 初始化内存分配范围
pub fn init() {
	unsafe {
		MANAGER = Some(MemoryManager::new(
			HEAP_START, KERNEL_PAGE_NUM, PAGE_SIZE, MEMORY_END
		));
		USER_HEAP_START = HEAP_START + KERNEL_PAGE_NUM * PAGE_SIZE;
		println!("st {:x}, user {:x}, ed {:x}", HEAP_START, USER_HEAP_START, MEMORY_END);
	}
}

#[inline(always)]
pub fn get_manager()->&'static mut impl MemoryOp {
	unsafe {
		let mut rt = None;
		if let Some(mgr) = &mut MANAGER {
			rt = Some(mgr);
		}
		rt.unwrap()
	}
}

#[allow(dead_code)]
pub fn print() {
	unsafe {
		if let Some(mgr) = &mut MANAGER {
			mgr.print();
		}
	}
}

pub fn test() {
	let mgr = get_manager();
	for _ in 0..10 {
		println!("addr {:x}", mgr.user_page(1).unwrap() as usize);
	}
	for _ in 0..100 {
		let addr = mgr.user_page(1).unwrap();
		println!("addr {:x}", addr as usize);
		mgr.free_page(addr);
	}
}