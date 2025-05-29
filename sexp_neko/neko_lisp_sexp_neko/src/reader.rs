extern crate alloc;
use alloc::{vec::Vec, string::String};

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
        println!("{}",c);
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
    
    println!("{:?}", tokens);

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
}

fn is_char_sexp_char(c:char) -> bool {
    match c{
        '(' | ')' => true,
        _ => false,
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

pub fn read_form() {
//解析Reader的第一个token，并判断是否处理接下来的内容。
}

pub fn read_list() {

}

pub fn read_atom() {

}
