//! # 任务池
//! 2022年10月29日 zgg

use alloc::{collections::BTreeMap, vec, string::{String, ToString}};
use tisu_sync::*;
use crate::{memory::{heap::Heap, config::MEMORY_END, stack::{Stack, STACK_PAGE_NUM}}, arch::trap::{Environment, Register}};
use super::{task_info::{TaskMainInfo, TaskExecutionInfo, TaskState}, task_memory::TaskArea, task_resource::TaskResource};

static mut PID_COUNT : AtomCounter = AtomCounter::new();
static mut TID_COUNT : AtomCounter = AtomCounter::new();

extern "C" {
    fn process_exit();
}

pub struct TaskPool {
    task_main_infos : ContentMutex<BTreeMap<usize, TaskMainInfo>>,
    task_execution_infos : ContentMutex<BTreeMap<usize, TaskExecutionInfo>>,
}

impl TaskPool {
    pub fn new()->Self {
        Self {
            task_main_infos : ContentMutex::new(BTreeMap::new(), true),
            task_execution_infos : ContentMutex::new(BTreeMap::new(), true),
        }
    }
    /// 创建任务，返回任务 ID
    pub fn create_task(&mut self, mut task_area : TaskArea, mut env : Environment)->Option<usize> {
        // TODO，添加内存判断，内存不够返回 None
        let pid = unsafe {PID_COUNT.add() + 1};
        let tid = unsafe {TID_COUNT.add() + 1};
        let main_info = TaskMainInfo {
            pid,
            state: TaskState::Sleeping,
            is_kernel: task_area.is_kernel,
            tid: vec![tid],
            heap: Heap::new(unsafe {MEMORY_END}, task_area.is_kernel),
            task_area,
            resource: TaskResource::new(pid),
            join_num: 0,
        };
        if main_info.is_kernel {
        }
        env.satp = main_info.task_area.satp();

        let mut stack = Stack::task_stack(tid, main_info.is_kernel);
        if stack.expand(STACK_PAGE_NUM, main_info.task_area.satp()) == Err(()) {
            return None;
        }
        env.epc = main_info.task_area.entry();
        env.regs[Register::SP.val()] = stack.stack_top;
        env.regs[Register::RA.val()] = process_exit as usize;
        let exec_info = TaskExecutionInfo {
            priority: 0,
            pid,
            task_id : tid,
            state: TaskState::Sleeping,
            is_kernel: main_info.is_kernel,
            is_main: true,
            trigger_time: 0,
            stack,
            env,
        };
        self.task_main_infos.lock().insert(pid, main_info);
        self.task_execution_infos.lock().insert(tid, exec_info);
        Some(tid)
    }

    pub fn operate_task<Func>(&mut self, task_id : usize, mut operation : Func)->Result<(), String> where Func : FnMut(&mut TaskExecutionInfo)->Result<(), String> {
        if self.task_execution_infos.lock().contains_key(&task_id) {
            operation(self.task_execution_infos.lock().get_mut(&task_id).unwrap())
        }
        else {
            Err("No task".to_string())
        }
    }

    pub fn get_task_exec_env(&self, task_id : usize)->Option<Environment> {
        if let Some(t) = self.task_execution_infos.lock().get(&task_id) {
            Some(t.env)
        }
        else {
            None
        }
    }
}