//! # 宏 crate
//! 2022年10月29日

#![no_std]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

extern crate proc_macro;
extern crate alloc;

#[macro_use]
mod cpu;
use alloc::{borrow::ToOwned, string::ToString};
use proc_macro::TokenStream;


#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

/// 生成 CSR 寄存器写入函数
#[proc_macro]
pub fn generate_write_csr_fn(name : TokenStream)->TokenStream {
    let t = "pub fn write_".to_owned() + name.to_string().as_str() + "(v : usize) {
        unsafe {
            asm!(
                \"mv t0, {val}
                csrw " + name.to_string().as_str() + ", t0
                \",
                val = in(reg) v
            )
        }
    }";
    t.parse().unwrap()
}

/// 生成 CSR 寄存器读取函数
#[proc_macro]
pub fn generate_read_csr_fn(name : TokenStream)->TokenStream {
    let t = "pub fn read_".to_owned() + name.to_string().as_str() + "()->usize {
        let mut res : usize = 0;
        unsafe {
            asm!(
                \"csrr {val}, " + name.to_string().as_str() + "\",
                val = out(reg) res
            )
        }
        return res;
    }";
    t.parse().unwrap()
}
