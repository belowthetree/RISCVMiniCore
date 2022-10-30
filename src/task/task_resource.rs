//! # 任务资源
//! 管理任务对公共资源的占用、释放
//!
//! 2022年10月30日 zgg


use alloc::vec::Vec;

pub struct TaskResource {
    pid : usize,
    files : Vec<usize>,
}

impl TaskResource {
    pub fn new(pid : usize)->Self {
        Self {
            pid,
            files : Vec::new(),
        }
    }
}

impl Drop for TaskResource {
    fn drop(&mut self) {
        // TODO 释放文件资源等
    }
}