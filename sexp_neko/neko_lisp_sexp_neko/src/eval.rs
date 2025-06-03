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
                return f.call(args);
            },
            NekoSymbol(s) => {
                if let Symbol(var) = s {
                    if var == "def!".to_string() {
                        return def(args,env);
                    } else if var == "let".to_string() {
                        return let_(args,env);
                    } else {
                        return apply(eval_ast(list,env),env);
                    }
                }
            },
            _ => {return list},
        };
    };
    return list;
}

fn def(mut args:Vec<NekoType>,env:&mut Env) -> NekoType {
    if args.len()%2 == 0 {
        let mut last_arg = NekoType::nil();
        let mut result_args:Vec<NekoType> = Vec::new();
        for arg in args {
            if let NekoSymbol(a) = last_arg.get_value() {
                let new_val = eval(arg,env);
                env.set(a,new_val.clone());
                result_args.push(new_val.clone());
                last_arg = NekoType::nil();
            } else {
                last_arg = arg.clone();
            }
        }
        if result_args.len() == 1 {
            return result_args.remove(0);
        } else {
            return NekoType::list(result_args)
        }
    } else {
        return NekoType::err("参数不匹配".to_string());
    }
}

fn let_(mut args:Vec<NekoType>,env:&mut Env) -> NekoType {
    let mut bindings = args.remove(0);
    let mut n_env = Env::new(Some(&env));
    if let NekoList(bs) = bindings.get_value() {
        let mut r_env = let_set_bindings(bs,&mut n_env);
        if let Some(e) = r_env {
            let mut n_env = e;
            let mut list = NekoType::list(args);
            let result = eval(list,&mut n_env);
            match result.get_value() {
                NekoList(mut l) => {
                    if l.len() == 1 {
                        return l.remove(0);
                    } else {
                        return result;
                    }
                }
                _ => {return result}
            }
        }
    };
    return NekoType::err("let的绑定不合规".to_string());
}

fn let_set_bindings(mut args:Vec<NekoType>,env:&mut Env) -> Option<&mut Env> {
    let mut first_arg = args.remove(0);
    match first_arg.get_value() {
        NekoList(mut b) => {
            return let_set_bindings(b,env);
        },
        NekoSymbol(s) => {
            if args.len() != 1 {
                return None;
            } else {
                let mut secend_arg = args.remove(0);
                let mut n = eval(secend_arg,env);
                env.set(s,n);
                return Some(env)
            }
        },
        _ => {None}
    }
}
