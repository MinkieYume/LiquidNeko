#![allow(non_snake_case)]
extern crate alloc;
mod reader;
mod printer;
mod types;
mod symbols;
use std::io::{self, Write};
use crate::types::NekoType;
use crate::types::NekoType::*;
use crate::symbols::Symbols;

fn main() {
    let mut symbols = Symbols::new();
    loop{
	let mut input = String::new();
	print!(">>> ");
	io::stdout().flush().unwrap();
	let bytes_read = io::stdin().read_line(&mut input).unwrap();
        if bytes_read == 0{
            println!("\nByeNyan");
            break;
        }
	let s = rep(&input,&mut symbols);
	println!("{}",&s);
	io::stdout().flush().unwrap();
    }
}

fn READ(mut s:&str,symb:&mut Symbols) -> NekoType {
    if let Some(mut r) = reader::read_str(s,symb) {
        reader::read_form(&mut r,symb)
    } else {
        NekoNil
    }
}

fn EVAL(mut n:NekoType,symb:&mut Symbols) -> NekoType {
    return n
}

fn PRINT(n:NekoType,symb:&mut Symbols) -> String {
    return printer::pr_str(n)
}

fn rep(mut s:&str,symb:&mut Symbols) -> String {
   let mut n = READ(s,symb);
    n = EVAL(n,symb);
    return PRINT(n,symb);
}
