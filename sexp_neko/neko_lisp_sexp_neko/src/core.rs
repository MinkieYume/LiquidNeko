use std::borrow::Borrow;

use alloc::{boxed::Box,string::String, vec::Vec,rc::Rc};
use hashbrown::HashMap;
use crate::types::Symbol;
use crate::types::NekoType;
use crate::types::NekoValue;
use crate::types::NekoValue::*;
use crate::types::Function;
use crate::types::Function::*;
use crate::env::Env;
use crate::eval::*;

pub struct Core {
    pub binddings:HashMap<Symbol, NekoType>
}

impl Core {
    pub fn default() -> Core {
        let mut binds:HashMap<Symbol, NekoType> = HashMap::new();
        let add =
            Lambda(Rc::new(Box::new(|v| add_v(v))));
        binds.insert(Symbol("+".to_string()),NekoType::func(add));
        let sub =
            Lambda(Rc::new(Box::new(|v| sub_v(v))));
        binds.insert(Symbol("-".to_string()),NekoType::func(sub));
        let mul =
            Lambda(Rc::new(Box::new(|v| mul_v(v))));
        binds.insert(Symbol("*".to_string()),NekoType::func(mul));
        let div =
            Lambda(Rc::new(Box::new(|v| div_v(v))));
        binds.insert(Symbol("/".to_string()),NekoType::func(div));
        let def =
            SpecialForms(Rc::new(
                Box::new(|v,e| def(v,e))));
        binds.insert(Symbol("def!".to_string()),
                     NekoType::func(def));
        let let_ =
            SpecialForms(Rc::new(
                Box::new(|v,e| let_(v,e))));
        binds.insert(Symbol("let".to_string()),
                     NekoType::func(let_));
        let if_ =
            SpecialForms(Rc::new(
                Box::new(|v,e| if_(v,e))));
        binds.insert(Symbol("if".to_string()),
                     NekoType::func(if_));
        let progn =
            SpecialForms(Rc::new(
                Box::new(|v,e| progn(v,e))));
        binds.insert(Symbol("progn".to_string()),
                     NekoType::func(progn));
        Core{
            binddings:binds,
        }
    }
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

fn sub_v(mut v:Vec<NekoType>) -> NekoType {
    if v.len() < 1 {
        NekoType::err("参数不足".to_string())
    } else {
        let mut n1 = v.remove(0);
        for n in v {
            n1=n1-n;
        }
        n1                
    }
}

fn mul_v(mut v:Vec<NekoType>) -> NekoType {
    if v.len() < 1 {
        NekoType::err("参数不足".to_string())
    } else {
        let mut n1 = v.remove(0);
        for n in v {
            n1=n1*n;
        }
        n1
    }
}

fn div_v(mut v:Vec<NekoType>) -> NekoType {
    if v.len() < 1 {
        NekoType::err("参数不足".to_string())
    } else {
        let mut n1 = v.remove(0);
        for n in v {
            n1=n1/n;
        }
        n1
    }
}

fn def(mut args:Vec<NekoType>,env:&mut Env) -> NekoType {
    if args.len()%2 == 0 {
        let mut last_arg = NekoType::nil();
        let mut result_args:Vec<NekoType> = Vec::new();
        for arg in args {
            if let NekoSymbol(a) = last_arg.copy_value() {
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
    if let NekoList(bs) = bindings.copy_value() {
        let mut r_env = let_set_bindings(bs,&mut n_env);
        if let Some(e) = r_env {
            let mut n_env = e;
            let mut list = NekoType::list(args);
            let result = eval(list,&mut n_env);
            match result.copy_value() {
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
    match first_arg.copy_value() {
        NekoList(mut b) => {
            let mut n_env = env;
            let mut oenv = let_set_bindings(b,n_env);
            if let Some(e) = oenv{
                n_env = e;
                for arg in args {
                    if let NekoList(ab) = arg.copy_value() {
                        let mut n_oenv =
                            let_set_bindings(ab,n_env);
                        if let Some(ne) = n_oenv {
                            n_env = ne
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                return Some(n_env);
            }
            return None;
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

fn if_(mut args:Vec<NekoType>,env:&mut Env) -> NekoType {
    if args.len() >= 2 && args.len() <= 3 {
        let mut arg = args.get(0).
            unwrap_or(&NekoType::nil()).clone();
        let condition = eval(arg.clone(),env);
        if let NekoNil = *condition.get_ref() {
            if args.len() == 3 {
                let mut arg3 = args.get(2).
                    unwrap_or(&NekoType::nil()).clone();
                eval(arg3.clone(),env)
            } else {
                NekoType::nil()
            }
        } else {
            let mut arg2 = args.get(1).
                unwrap_or(&NekoType::nil()).clone();
            eval(arg2.clone(),env)
        }
    } else {
        NekoType::err("不合法的if语句".to_string())
    }
}

fn progn(mut args:Vec<NekoType>,env:&mut Env) -> NekoType {
    let mut result = NekoType::nil();
    for arg in args {
        result = eval(arg,env);
    }
    return result;
}
