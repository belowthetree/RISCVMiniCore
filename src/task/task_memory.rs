//! # 程序内存区域
//!
//! 2021年4月29日 zg

use core::cmp::{max, min};
use crate::{memory::{config::{DATA_START, HEAP_START, KERNEL_PAGE_NUM, MEMORY_START, PAGE_SIZE, RODATA_END}, map::SATP, free_page}};
use alloc::vec::Vec;

extern "C" {
    fn thread_exit();
    fn process_exit();
    fn waiting();
    fn s_trap_vector();
}

#[derive(Debug)]
pub struct Area {
    vst : usize,
    ved : usize,
    pst : usize,
    ped : usize,
    atype : AreaType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AreaType {
    Code,
    Data,
    All,
}

impl Area {
    pub fn new(vst : usize, ved : usize, pst: usize, ped: usize)->Self {
        Self {
            vst,
            ved,
            pst,
            ped,
            atype : AreaType::All
        }
    }

    pub fn kernel_data()->Self {
        let vst = unsafe {DATA_START};
        let ved = unsafe {HEAP_START + KERNEL_PAGE_NUM * PAGE_SIZE};
        let ved = (ved + PAGE_SIZE - 1) / PAGE_SIZE * PAGE_SIZE;
        let vst = vst / PAGE_SIZE * PAGE_SIZE;
        Self {
            vst,
            ved,
            pst : vst,
            ped : ved,
            atype : AreaType::Data
        }
    }

    pub fn kernel_code()->Self {
        let vst = unsafe {MEMORY_START};
        let ved = unsafe {RODATA_END};
        let ved = (ved + PAGE_SIZE - 1) / PAGE_SIZE * PAGE_SIZE;
        let vst = vst / PAGE_SIZE * PAGE_SIZE;
        Self {
            vst,
            ved,
            pst : vst,
            ped : ved,
            atype : AreaType::Code
        }
    }

    pub fn virtio_area()->Self {
        let vst = 0x1000_0000;
        let ved = 0x1000_9000;
        Self {
            vst,
            ved,
            pst : vst,
            ped:ved,
            atype : AreaType::Data
        }
    }

    pub fn rtc_area()->Self {
        let st = crate::driver::rtc::BASE_ADDR / PAGE_SIZE * PAGE_SIZE;
        let ed = (crate::driver::rtc::BASE_ADDR + PAGE_SIZE - 1) / PAGE_SIZE * PAGE_SIZE;
        Self {
            vst : st,
            pst : st,
            ved : ed,
            ped : ed,
            atype : AreaType::Data,
        }
    }

    pub fn test_area()->Self {
        Self {
            vst : 0x10_0000,
            ved : 0x10_1000,
            ped : 0x10_1000,
            pst : 0x10_0000,
            atype : AreaType::Data,
        }
    }

    pub fn timer_area()->Self {
        let vst = 0x200_0000;
        let ved = 0x200_C000;
        Self {
            vst,
            ved,
            pst : vst,
            ped:ved,
            atype : AreaType::Data
        }
    }

    /// 内核中用户程序会用到的函数
    pub fn user_func()->Self {
        let st = min(process_exit as usize, thread_exit as usize);
        let st = min(st, waiting as usize);
        let ed = max(process_exit as usize, thread_exit as usize);
        let ed = max(ed, waiting as usize);
        let ed = (ed + PAGE_SIZE - 1) / PAGE_SIZE * PAGE_SIZE;
        let st = st / PAGE_SIZE * PAGE_SIZE;
        Self {
            vst:st,
            ved:ed,
            pst:st,
            ped:ed,
            atype:AreaType::Code
        }
    }

    pub fn contains(&self, va:usize)->bool {
        self.vst <= va && self.ved > va
    }

    pub fn virt_to_phy(&self, va:usize)->usize {
        self.pst + va - self.vst
    }
}

pub struct TaskArea {
    entry : usize,
    areas : Vec<Area>,
    satp : SATP,
    pub is_kernel : bool
}

impl TaskArea {
    pub fn new(entry:usize, is_kernel : bool)->Self {
        Self {
            entry,
            areas : Vec::new(),
            satp : SATP::new(),
            is_kernel
        }
    }
    pub fn kernel_area(entry:usize)->Self {
        let mut slf = Self::new(entry, true);
        slf.push_area(Area::kernel_code());
        slf.push_area(Area::kernel_data());
        slf.push_area(Area::virtio_area());
        slf.push_area(Area::timer_area());
        slf.push_area(Area::rtc_area());
        slf.push_area(Area::test_area());
        slf
    }

    pub fn entry(&self)->usize {
        self.entry
    }

    pub fn satp(&self)->usize {
        self.satp.flag
    }

    pub fn push_area(&mut self, area : Area) {
        let mut vst = area.vst;
        let mut pst = area.pst;
        while vst < area.ved && pst < area.ped {
            match area.atype {
                AreaType::Code => self.satp.map_code(vst, pst, self.is_kernel),
                AreaType::Data => self.satp.map_data(vst, pst, self.is_kernel),
                AreaType::All => self.satp.map_all(vst, pst, self.is_kernel)
            }
            vst += PAGE_SIZE;
            pst += PAGE_SIZE;
        }
        self.areas.push(area);
    }

    pub fn virt_to_phy(&self, va:usize)->usize {
        // println!("va {:x}", va);
        let area = self.areas.iter().find(|area| {
            // println!("vst {:x} ved {:x}", area.vst, area.ved);
            area.contains(va)
        }).unwrap();
        area.virt_to_phy(va)
    }

    pub fn contains(&self, va:usize)->bool {
        self.areas.iter().find(|area| {
            area.contains(va)
        }).is_some()
    }
}

impl Drop for TaskArea {
    fn drop(&mut self) {
        for area in self.areas.iter() {
            let addr = area.pst;
            // HEAP_START 以下的地址不是内存分配所得，不需要回收
            if addr < unsafe {HEAP_START} {
                continue;
            }
            free_page(addr as *mut u8);
        }
    }
}
