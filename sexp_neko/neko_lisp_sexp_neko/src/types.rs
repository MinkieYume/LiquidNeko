use alloc::{vec::Vec, string::String, boxed::Box ,rc::Rc};
use core::ops::{Add,Sub,Mul,Div,Fn,Deref};
use core::cmp::{Eq,PartialEq};
use core::mem::discriminant;
use core::cell::RefCell;
use std::borrow::Borrow;
use NekoValue::*;

#[derive(Clone)]
pub enum NekoValue {
    NekoSymbol(Symbol),
    Nekoi64(i64),
    Nekof64(f64),
    NekoString(String),
    NekoChar(char),
    NekoBool(Option<bool>),
    NekoKeyword(String),
    NekoList(Vec<NekoType>),
    NekoFn(Function),
    NekoErr(String),
    NekoTrue,
    NekoFalse,
    NekoNil
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Symbol(pub String);

#[derive(Clone)]
pub struct NekoType(pub Rc<NekoValue>);

#[derive(Clone)]
pub struct Function {
    pub boxes:Rc<Box<dyn Fn(Vec<NekoType>) -> NekoType>>,
}

impl NekoType {
    pub fn nil() -> NekoType {
        NekoType(Rc::new(NekoNil))
    }

    pub fn is_nil(&self) -> bool {
        match *self.0 {
            NekoNil => true,
            _ => false,
        }
    }

    pub fn symbol(s:String) -> NekoType {
        NekoType(Rc::new(NekoSymbol(Symbol(s))))
    }

    pub fn from_symbol(s:Symbol) -> NekoType {
        NekoType(Rc::new(NekoSymbol(s)))
    }

    pub fn is_symbol(&self) -> bool {
        match *self.0 {
            NekoSymbol(_) => true,
            _ => false,
        }
    }

    pub fn int_64(i:i64) -> NekoType {
        NekoType(Rc::new(Nekoi64(i)))
    }

    pub fn is_int(&self) -> bool {
        match *self.0 {
            Nekoi64(_) => true,
            _ => false,
        }
    }
    
    pub fn float_64(f:f64) -> NekoType {
        NekoType(Rc::new(Nekof64(f)))
    }

    pub fn is_float(&self) -> bool {
        match *self.0 {
            Nekof64(_) => true,
            _ => false,
        }
    }

    pub fn string(s:String) -> NekoType {
        NekoType(Rc::new(NekoString(s)))
    }

    pub fn is_string(&self) -> bool {
        match *self.0 {
            NekoString(_) => true,
            _ => false,
        }
    }

    pub fn neko_char(c:char) -> NekoType {
        NekoType(Rc::new(NekoChar(c)))
    }

    pub fn is_char(&self) -> bool {
        match *self.0 {
            NekoChar(_) => true,
            _ => false,
        }
    }

    pub fn list(v:Vec<NekoType>) -> NekoType {
        NekoType(Rc::new(NekoList(v)))
    }

    pub fn is_list(&self) -> bool {
        match *self.0 {
            NekoList(_) => true,
            _ => false,
        }
    }

    pub fn keyword(s:String) -> NekoType {
        NekoType(Rc::new(NekoKeyword(s)))
    }

    pub fn is_keyword(&self) -> bool {
        match *self.0 {
            NekoKeyword(_) => true,
            _ => false,
        }
    }

    pub fn err(s:String) -> NekoType {
        NekoType(Rc::new(NekoErr(s)))
    }

    pub fn is_err(&self) -> bool {
        match *self.0 {
            NekoErr(_) => true,
            _ => false,
        }
    }

    pub fn neko_bool(o:Option<bool>) -> NekoType {
        NekoType(Rc::new(NekoBool(o)))
    }

    pub fn is_bool(&self) -> bool {
        match *self.0 {
            NekoBool(_) => true,
            _ => false,
        }
    }

    pub fn neko_true() -> NekoType {
        NekoType(Rc::new(NekoTrue))
    }

    pub fn is_true(&self) -> bool {
        match *self.0 {
            NekoTrue => true,
            _ => false,
        }
    }
    
    pub fn neko_false() -> NekoType {
        NekoType(Rc::new(NekoFalse))
    }
    
    pub fn is_false(&self) -> bool {
        match *self.0 {
            NekoFalse => true,
            _ => false,
        }
    }

    pub fn func(func:Function) -> NekoType {
        NekoType(Rc::new(NekoFn(func)))
    }

    pub fn is_fn(&self) -> bool{
        match *self.0 {
            NekoFn(_) => true,
            _ => false,
        }
    }

    pub fn get_type(&self) -> NekoType {
        let s:&str = self.get_type_str();
        NekoType(Rc::new(NekoString(s.to_string())))
    }

    pub fn get_type_str(&self) -> &str {
        match *self.0 {
            NekoList(_) => "list",
            NekoBool(_) => "bool",
            NekoChar(_) => "char",
            NekoSymbol(_) => "symbol",
            Nekoi64(_) => "int",
            Nekof64(_) => "float",
            NekoString(_) => "string",
            NekoErr(_) => "err",
            NekoKeyword(_) => "keyword",
            NekoTrue => "true",
            NekoFalse => "false",
            NekoNil => "nil",
            _ => "unknown",
        }
    }

    pub fn get_value(&self) -> NekoValue {
        return (*self.0).clone()
    }

    pub fn get_ref(&self) -> Rc<NekoValue> {
        return self.0.clone()
    }
}

impl Symbol {
    pub fn val(&self) -> String {
        self.0.clone()
    }
}

impl Function {
    pub fn call(&self,v:Vec<NekoType>) -> NekoType {
        (*self.boxes)(v)
    }
}

impl Add for NekoType {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        match self.get_value() {
            NekoList(mut a) => {
                if let NekoList(mut b) = other.get_value() {
                    a.append(&mut b);
                    return Self::list(a);
                }
            },
            NekoSymbol(a) => {
                if let NekoSymbol(b) = other.get_value() {
                    let mut c = a.val();
                    c.push_str(b.val().as_str());
                    return Self::symbol(c);
                }
            },
            Nekoi64(a) => {
                if let Nekoi64(b) = other.get_value() {
                    return Self::int_64(a+b);
                }
            },
            Nekof64(a) => {
                if let Nekof64(b) = other.get_value() {
                    return Self::float_64(a+b);
                }
            },
            NekoString(mut a) => {
                if let NekoString(b) = other.get_value() {
                    a.push_str(b.as_str());
                    return Self::string(a);
                }
            },
            NekoKeyword(mut a) => {
                if let NekoKeyword(b) =  other.get_value() {
                    a.push_str(b.as_str());
                    return Self::keyword(a);
                }
            }
            _ => {}
        }
        Self::err("不支持的操作".to_string())
    }
}
