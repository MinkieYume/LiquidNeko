#![allow(non_snake_case)]
mod reader;
mod printer;
use std::io::{self, Write};

fn main() {
    let s = "(+ \"~@123 $^n 456\" ~@aadx)";
    reader::read_str(&s);
/*    loop{
	let mut input = String::new();
	print!(">>> ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut input).unwrap();
	let s = rep(&input);
	println!("{}",&s);
	io::stdout().flush().unwrap();
    }*/
}

fn READ(mut s:&str) -> &str {
    return s
}

fn EVAL(mut s:&str) -> &str {
    return s
}

fn PRINT(mut s:&str) -> &str {
    return s
}

fn rep(mut s:&str) -> &str {
    s = READ(s);
    s = EVAL(s);
    s = PRINT(s);
    return s
}
