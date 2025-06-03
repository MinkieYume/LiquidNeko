use crate::symbols::Symbols;
use alloc::{boxed::Box,string::String, vec::Vec,rc::Rc};
use core::cell::RefCell;
use hashbrown::HashMap;
use core::fmt::Write;
use crate::core::Core;
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

    pub fn with_bindings(outer:Option<&Env>,mut bindings:HashMap<Symbol, NekoType>) -> Env {
        let mut env = Env(Rc::new(RefCell::new(EnvType {
            outer: outer.map(|e| e.clone()),
            data: HashMap::new(),
        })));
        let keys = bindings.keys();
        for bind in keys {
            if let Some(mut val) = bindings.get(bind) {
                env.set(bind.clone(),val.clone());
            }
            
        }
        env
    }

    pub fn default() -> Env {
        let core = Core::default();
        Env::with_bindings(None,core.binddings)
    }

    pub fn set(&self,key:Symbol, val: NekoType) {
        self.0.borrow_mut().data.insert(key, val);
    }

    pub fn set_by_str(&self, key: &str, val: NekoType) {
        let skey = Symbol(key.to_string());
        self.0.borrow_mut().data.insert(skey, val);
    }

    pub fn find(&self,key:&Symbol) -> Option<Env> {
        let renv = self.0.borrow();
        let val = renv.data.get(key);
        match val {
            Some(_) => Some(self.clone()),
            None => {
                if let Some(o) = &renv.outer {
                    return Some(o.clone());
                } else {
                    return None
                }
            },
        }
    }

    pub fn get(&self,key:&Symbol) -> NekoType {
        let o_env = self.find(key);
        match o_env {
            Some(env) => {
                let renv = env.0.borrow();
                let val = renv.data.get(key);
                match val {
                    Some(neko) => neko.clone(),
                    None => NekoType::err("环境中不存在该键".to_string()),
                }
            },
            None => NekoType::err("环境中不存在该键".to_string()),
        }
    }

    pub fn get_by_str(&self,key:&str) -> NekoType {
        self.get(&Symbol(key.to_string()))
    }
}
