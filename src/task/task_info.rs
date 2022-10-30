//! # 任务信息
//! 任务信息分为两种，一种是任务的执行信息，存储寄存器值、优先级等信息；二是存储任务发起程序的信息，包括内存分布、资源占用等
//! 2022年10月30日 zgg

use alloc::vec::Vec;
use crate::{memory::{map::SATP, heap::Heap, stack::Stack}, interrupt::environment::Environment};
use super::{task_resource::TaskResource, task_memory::TaskArea};

#[derive(Debug, Clone, Copy)]
pub enum TaskState {
    Running,
    Sleeping,
}

pub struct TaskMainInfo {
    pub pid : usize,
    pub satp : SATP,
    pub state : TaskState,
    pub is_kernel : bool,
    pub tid : Vec<usize>,
    pub heap : Heap,
    pub task_area : TaskArea,
    pub resource : TaskResource,
    pub join_num : usize,
}

pub struct TaskExecutionInfo {
    pub priority : usize,
    pub pid : usize,
    pub task_id : usize,
    pub state : TaskState,
    pub is_kernel : bool,
    pub is_main : bool,
    pub trigger_time : usize,
    pub stack : Stack,
    pub env : Environment,
}

