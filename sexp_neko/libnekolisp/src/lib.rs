#![no_std]
#![no_main]
#![allow(non_snake_case)]
extern crate alloc;
mod types;
mod reader;
mod printer;
mod symbols;
mod eval;
mod env;
mod nekocore;
use alloc::alloc::{GlobalAlloc, Layout};
use alloc::vec::Vec;
use alloc::string::String;
use mimalloc_rust::*;
use core::fmt::Write;
use ::core::panic::PanicInfo;
use crate::types::NekoType;
use crate::env::Env;
use mimalloc_rust::*;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}  // 在发生 panic 时进入死循环，通常意味着程序崩溃
}

#[unsafe(no_mangle)]
pub extern "C" fn rep_str(mut s:&str) -> Vec<String> {
    let mut env = Env::default();
    let mut strs = reader::pre_read_str(&s,env.clone());
    let mut results:Vec<String> = Vec::new();
    //println!("{:?}",&strs);
    for st in strs {
        let s = rep(st.as_str(),env.clone());
        results.push(st);
    }
    return results;
}

fn rep(mut s:&str,env:Env) -> String {
    let mut n = reader::read_str(s,env.clone());
    n = eval::eval(n,env.clone());
    return printer::pr_str(n);
}
