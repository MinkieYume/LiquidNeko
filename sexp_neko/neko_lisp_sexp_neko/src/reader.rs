// heapless Vec：适用于 no_std 裸机
#[cfg(feature = "use_heapless")]
pub mod types {
    use heapless::{Vec, String as Str};
    pub type String = Str<64>;
    pub type StrVec = Vec<String, 64>;
}

// alloc Vec：适用于有 allocator 的 no_std
#[cfg(all(feature = "use_alloc", not(feature = "use_std")))]
pub mod types {
    extern crate alloc;
    use alloc::{vec::Vec, string::String as Str};
    pub type String = Str;
    pub type StrVec = Vec<String>;
}

// std Vec：适用于标准系统
#[cfg(feature = "use_std")]
pub mod types {
    use std::{vec::Vec, string::String as Str};
    pub type String = Str;
    pub type StrVec = Vec<String>;
}

use types::{String,StrVec};

pub struct Reader {
    tokens: StrVec,
    position: usize,
}

impl Reader {
    pub fn peek(&self) -> Option<String> {
        if self.tokens.len() > self.position {
            Some(self.tokens[self.position].to_owned())
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<String> {
        if let Some(token) = self.peek() {
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }
}

struct ReaderChars{
    s_exp_begin_char:char,
    s_exp_end_char:char,
    quote_begin_char:char,
    quote_end_char:char,
}

impl ReaderChars{
    pub fn init(&mut self) {
        self.s_exp_begin_char = '(';
        self.s_exp_end_char = ')';
        self.quote_begin_char = '"';
        self.quote_end_char = '"';
    }
    
    // Getter methods
    pub fn get_s_exp_begin_char(&self) -> char {
        self.s_exp_begin_char
    }

    pub fn get_s_exp_end_char(&self) -> char {
        self.s_exp_end_char
    }

    pub fn get_quote_begin_char(&self) -> char {
        self.quote_begin_char
    }

    pub fn get_quote_end_char(&self) -> char {
        self.quote_end_char
    }

    // Setter methods
    pub fn set_s_exp_begin_char(&mut self, c: char) {
        self.s_exp_begin_char = c;
    }

    pub fn set_s_exp_end_char(&mut self, c: char) {
        self.s_exp_end_char = c;
    }

    pub fn set_quote_begin_char(&mut self, c: char) {
        self.quote_begin_char = c;
    }

    pub fn set_quote_end_char(&mut self, c: char) {
        self.quote_end_char = c;
    } 
}

pub fn read_str(s:&str) {
    tokenize(s);
}

pub fn tokenize(s:&str) {
    let mut tokens:StrVec = Vec::new();
    let mut token:String = String::new();
    let mut quoting = false;
    for c in s.chars() {
        // 逐个处理每个字符
        println!("Character: {}", c);
        if token == "~@"{
            tokens.push(token.clone());
            token = String::new();
        }
        if is_char_special_char(&c) {
            token.push(c);
            tokens.push(token.clone());
            token = String::new();
        }
        if is_char_quote(&c) {
            token.push(c);
            if quoting {
                tokens.push(token.clone());
                token = String::new();
            }
            quoting = !quoting;
        }
        if !token.is_empty() {
            if token != "~@"{
                tokens.push(token.clone());
            }
        }
    }
}

fn is_char_special_char(c:&char) -> bool{
    match c {
        '(' | ')' | '{' | '}' | '\'' | '`' | '~' | '@' | '^' => true,
        _ => false,
    }
}

fn is_char_quote(c:&char) -> bool{
    if *c == '"' {
        true
    }else{
        false
    }
}

pub fn read_form() {
//解析Reader的第一个token，并判断是否处理接下来的内容。
}

pub fn read_list() {

}

pub fn read_atom() {

}
