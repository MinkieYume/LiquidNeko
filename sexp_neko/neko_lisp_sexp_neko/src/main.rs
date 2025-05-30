#![allow(non_snake_case)]
extern crate alloc;
mod reader;
mod printer;
mod types;
mod symbols;
use crate::types::NekoType;
use crate::types::NekoType::*;
use std::io::{self, Write};

fn main() {
    loop{
	let mut input = String::new();
	print!(">>> ");
	io::stdout().flush().unwrap();
	let bytes_read = io::stdin().read_line(&mut input).unwrap();
        if bytes_read == 0{
            println!("\nByeNyan");
            break;
        }
	let s = rep(&input);
	println!("{}",&s);
	io::stdout().flush().unwrap();
    }
}

fn READ(mut s:&str) -> NekoType {
    let mut r = reader::read_str(s);
    return reader::read_form(&mut r);
}

fn EVAL(mut n:NekoType) -> NekoType {
    return n
}

fn PRINT(n:NekoType) -> String {
    return printer::pr_str(n)
}

fn rep(mut s:&str) -> String {
   let mut n = READ(s);
    n = EVAL(n);
    return PRINT(n);
}
