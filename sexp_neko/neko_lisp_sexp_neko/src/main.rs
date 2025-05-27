#![allow(non_snake_case)]
use std::io;

fn main() {
    loop{
	let mut input = String::new();
	print!(">>> ");
	let mut s = io::stdin().read_line(&mut input).unwrap();
	s = rep(s);
	println!("{}",&s);
    }
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
