use alloc::{vec::Vec, string::String, boxed::Box};
use SymbolTypes::*;

use crate::reader::Reader;

#[derive(Debug, Clone, PartialEq)]
enum SymbolTypes {
    SymbolChar(char),
    SymbolStr(String),
    SymbolCharList(Vec<char>),
    SymbolStrList(Vec<String>),
    SymbolSpecialChars,
}

pub struct Symbols {
    s_exp_begin:SymbolTypes,
    s_exp_end:SymbolTypes,
    quote_symbol:SymbolTypes,
    change_mean:SymbolTypes,
    comment_begin:SymbolTypes,
    comment_end:SymbolTypes,
    marco_symbols:SymbolTypes,
    marco_str_symbols:SymbolTypes,
    split_symbols:SymbolTypes,
    special_symbols:SymbolTypes,
}

impl Symbols {
    pub fn new() -> Symbols {
        Symbols {
            s_exp_begin : SymbolChar('('),
            s_exp_end : SymbolChar(')'),
            quote_symbol : SymbolChar('"'),
            change_mean : SymbolChar('\\'),
            comment_begin : SymbolChar(';'),
            comment_end : SymbolChar('\n'),
            split_symbols : SymbolCharList(vec![' ',',','\n']),
            marco_symbols : SymbolCharList(vec!['\'','`','~','@','^']),
            marco_str_symbols : SymbolStrList(vec!["~@".to_string(),"@~".to_string()]),
            special_symbols : SymbolSpecialChars,
        }
    }

    pub fn sexp_direction(&mut self,c:char) -> Option<bool> {
        //如果char是sexp符号，则获取方向的判定。
        if self.pair_char(c,"s_exp_begin") {
            Some(true)
        } else if self.pair_char(c,"s_exp_end") {
            Some(false)
        } else {
            None
        }
    }

    pub fn pair_str(&mut self,s:&str,y:&str) -> bool {
        match y {
            "marco_str" => str_pair(s,self.marco_str_symbols.clone()),
            _ => false,
        }
    }
    
    pub fn pair_char(&mut self,c:char,s:&str) -> bool {
        match s {
            "s_exp_begin" => char_pair(c,self.s_exp_begin.clone()),
            "s_exp_end" => char_pair(c,self.s_exp_end.clone()),
            "quote_symbol" => char_pair(c,self.quote_symbol.clone()),
            "change_mean" => char_pair(c,self.change_mean.clone()),
            "comment_begin" => char_pair(c,self.comment_begin.clone()),
            "comment_end" => char_pair(c,self.comment_end.clone()),
            "split" => char_pair(c,self.split_symbols.clone()),
            "special" => char_pair(c,self.special_symbols.clone()),
            "marco_symbols" => char_pair(c,self.marco_symbols.clone()),
            _ => false,
        }
    }
}

fn str_pair(s:&str,y:SymbolTypes) -> bool {
    match y {
        SymbolStr(p) => s == p.as_str(),
        SymbolStrList(pl) => {
            for p in pl{
                if s == p.as_str() {
                    return true;
                }
            }
            return false;
        },
        _ => false,
    }
}

fn char_pair(c:char,s:SymbolTypes) -> bool {
    match s {
        SymbolChar(p) => c == p,
        SymbolCharList(pl) => {
            for p in pl {
                if c == p {
                    return true;
                }
            }
            return false;
        },
        SymbolSpecialChars => c.is_ascii_punctuation(),
        _ => false,
    }
}
