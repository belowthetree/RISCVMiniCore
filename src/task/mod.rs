//! 任务系统
//! 任务系统基于任务池、调度器两个接口实现，可以自由替换、扩展
//! 任务系统的调度放置于主线程执行
//! 2022年10月29日 zgg

mod task_pool;
mod task_info;
mod task_resource;
mod task_memory;
mod scheduler;

use task_pool::TaskPool;
use crate::interrupt::environment::Environment;
use self::{scheduler::Scheduler, task_memory::TaskArea, task_info::TaskExecutionInfo};

pub use task_info::TaskState;

extern "C" {
	fn start_kernel_process(env_addr : *mut u8);
}

pub struct TaskManager<TaskPoolType, SchedulerType> {
    task_pool : TaskPoolType,
    scheduler : SchedulerType,
}

static mut TASK_MANAGER : Option<TaskManager<TaskPool, Scheduler>> = None;

pub fn init(entry : usize) {
    unsafe {
        TASK_MANAGER = Some(TaskManager { task_pool: TaskPool::new(), scheduler : Scheduler {} })
    }
    let task_id = create_task(entry, true).unwrap();
	set_task_state(task_id, TaskState::Running);
	schedule();
    let env = get_manager().task_pool.get_task_exec_env(task_id).expect("no env");
    println!("before start");
    unsafe {
        start_kernel_process(&env as *const Environment as *mut u8);
    }
	println!("start kernel task fail");
}

/// 创建任务并返回任务 id
pub fn create_task(entry : usize, is_kernel : bool)->Option<usize> {
    let task_area;
    if is_kernel {
        task_area = TaskArea::kernel_area(entry);
    }
    else {
        task_area = TaskArea::new(entry, is_kernel)
    }
    get_manager().task_pool.create_task(task_area, Environment::new())
}

pub fn set_task_state(task_id : usize, state : TaskState) {
    get_manager().task_pool.operate_task(task_id, |info : &mut TaskExecutionInfo|{
        info.state = state;
        Ok(())
    }).unwrap();
}

pub fn schedule() {
    
}

fn get_manager()->&'static mut TaskManager<TaskPool, Scheduler> {
    unsafe {
        let mut t = None;
        if let Some(mgr) = &mut TASK_MANAGER {
            t = Some(mgr);
        }
        t.unwrap()
    }
}
