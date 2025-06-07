use alloc::{vec::Vec, string::String, boxed::Box,rc::Rc};
use core::cell::RefCell;
use hashbrown::HashMap;
use SymbolTypes::*;
use crate::{reader::Reader, types::NekoType,core::*};

#[derive(Debug, Clone, PartialEq,Eq,Hash)]
enum SymbolTypes {
    SymbolChar(char),
    SymbolStr(String),
    SymbolCharList(Vec<char>),
    SymbolStrList(Vec<String>),
    SymbolSpecialChars,
}

#[derive(Clone)]
pub struct Symbols {
    reader_marcos:HashMap<SymbolTypes,NekoType>,
    data:HashMap<String,SymbolTypes>,
}

#[derive(Clone)]
pub struct SymbolRef(Rc<RefCell<Symbols>>);

impl SymbolRef {
    pub fn new() -> SymbolRef {
        let mut symbolref = SymbolRef(Rc::new(RefCell::new(Symbols::new())));
        symbolref.set("s_exp_begin",SymbolChar('('));
        symbolref.set("s_exp_end",SymbolChar(')'));
        symbolref.set("quote_symbol",SymbolChar('"'));
        symbolref.set("change_mean",SymbolChar('\\'));
        symbolref.set("comment_begin",SymbolChar(';'));
        symbolref.set("comment_end",SymbolChar('\n'));
        symbolref.set("keyword",SymbolChar(':'));
        symbolref.set("split",SymbolCharList(vec![' ',',','\n']));
        symbolref.set("special",SymbolSpecialChars);
        symbolref.set_reader_marco(SymbolChar('@'),NekoType::symbol("deref".to_string()));
        return symbolref;
        
    }

    pub fn set(&self,key:&str,val:SymbolTypes) {
        self.0.borrow_mut().data.insert(key.to_string(),val);
    }

    pub fn set_reader_marco(&self,key:SymbolTypes,val:NekoType) {
        self.0.borrow_mut().reader_marcos.insert(key,val);
    }

    pub fn get_reader_marco(&self,key:SymbolTypes) -> Option<NekoType> {
        let s = self.0.borrow();
        let val = s.reader_marcos.get(&key);
        match val {
            Some(n) => Some(n.clone()),
            None => None,
        }
    }

    pub fn get(&self,key:&str) -> Option<SymbolTypes>{
        let s = self.0.borrow();
        let val = s.data.get(&key.to_string());
        match val {
            Some(n) => Some(n.clone()),
            None => None,
        }
    }
    
    pub fn sexp_direction(&self,c:char) -> Option<bool> {
        //如果char是sexp符号，则获取方向的判定。
        if self.pair_char(c,"s_exp_begin") {
            Some(true)
        } else if self.pair_char(c,"s_exp_end") {
            Some(false)
        } else {
            None
        }
    }

    pub fn parse_str_reader_marco(&self,s:&str) -> Option<NekoType> {
        let mut marcos = &self.0.borrow().reader_marcos;
        for marco in marcos.keys() {
            if s.len() > 1 {
                if str_pair(s,marco.clone()){
                    let n = marcos.get(marco).unwrap().clone();
                    return Some(n);
                };
            } else {
                let c = s.chars().next();
                if let Some(nc) = c{
                    if char_pair(nc,marco.clone()) {
                        return self.parse_char_reader_marco(nc);
                    };
                }
            }

        }
        return None;
    }

    pub fn parse_char_reader_marco(&self,c:char) -> Option<NekoType> {
        let mut marcos = &self.0.borrow().reader_marcos;
        for marco in marcos.keys() {
            if char_pair(c,marco.clone()) {
                let n = marcos.get(marco).unwrap().clone();
                return Some(n);
            }
            
        }
        return None;
    }

    pub fn is_str_reader_marco(&self,s:&str) -> bool{
        let r = self.parse_str_reader_marco(s);
        if let Some(_) = r {
            true
        } else {
            false
        }
    }

    pub fn is_char_reader_marco(&self,c:char) -> bool{
        let r = self.parse_char_reader_marco(c);
        if let Some(_) = r {
            true
        } else {
            false
        }
    }

    pub fn pair_str(&self,s:&str,y:&str) -> bool {
        if let Some(symbol) = self.get(y) {
            str_pair(s,symbol)
        } else {
            false
        }
    }
    
    pub fn pair_char(&self,c:char,s:&str) -> bool {
        if let Some(symbol) = self.get(s) {
            char_pair(c,symbol)
        } else {
            false
        }
    }
    
}

impl Symbols {
    pub fn new() -> Symbols {
        Symbols {
            data:HashMap::new(),
            reader_marcos:HashMap::new(),
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
