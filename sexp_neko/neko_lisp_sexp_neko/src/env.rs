use crate::symbols::Symbols;
use crate::types::NekoType;
use crate::types::NekoType::*;
use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};
use core::fmt::Write;
use core::ops::Fn;
use core::mem::discriminant;

pub struct NekoEnv {
    niko_env: BTreeMap<String,Box<dyn Fn(Vec<NekoType>) -> NekoType>>,
}

impl NekoEnv {
    pub fn new() -> NekoEnv {
        NekoEnv {
            niko_env: BTreeMap::new().insert("+".to_string(), Box::new(|v| add(v))),
        }
    }
}

fn add(v:Vec<NekoType>) -> NekoType {
    let result = NekoNil;
    for n in v {
        if result == NekoNil{
            result = n.clone()
        } else if discriminant(&result) == discriminant(&n) {
            match n {
                NekoInt(i) => {
                    let mut ni = 0;
                    match n {}
                    result = NekoInt(i);
                },
                _ => {result = NekoErr("类型不支持".to_string())}
            }
        } else {
            result = NekoErr("无法将不支持的类型相加".to_string());
            break;
        }
    }
    result
}
