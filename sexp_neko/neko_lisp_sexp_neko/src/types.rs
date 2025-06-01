use alloc::{vec::Vec, string::String, boxed::Box ,rc::Rc};
use core::cell::RefCell;

#[derive(Clone)]
pub enum NekoValue {
    NekoSymbol(String),
    NekoInt(i64),
    NekoFloat(f64),
    NekoString(String),
    NekoChar(char),
    NekoBool(Option<bool>),
    NekoKeyword(String),
    NekoList(Vec<NekoType>),
    NekoErr(String),
    NekoTrue,
    NekoFalse,
    NekoNil
}

#[derive(Clone)]
pub struct NekoType(pub Rc<NekoValue>);

impl NekoType {
    pub fn nil() -> NekoType {
        NekoType(Rc::new(NekoValue::NekoNil))
    }

    pub fn is_nil(&self) -> bool {
        match *self.0 {
            NekoValue::NekoNil => true,
            _ => false,
        }
    }

    pub fn symbol(s:String) -> NekoType {
        NekoType(Rc::new(NekoValue::NekoSymbol(s)))
    }

    pub fn is_symbol(&self) -> bool {
        match *self.0 {
            NekoValue::NekoSymbol(_) => true,
            _ => false,
        }
    }

    pub fn int(i:i64) -> NekoType {
        NekoType(Rc::new(NekoValue::NekoInt(i)))
    }

    pub fn is_int(&self) -> bool {
        match *self.0 {
            NekoValue::NekoInt(_) => true,
            _ => false,
        }
    }
    
    pub fn float(f:f64) -> NekoType {
        NekoType(Rc::new(NekoValue::NekoFloat(f)))
    }

    pub fn is_float(&self) -> bool {
        match *self.0 {
            NekoValue::NekoFloat(_) => true,
            _ => false,
        }
    }

    pub fn string(s:String) -> NekoType {
        NekoType(Rc::new(NekoValue::NekoString(s)))
    }

    pub fn is_string(&self) -> bool {
        match *self.0 {
            NekoValue::NekoString(_) => true,
            _ => false,
        }
    }

    pub fn neko_char(c:char) -> NekoType {
        NekoType(Rc::new(NekoValue::NekoChar(c)))
    }

    pub fn is_char(&self) -> bool {
        match *self.0 {
            NekoValue::NekoChar(_) => true,
            _ => false,
        }
    }

    pub fn list(v:Vec<NekoType>) -> NekoType {
        NekoType(Rc::new(NekoValue::NekoList(v)))
    }

    pub fn is_list(&self) -> bool {
        match *self.0 {
            NekoValue::NekoList(_) => true,
            _ => false,
        }
    }

    pub fn keyword(s:String) -> NekoType {
        NekoType(Rc::new(NekoValue::NekoKeyword(s)))
    }

    pub fn is_keyword(&self) -> bool {
        match *self.0 {
            NekoValue::NekoKeyword(_) => true,
            _ => false,
        }
    }

    pub fn err(s:String) -> NekoType {
        NekoType(Rc::new(NekoValue::NekoErr(s)))
    }

    pub fn is_err(&self) -> bool {
        match *self.0 {
            NekoValue::NekoErr(_) => true,
            _ => false,
        }
    }

    pub fn neko_bool(o:Option<bool>) -> NekoType {
        NekoType(Rc::new(NekoValue::NekoBool(o)))
    }

    pub fn is_bool(&self) -> bool {
        match *self.0 {
            NekoValue::NekoBool(_) => true,
            _ => false,
        }
    }

    pub fn neko_true() -> NekoType {
        NekoType(Rc::new(NekoValue::NekoTrue))
    }

    pub fn is_true(&self) -> bool {
        match *self.0 {
            NekoValue::NekoTrue => true,
            _ => false,
        }
    }
    
    pub fn neko_false() -> NekoType {
        NekoType(Rc::new(NekoValue::NekoFalse))
    }
    
    pub fn is_false(&self) -> bool {
        match *self.0 {
            NekoValue::NekoFalse => true,
            _ => false,
        }
    }

    pub fn get_type(&self) -> NekoType {
        let s:&str = self.get_type_str();
        NekoType(Rc::new(NekoValue::NekoString(s.to_string())))
    }

    pub fn get_type_str(&self) -> &str {
        match *self.0 {
            NekoValue::NekoList(_) => "list",
            NekoValue::NekoBool(_) => "bool",
            NekoValue::NekoChar(_) => "char",
            NekoValue::NekoSymbol(_) => "symbol",
            NekoValue::NekoInt(_) => "int",
            NekoValue::NekoFloat(_) => "float",
            NekoValue::NekoString(_) => "string",
            NekoValue::NekoErr(_) => "err",
            NekoValue::NekoKeyword(_) => "keyword",
            NekoValue::NekoTrue => "true",
            NekoValue::NekoFalse => "false",
            NekoValue::NekoNil => "nil",
            _ => "unknown",
        }
    }

    pub fn get_value(&self) -> NekoValue {
        return (*self.0).clone()
    }
}
