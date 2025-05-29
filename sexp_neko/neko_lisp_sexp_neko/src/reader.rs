extern crate alloc;
use alloc::{vec::Vec, string::String};
use crate::types::NekoType;
use crate::types::NekoType::*;

pub struct Reader {
    tokens: Vec<String>,
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

pub fn read_str(s:&str) -> Reader{
    let mut reader = Reader {
        tokens: tokenize(s),
        position: 0,
    };
    return reader;
}

pub fn tokenize(s:&str) -> Vec<String>{
    let mut last_tokens:Vec<String> = Vec::new();
    let mut tokens:Vec<String> = Vec::new();
    let mut token:String = String::new();

    //循环1,处理空格、逗号、引号、分号与换行
    let mut last_char = ' ';
    let mut quoting = false;
    let mut commenting = false;
    for c in s.chars() {
        if is_char_quote(c) {
            //处理引号
            token.push(c);  // quote 本身加进去
            if !is_char_change_mean(last_char){
                quoting = !quoting;
            }

            // 如果刚结束字符串，提交 token
            if !quoting {
                tokens.push(token.clone());
                token.clear();
            }
            continue;
        } else if is_char_comment_symbol(c) && !quoting {
            //处理分号
            tokens.push(token.clone());
            token.clear();
            token.push(c);
            commenting = true;
            continue;
        } else if is_char_change_line(c) && !quoting {
            //处理换行符
            commenting = false;   
        }
        
        if quoting || commenting {
            // 在引号或注释时，照单全收
            token.push(c);
        } else if is_char_trim_symbol(c) {
            // 分隔符，提交 token（如果非空）
            if !token.is_empty() {
                tokens.push(token.clone());
                token.clear();
            }
        } else {
            // 普通字符
            token.push(c);
        }
        //println!("{}",c);
        if is_char_change_mean(c) && is_char_change_mean(last_char){
            last_char = ' ';
        } else{
            last_char = c;
        }
    }
    // 处理最后一个 token
    if !token.is_empty() {
        tokens.push(token.clone());
        token.clear();
    }
     // 准备下一组匹配循环
    last_tokens = tokens.clone();
    tokens.clear();
    last_char = ' ';

    //循环2:分离宏字符与普通字符（特殊的宏字符与普通字符区分）
    for last_token in last_tokens {
        let mut pushed = false;
        for c in last_token.chars() {
            if is_char_quote(c) || is_char_comment_symbol(c) {
                //判断是否是引号或注释符号，是的话忽略本token
                tokens.push(last_token.clone());
                pushed = true;
                break;
            } else if is_char_sexp_char(c) {
                //匹配S表达式符号（括号）并单独分开
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
                token.push(c);
                tokens.push(token.clone());
                token.clear();
            } else if is_char_marco_symbol(c) {
                //匹配宏符号（其它ASCII标准的特殊符号）
                token.push(c);
            } else {
                //匹配普通字符
                if is_char_marco_symbol(last_char) {
                    tokens.push(token.clone());
                    token.clear();
                }
                token.push(c);
            }
            last_char = c;
        }
        if !pushed {
            if !token.is_empty() {
                tokens.push(token.clone());
                token.clear();
            }
        }
    }
    
    //println!("{:?}", tokens);

     // 准备下一组匹配循环
    last_tokens = tokens.clone();
    tokens.clear();
    last_char = ' ';
    // 循环3:区分单字符与宏字符组合
    for last_token in last_tokens {
        if is_string_marco_symbol(&last_token) {
            tokens.push(last_token.clone());
        } else {
            let mut pushed = false;
            for c in last_token.chars(){
                if is_char_quote(c) || is_char_comment_symbol(c) {
                    //判断是否是引号或注释符号，是的话忽略本token
                    tokens.push(last_token.clone());
                    pushed = true;
                    break;
                } else if is_char_marco_symbol(c) {
                    token.push(c);
                    tokens.push(token.clone());
                    token.clear();
                    pushed = true; //前面已经处理过字母了，所以只要一个字符是符号，后面也必定是
                }
            }
            if !pushed && !last_token.is_empty() {
                tokens.push(last_token.clone());
                token.clear();
            }
        }
        
    }
    
    println!("{:?}", tokens);
    return tokens;
}

fn is_char_sexp_char(c:char) -> bool {
    match c{
        '(' | ')' => true,
        _ => false,
    }
}

fn if_char_sexp_with_direction(c:char) -> Option<bool> {
    //如果char是sexp符号，则获取方向的判定。
    match c {
        '(' => Some(true),
        ')' => Some(true),
        _ => None
    }
}

fn is_char_change_mean(c:char) -> bool {
    if c == '\\' {
        true
    }else{
        false
    }
}

fn is_char_comment_symbol(c:char) -> bool {
    if c == ';' {
        true
    }else{
        false
    }
}

fn is_char_change_line(c:char) -> bool {
    match c {
        '\n' => true,
        _ => false,
    }
}

fn is_char_marco_symbol(c:char) -> bool{
    if c.is_ascii_punctuation() {
        true
    }else {
        false
    }
}

fn is_string_marco_symbol(s:&String) -> bool {
    match s.as_str() {
        "~@" | "@~" => true,
        _ => false,
    }
}

fn is_char_trim_symbol(c:char) -> bool {
    match c {
        '\n' | ' ' | ',' => true,
        _ => false,
    }
}

fn is_char_special_char(c:char) -> bool{
    match c {
        '(' | ')' | '{' | '}' | '\'' | '`' | '~' | '@' | '^' => true,
        _ => false,
    }
}

fn is_char_quote(c:char) -> bool{
    if c == '"' {
        true
    }else{
        false
    }
}

pub fn read_form(r:&mut Reader) -> NekoType {
    //解析Sexp表达式形式
    if let Some(c) = r.peek().and_then(|token| token.chars().next()) {
        if let Some(true) = if_char_sexp_with_direction(c) {
            return read_list(r);
        } else {
            return read_atom(r);
        }
    } else {
        return NekoNil;
    }
}

pub fn read_list(r:&mut Reader) -> NekoType {
    //解析Sexp表达式本身
    let mut list:Vec<NekoType> = Vec::new();
    //反复读入元素：
    while let Some(token) = r.next() {
        let c = token.chars().next().unwrap();
        if let Some(false) = if_char_sexp_with_direction(c) {
            return NekoList(list);
        }
        list.push(read_form(r));
    }
    return NekoErr("Sexp表达式没有结尾".to_string());
}

pub fn read_atom(r:&mut Reader) -> NekoType {
    //解析Sexp表达式内容
    let s = r.peek().unwrap();
    if let Some(i) = parse_integer(s.as_str()) {
        NekoInt(i)
    } else {
        NekoSymbol(s)
    }
}

fn parse_integer(s: &str) -> Option<i64> {
    s.parse::<i64>().ok()
}
