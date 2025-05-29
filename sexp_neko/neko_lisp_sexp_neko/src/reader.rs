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
    let mut last_tokens:StrVec = Vec::new();
    let mut tokens:StrVec = Vec::new();
    let mut token:String = String::new();
    let mut quoting = false;
    let mut char_pos = 0;

    //循环1,处理空格、逗号与引号
    for c in s.chars(){
        if is_char_quote(&c) {
            token.push(c);  // quote 本身加进去
            quoting = !quoting;

            // 如果刚结束字符串，提交 token
            if !quoting {
                tokens.push(token.clone());
                token.clear();
            }
            continue;
        }

        if quoting {
            // 在引号内，照单全收
            token.push(c);
        } else if c == ' ' || c == ',' {
            // 分隔符，提交 token（如果非空）
            if !token.is_empty() {
                tokens.push(token.clone());
                token.clear();
            }
        } else {
            // 普通字符
            token.push(c);
        }
        println!("{}",c)
        
    }
    // 处理最后一个 token
    if !token.is_empty() {
        tokens.push(token.clone());
        token.clear();
    }
     // 准备下一组匹配循环
    last_tokens = tokens.clone();
    tokens.clear();

    //循环2:分离宏字符与普通字符（特殊的宏字符与普通字符区分）
    for last_token in last_tokens {
        let mut pushed = false;
        for c in last_token.chars() {
            if is_char_quote(&c) {
                tokens.push(last_token.clone());
                pushed = true;
                break;
            }else if is_char_sexp_char(&c) {
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
                token.push(c);
                tokens.push(token.clone());
                token.clear();
            }else if is_char_marco_symbol(&c) {
                token.push(c);
            }
        }
        if !pushed {
            if !token.is_empty() {
                tokens.push(token.clone());
                token.clear();
            }
        }
    }
    
    println!("{:?}", tokens);
    
/*    for c in s.chars() {
        // 逐个处理每个字符，传统的办法行不通，也许要递归。
        if c != ' ' && c != ',' {
            token.push(c);
        }else{
            tokens.push(token.clone());
            token = String::new();
        }*/
/*        if is_char_quote(&c) {
            if quoting {
                tokens.push(token.clone());
                token = String::new();
            }
            quoting = !quoting;
        }
        if token == "~@"{
            tokens.push(token.clone());
            token = String::new();
        }
        if is_char_sexp_char(&c) {
            tokens.push(token.clone());
            token = String::new();
        }
        if c == ' ' || c == ','{
            */
    }

fn is_char_sexp_char(c:&char) -> bool {
    match c{
        '(' | ')' => true,
        _ => false,
    }
}

fn is_char_marco_symbol(c:&char) -> bool{
    if c.is_ascii_punctuation() {
        true
    }else {
        false
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
