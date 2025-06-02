use alloc::{vec::Vec, string::String, boxed::Box,collections::BTreeMap};
use core::fmt::Write;
use std::borrow::Borrow;
use crate::types::NekoType;
use crate::types::NekoValue;
use crate::types::NekoValue::*;
use crate::symbols::Symbols;
use crate::env::Env;

pub fn eval_ast(mut n:NekoType,env:&mut Env) -> NekoType {
    //对单个参数进行求值
    match n.get_ref().borrow() {
        NekoSymbol(s) => {env.get(&s)},
        NekoList(v) => {
            let mut nv:Vec<NekoType> = Vec::new();
            for e in v {
                let mut ne = eval(e.clone(),env);
                nv.push(ne);
            }
            NekoType::list(nv)
        },
        _ => n,
    }
}

pub fn eval(mut n:NekoType,env:&mut Env) -> NekoType {
    //对参数进行执行操作
    match n.get_ref().borrow() {
        NekoList(v) => {
            if !v.is_empty() {
                let mut nn = eval_ast(n,env);
                if let NekoList(mut nv) = nn.get_value() {
                    let mut nekofn = nv.remove(0);
                    let mut args:Vec<NekoType> = Vec::new();
                    if let NekoFn(f) = nekofn.get_value() {
                        for cn in nv {
                            args.push(cn);
                        }
                        return f.call(args);
                    }
                }
                return nn;
                
            } else {
                n
            }
        },
        _ => eval_ast(n,env)
    }
}
