//! # 任务池
//! 2022年10月29日 zgg

use alloc::{collections::BTreeMap, vec::Vec};
use tisu_sync::*;
use super::{process::Process, thread::Thread};

pub struct TaskPool {
    process : ContentMutex<BTreeMap<usize, Process>>,
    thread : ContentMutex<BTreeMap<usize, Thread>>,
}

impl TaskPool {
    pub fn new()->Self {
        Self {
            process : ContentMutex::new(BTreeMap::new(), true),
            thread : ContentMutex::new(BTreeMap::new(), true),
        }
    }
}