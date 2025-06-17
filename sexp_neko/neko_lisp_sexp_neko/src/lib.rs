#![allow(non_snake_case)]
#![no_std]
#![no_main]
extern crate alloc;
mod types;
mod reader;
mod printer;
mod symbols;
mod eval;
mod env;
mod nekocore;
use alloc::alloc::{GlobalAlloc, Layout};
use alloc::string::String;
use mimalloc_rust::*;
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

//use std::io::{self, Write};

/*fn main() {
    let mut env = Env::default();
    loop {
	let mut input = String::new();
	print!(">>> ");
	io::stdout().flush().unwrap();
	let bytes_read = io::stdin().read_line(&mut input).unwrap();
        if bytes_read == 0{
            println!("\nByeNyan");
            break;
        }
        let mut strs = reader::pre_read_str(&input,env.clone());
        let mut results:Vec<String> = Vec::new();
        //println!("{:?}",&strs);
        for st in strs {
            let s = rep(st.as_str(),env.clone());
            results.push(st);
            println!("{}",&s);
        }
	io::stdout().flush().unwrap();
    }
}*/

fn READ(mut s:&str,env:Env) -> NekoType {
    reader::read_str(s,env.clone())
}

fn EVAL(n:NekoType,env:Env) -> NekoType {
    eval::eval(n,env.clone())
}

fn PRINT(n:NekoType,env:Env) -> String {
    return printer::pr_str(n)
}

fn rep(mut s:&str,env:Env) -> String {
   let mut n = READ(s,env.clone());
    n = EVAL(n,env.clone());
    return PRINT(n,env.clone());
}
