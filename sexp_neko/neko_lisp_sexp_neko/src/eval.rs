use alloc::{vec::Vec, string::String, boxed::Box,collections::BTreeMap};
use core::fmt::Write;
use std::borrow::Borrow;
use crate::types::NekoType;
use crate::types::NekoValue;
use crate::types::NekoValue::*;
use crate::types::Symbol;
use crate::symbols::Symbols;
use crate::env::Env;

pub fn eval_ast(mut n:NekoType,env:Env) -> NekoType {
    //对单个参数进行求值
    match n.get_ref().borrow() {
        NekoSymbol(s) => {env.get(&s)},
        NekoList(v) => {
            let mut nv:Vec<NekoType> = Vec::new();
            for e in v {
                let mut ne = eval(e.clone(),env.clone());
                nv.push(ne);
            }
            NekoType::list(nv)
        },
        _ => n,
    }
}

pub fn eval(mut n:NekoType,env:Env) -> NekoType {
    //对参数进行判定并决定是否执行或应用
    let mut result = NekoType::nil();
    let mut v_n = n.clone();
    let mut v_env = env.clone();
    loop {
        match v_n.get_ref().borrow() {
            NekoList(v) => {
                if !v.is_empty() {
                    result = apply(v_n,v_env.clone());
                } else {
                    result = v_n;
                }
            },
            _ => {result = eval_ast(v_n,v_env.clone());}
        }
        if let Some(tco) = v_env.get_tco() {
            v_env = tco.clone();
            v_n = result.clone();
            v_env.clear_tco();
        } else {
            return result;
        }
    }
}

pub fn apply(mut list:NekoType,env:Env) -> NekoType {
    //对列表执行求值与应用操作
    if let NekoList(mut v) = list.copy_value() {
        let mut first_arg = v.remove(0);
        let mut args:Vec<NekoType> = Vec::new();
        for cn in v {
            args.push(cn);
        };
        match first_arg.copy_value() {
            NekoFn(f) => {
                if !f.is_special(){
                    return f.call(args,env.clone());
                } else {
                    return NekoType::err("不可在此调用特殊形式".to_string());
                }
            },
            NekoList(_)|NekoSymbol(_) => {
                let mut nfn
                    = eval_ast(first_arg.clone(),env.clone());
                if let NekoFn(f) = nfn.copy_value() {
                    if f.is_special() {
                        return f.call(args,env.clone());
                    }
                }
                return apply(eval_ast(list,env.clone()),env.clone());
            },
            _ => {return list},
        };
    };
    return list;
}
