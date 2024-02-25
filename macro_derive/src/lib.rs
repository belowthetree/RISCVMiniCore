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

use alloc::{borrow::ToOwned, fmt::format, format, string::{String, ToString}, vec::Vec};
use proc_macro::TokenStream;


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

/// 生成 sbi 函数
/// 函数名、FID、EID、参数...
#[proc_macro]
pub fn generate_sbi_fn(args : TokenStream)->TokenStream {
    let tmp = args.to_string();
    let strs : Vec<&str> = tmp.split(",").collect();
    let func_name = strs[0];
    let fid = strs[1];
    let eid = strs[2];
    let mut input_regs = String::new();
    let mut args_str = String::new();
    for i in 3 .. strs.len() {
        let name : Vec<&str> = strs[i].split(":").collect();
        input_regs += format!("in(\"a{}\") {}, ", i - 3, name[0].trim()).as_str();
        args_str += strs[i];
        if i != strs.len() - 1 {
            args_str += ", ";
        }
    }
    let res = format!("pub fn {}({})->SBIRet {{
        let error : usize;
        let value : usize;
        unsafe {{
            asm!(
                \"li a7, {} \",
                \"li a6, {} \",
                \"ecall\",
                {} lateout(\"a0\") error, lateout(\"a1\") value,
            )
        }}
        SBIRet {{
            error,
            value,
        }}
    }}", func_name, args_str, eid, fid, input_regs);
    res.parse().unwrap()
}

#[proc_macro]
pub fn generate_sbi_extension_exist_fn(args : TokenStream)->TokenStream {
    let t = args.to_string();
    let strs : Vec<&str> = t.split(",").collect();
    let res = format!("pub fn {}()->bool {{
        sbi_probe_extension({}).error == 0
    }}", strs[0], strs[1]);
    res.parse().unwrap()
}
