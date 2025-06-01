#![allow(non_snake_case)]
extern crate alloc;
mod types;
mod reader;
mod printer;
mod symbols;
mod eval;
mod env;
use std::io::{self, Write};
use crate::types::NekoType;
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
        let mut strs = reader::pre_read_str(&input,&mut symbols);
        let mut results:Vec<String> = Vec::new();
        //println!("{:?}",&strs);
        for st in strs {
            let s = rep(st.as_str(),&mut symbols);
            results.push(st);
            println!("{}",&s);
        }
	io::stdout().flush().unwrap();
    }
}

fn READ(mut s:&str,symb:&mut Symbols) -> NekoType {
    reader::read_str(s,symb)
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
