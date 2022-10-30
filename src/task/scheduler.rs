//! # 调度器
//! 任务调度器
//! 2022年10月30日 zgg

use alloc::collections::BTreeMap;

use super::task_info::TaskExecutionInfo;

pub struct Scheduler {}

impl Scheduler {
    pub fn schedule(exec_infos : &mut BTreeMap<usize, TaskExecutionInfo>) {
    }
}