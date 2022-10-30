//! #堆内存
//! 在基础内存分配的基础上实现可自动扩展的堆内存
//! 2022年10月23日 zgg

mod heap;
mod memory_pool;

pub use heap::Heap;
pub use heap::MAX_HEAP_SIZE;