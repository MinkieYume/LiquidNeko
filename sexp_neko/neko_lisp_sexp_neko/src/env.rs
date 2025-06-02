use crate::symbols::Symbols;
use alloc::{boxed::Box,string::String, vec::Vec,rc::Rc};
use core::cell::RefCell;
use hashbrown::HashMap;
use core::fmt::Write;
use crate::types::NekoType;
use crate::types::Symbol;
use crate::types::Function;

#[derive(Clone)]
pub struct EnvType {
    pub outer: Option<Env>,
    pub data: HashMap<Symbol, NekoType>,
}

#[derive(Clone)]
pub struct Env(Rc<RefCell<EnvType>>);

impl Env {
    pub fn new(outer:Option<&Env>) -> Env {
        Env(Rc::new(RefCell::new(EnvType {
            outer: outer.map(|e| e.clone()),
            data: HashMap::new(),
        })))
    }

    pub fn default() -> Env {
        let mut env = Env(Rc::new(RefCell::new(EnvType {
            outer: None,
            data: HashMap::new(),
        })));
        let add = Function {
            boxes:Rc::new(Box::new(|v| Self::add_v(v)))
        };
        env.set(Symbol("+".to_string()),NekoType::func(add));
        env
    }

    fn add_v(mut v:Vec<NekoType>) -> NekoType {
        if v.len() < 1 {
            NekoType::err("参数不足".to_string())
        } else {
            let mut n1 = v.remove(0);
            for n in v {
                n1=n1+n;
            }
            n1                
        }
    }

    pub fn set(&self,key:Symbol, val: NekoType) {
        self.0.borrow_mut().data.insert(key, val);
    }

    pub fn set_by_str(&self, key: &str, val: NekoType) {
        let skey = Symbol(key.to_string());
        self.0.borrow_mut().data.insert(skey, val);
    }

    pub fn get(&self,key:&Symbol) -> NekoType {
        let renv = self.0.borrow();
        let val = renv.data.get(key);
        match val {
            Some(neko) => neko.clone(),
            None => NekoType::err("不存在的键".to_string()),
        }
    }

    pub fn get_by_str(&self,key:&str) -> NekoType {
        let renv = self.0.borrow();
        let val = renv.data.get(&Symbol(key.to_string()));
        match val {
            Some(neko) => neko.clone(),
            None => NekoType::err("获取键失败".to_string()),
        }
    }
}
