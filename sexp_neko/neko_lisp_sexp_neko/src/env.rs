use crate::symbols::Symbols;
use alloc::{boxed::Box,string::String, vec::Vec,rc::Rc};
use core::cell::RefCell;
use hashbrown::HashMap;
use core::fmt::Write;
use crate::types::NekoType;

#[derive(Clone)]
pub struct EnvType {
    pub outer: Option<Env>,
    pub data: HashMap<String, NekoType>,
}

#[derive(Clone)]
pub struct Env(Rc<RefCell<EnvType>>);

