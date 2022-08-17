#![no_std]
#![no_main]
#![feature(asm_sym)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use alloc::format;
use ckb_std::cstr_core::CStr;
use ckb_std::{default_alloc, syscalls::debug};
use core::arch::asm;
use core::slice::from_raw_parts;

ckb_std::entry!(program_entry);
default_alloc!();

pub fn program_entry(argc: u64, argv: *const *const u8) -> i8 {
    if argc <= 2 {
        let args = unsafe { from_raw_parts(argv, argc as usize) };
        let arg1 = unsafe { CStr::from_ptr(args[1]) };
        let arg1 = arg1.to_str().unwrap();
        let n = i32::from_str_radix(&arg1, 10).expect("parsing");
        let result = fib(n);
        debug(format!("Result: {}", result));
        (result + 3) as i8
    } else {
        -1
    }
}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}
