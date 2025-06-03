use alloc::{vec::Vec, string::String, boxed::Box,collections::BTreeMap};
use core::fmt::Write;
use std::borrow::Borrow;
use crate::types::NekoType;
use crate::types::NekoValue;
use crate::types::NekoValue::*;
use crate::types::Symbol;
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
    //对参数进行判定并决定是否执行或应用
    match n.get_ref().borrow() {
        NekoList(v) => {
            if !v.is_empty() {                
                apply(n,env)
            } else {
                n
            }
        },
        _ => eval_ast(n,env)
    }
}

pub fn apply(mut list:NekoType,env:&mut Env) -> NekoType {
    //对列表执行求值与应用操作
    if let NekoList(mut v) = list.get_value() {
        let mut first_arg = v.remove(0);
        let mut args:Vec<NekoType> = Vec::new();
        for cn in v {
            args.push(cn);
        };
        match first_arg.get_value() {
            NekoFn(f) => {
                if f.is_lambda(){
                    return f.call_l(args);
                } else {
                    return NekoType::err("不可调用函数".to_string());
                }
            },
            NekoSymbol(_) => {
                let mut nfn
                    = eval_ast(first_arg.clone(),env);
                if let NekoFn(f) = nfn.get_value() {
                    if f.is_special() {
                        return f.call_s(args,env);
                    }
                }
                return apply(eval_ast(list,env),env);
            },
            _ => {return list},
        };
    };
    return list;
}
