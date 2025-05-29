#![allow(non_snake_case)]
mod reader;
mod printer;
mod types;
use crate::types::NekoType;
use crate::types::NekoType::*;
use std::io::{self, Write};

fn main() {
    loop{
	let mut input = String::new();
	print!(">>> ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut input).unwrap();
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
