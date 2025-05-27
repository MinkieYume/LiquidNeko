#![allow(non_snake_case)]

fn main() {
    rep()
}

fn READ(s:&str) -> &str {
    return s
}

fn EVAL(s:&str) -> &str {
    return s
}

fn PRINT(s:&str) -> &str {
    return s
}

fn rep() {
    let mut s = "hello world";
    s = READ(s);
    s = EVAL(s);
    s = PRINT(s);
    println!("{}",&s);
}
