use alloc::{vec::Vec, string::String, boxed::Box};
use SymbolTypes::*;

#[derive(Debug, Clone, PartialEq)]
enum SymbolTypes {
    SymbolChar(char),
    SymbolStr(str),
    SymbolCharList(Vec<char>),
    SymbolStrList(Vec<str>),
    SymbolSpecialChars,
}

struct Symbols {
    s_exp_begin:SymbolTypes,
    s_exp_end:SymbolTypes,
    quote_symbol:SymbolTypes,
    change_mean:SymbolTypes,
    change_line:SymbolTypes,
    comment_begin:SymbolTypes,
    comment_end:SymbolTypes,
    marco_symbols:SymbolTypes,
    marco_str_symbols:SymbolTypes,
    special_symbols:SymbolTypes,
}

impl Symbols {
    pub fn init(&mut self) {
        self.s_exp_begin = SymbolChar('(');
        self.s_exp_end = SymbolChar(')');
        self.quote_symbol = SymbolChar('"');
        self.change_mean = SymbolChar('\\');
        self.comment_begin = SymbolChar(';');
        self.comment_end = SymbolChar('\n');
        self.marco_str_symbols = SymbolStrList(vec!["~@","@~"]);
        self.special_symbols = SymbolSpecialChars;
    }

    fn sexp_direction(&mut self,c:char) -> Option<bool> {
        //如果char是sexp符号，则获取方向的判定。
        if self.pair_char(c,"s_exp_begin") {
            Some(true)
        } else if self.pair_char(c,"s_exp_end") {
            Some(false)
        } else {
            None
        }
    }

    pub fn pair_str() -> bool {
        
    }
    
    pub fn pair_char(&mut self,c:char,s:&str) -> bool {
        match s {
            "s_exp_begin" => char_pair(c,self.s_exp_begin.clone()),
            "s_exp_end" => char_pair(c,self.s_exp_end.clone()),
            "quote_symbol" => char_pair(c,self.quote_symbol.clone()),
            "change_mean" => char_pair(c,self.change_mean.clone()),
            "comment_begin" => char_pair(c,self.comment_begin.clone()),
            "comment_end" => char_pair(c,self.comment_end.clone()),
            "special" => char_pair(c,self.special_symbols.clone()),
            _ => false,
        }
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
