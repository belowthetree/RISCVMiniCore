//! # 栈结构
//! 在虚拟地址空间向下扩展的内存结构，自动拼接物理页
//! 2022年10月19日 zgg

#![allow(unused)]

use alloc::vec::Vec;
use super::map::SATP;
use super::*;

/// 用于描述栈区域从物理地址到虚拟地址的映射
#[derive(Debug, Clone, Copy)]
struct StackArea {
    vst : usize,    /// 虚拟地址起始
    ved : usize,
    pst : usize,    /// 物理地址起始
    ped : usize,
}

impl StackArea {
    pub fn new(vst : usize, ved : usize, pst : usize, ped : usize)->Self {
        Self {
            vst,
            ved,
            pst,
            ped,
        }
    }

    pub fn contain(&self, va: usize)->bool {
        self.vst <= va && self.ved > va
    }

    pub fn virt_to_phy(&self, va: usize)->usize {
        self.pst + (va - self.vst)
    }
}

/// 描述完整栈信息
/// TODO 添加调用回溯功能
pub struct Stack {
    stack_top : usize,
    stack_bottom : usize,
    last_page : usize,      /// 栈剩余可使用的页面
    is_kernel : bool,
    area : Vec<StackArea>,
}

impl Stack {
    pub fn new(tid : usize, virtual_stack_top : usize, max_page : usize, is_kernel : bool)->Self {
        Self {
            stack_top : virtual_stack_top,
            stack_bottom : virtual_stack_top,
            is_kernel,
            last_page : max_page,
            area : Vec::new(),
        }
    }

    /// 向下扩展栈的逻辑地址，使用物理页拼接并映射
    pub fn expand(&mut self, page_num : usize, satp : &SATP)->Result<(), ()> {
        if page_num > self.last_page {
            return Err(())
        }
        let addr;
        if self.is_kernel {
            addr = kernel_page(page_num);
        }
        else {
            addr = user_page(page_num);
        }
        if let Some(addr) = addr {
            let ved = self.stack_bottom;
            let t = page_num * PAGE_SIZE;
            self.stack_bottom -= t;
            let mut vst = self.stack_bottom;
            let mut pst = addr as usize;
            self.area.push(StackArea::new(vst, ved, pst, pst + t));
            while vst < ved {
                satp.map_data(vst, pst, self.is_kernel);
                vst += PAGE_SIZE;
                pst += PAGE_SIZE;
            }
            self.last_page -= page_num;
            Ok(())
        }
        else {
            Err(())
        }
    }

    /// 拷贝另一个栈，包括栈的大小及内容
    pub fn copy(&mut self, other : &Self, satp : &SATP) {
        self.stack_bottom = self.stack_top - (other.stack_top - other.stack_bottom);
        self.last_page -= (self.stack_top - self.stack_bottom) / PAGE_SIZE;
        let mut vst = self.stack_bottom;
        for area in other.area.iter().rev() {
            let mut a = area.clone();
            a.vst = vst;
            a.ved = vst + (area.ved - area.vst);
            let num = (area.ved - area.vst) / PAGE_SIZE;
            if self.is_kernel {
                a.pst = kernel_page(num).unwrap() as usize;
            }
            else {
                a.pst = user_page(num).unwrap() as usize;
            }
            unsafe {
                (a.pst as *mut u8).copy_from(area.pst as *mut u8, area.ped - area.pst);
            }
            let mut pst = a.pst;
            while vst < a.ved {
                satp.map_data(vst, pst, self.is_kernel);
                vst += PAGE_SIZE;
                pst += PAGE_SIZE;
            }
            a.ped = pst;
            self.area.push(a);
        }
    }

    /// 得到一个虚拟地址在此栈中对应的物理地址，如果不包含这个虚拟地址则 panic
    pub fn virt_to_phy(&self, va:usize)->usize {
        for area in self.area.iter() {
            if area.contain(va) {
                return area.virt_to_phy(va);
            }
        }
        panic!("stack virt to phy err {:x}", va);
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        for area in self.area.iter() {
            free_page(area.pst as *mut u8);
        }
    }
}