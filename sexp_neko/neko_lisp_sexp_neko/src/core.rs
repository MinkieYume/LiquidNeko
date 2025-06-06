use alloc::{boxed::Box,string::String, vec::Vec,rc::Rc};
use core::cell::RefCell;
use std::borrow::Borrow;
use hashbrown::HashMap;
use crate::types::Symbol;
use crate::types::NekoType;
use crate::types::Function;
use crate::types::NekoValue;
use crate::types::NekoValue::*;
use crate::env::Env;
use crate::reader::*;
use crate::eval::*;
use crate::printer::*;

pub struct Core {
    pub binddings:HashMap<Symbol, NekoType>
}

impl Core {
    pub fn default() -> Core {
        let mut binds:HashMap<Symbol, NekoType> = HashMap::new();
        let add =
            Function::new_box(Rc::new(Box::new(|v,e| add_v(v))),"ADD",false);
        binds.insert(Symbol("+".to_string()),NekoType::func(add));
        let sub =
            Function::new_box(Rc::new(Box::new(|v,e| sub_v(v))),"SUB",false);
        binds.insert(Symbol("-".to_string()),NekoType::func(sub));
        let mul =
            Function::new_box(Rc::new(Box::new(|v,e| mul_v(v))),"MUL",false);
        binds.insert(Symbol("*".to_string()),NekoType::func(mul));
        let div =
            Function::new_box(Rc::new(Box::new(|v,e| div_v(v))),"DIV",false);
        binds.insert(Symbol("/".to_string()),NekoType::func(div));
        let list =
            Function::new_box(Rc::new(Box::new(|v,e| list(v))),"LIST",false);
        binds.insert(Symbol("list".to_string()),NekoType::func(list));
        let listp =
            Function::new_box(Rc::new(Box::new(|v,e| listp(v))),"LIST?",false);
        binds.insert(Symbol("list?".to_string()),NekoType::func(listp));
        let emptyp =
            Function::new_box(Rc::new(Box::new(|v,e| emptyp(v))),"EMPTY?",false);
        binds.insert(Symbol("empty".to_string()),NekoType::func(emptyp));
        let count =
            Function::new_box(Rc::new(Box::new(|v,e| count(v))),"COUNT",false);
        binds.insert(Symbol("count?".to_string()),NekoType::func(count));
        let read_string =
            Function::new_box(Rc::new(Box::new(|v,e| read_string(v,e))),"READSTRING",false);
        binds.insert(Symbol("read-string".to_string()),NekoType::func(read_string));
        let atom =
            Function::new_box(Rc::new(Box::new(|v,e| atom(v,e))),"ATOM",false);
        binds.insert(Symbol("atom".to_string()),NekoType::func(atom));
        let atomp =
            Function::new_box(Rc::new(Box::new(|v,e| atomp(v,e))),"ATOM?",false);
        binds.insert(Symbol("atom?".to_string()),NekoType::func(atomp));
        let deref =
            Function::new_box(Rc::new(Box::new(|v,e| deref(v,e))),"DEREF",false);
        binds.insert(Symbol("deref".to_string()),NekoType::func(deref));
        let reset =
            Function::new_box(Rc::new(Box::new(|v,e| reset(v,e))),"RESET",false);
        binds.insert(Symbol("reset".to_string()),NekoType::func(reset));
        let swap =
            Function::new_box(Rc::new(Box::new(|v,e| swap(v,e))),"SWAP",false);
        binds.insert(Symbol("swap".to_string()),NekoType::func(swap));
        let def =
            Function::new_box(Rc::new(
                Box::new(|v,e| def(v,e))),"DEF",true);
        binds.insert(Symbol("def!".to_string()),
                     NekoType::func(def));
        let let_ =
            Function::new_box(Rc::new(
                Box::new(|v,e| let_(v,e))),"LET",true);
        binds.insert(Symbol("let".to_string()),
                     NekoType::func(let_));
        let if_ =
            Function::new_box(Rc::new(
                Box::new(|v,e| if_(v,e))),"IF",true);
        binds.insert(Symbol("if".to_string()),
                     NekoType::func(if_));
        let progn =
            Function::new_box(Rc::new(
                Box::new(|v,e| progn(v,e))),"PROGN",true);
        binds.insert(Symbol("progn".to_string()),
                     NekoType::func(progn));
        let lambda =
            Function::new_box(Rc::new(
                Box::new(|v,e| lambda(v,e))),"LAMBDA",true);
        binds.insert(Symbol("lambda".to_string()),
                     NekoType::func(lambda));
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

fn list(mut v:Vec<NekoType>) -> NekoType {
    return NekoType::list(v);
}

fn listp(mut v:Vec<NekoType>) -> NekoType {
    if v.len() == 1 {
        let l = v.remove(0);
        if let NekoList(_) = *l.get_ref(){
            return NekoType::neko_true();
        } else {
            return NekoType::nil();
        }
    } else {
        return NekoType::err("只支持单一参数判定".to_string());
    }
}

fn emptyp(mut v:Vec<NekoType>) -> NekoType {
    let l = v.remove(0);
    if let NekoList(nv) = l.get_ref().borrow() {
        if nv.is_empty(){
            return NekoType::neko_true();
        } else {
            return NekoType::nil();
        }
    } else {
        return NekoType::err("参数不是列表".to_string());
    }
}

fn count(mut v:Vec<NekoType>) -> NekoType {
    let l = v.remove(0);
    if let NekoList(nv) = l.get_ref().borrow() {
        return NekoType::int_64(nv.len() as i64);
    } else {
        return NekoType::err("参数不是列表".to_string());
    }
}

fn read_string(mut v:Vec<NekoType>,env:Env) -> NekoType {
    let mut results:Vec<NekoType> = Vec::new();
    for arg in v {
        if let NekoString(s) = arg.get_ref().borrow() {
            results.push(read_str(s.as_str(),env.clone()));
        }
    }
    if results.len() != 1 {
        return NekoType::list(results);
    } else {
        return results.remove(0);
    }
    
}

fn def(mut args:Vec<NekoType>,env:Env) -> NekoType {
    if args.len()%2 == 0 {
        let mut last_arg = NekoType::nil();
        let mut result_args:Vec<NekoType> = Vec::new();
        for arg in args {
            if let NekoSymbol(a) = last_arg.copy_value() {
                let new_val = eval(arg,env.clone());
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

fn let_(mut args:Vec<NekoType>,env:Env) -> NekoType {
    let mut bindings = args.remove(0);
    let mut n_env = Env::new(Some(env.clone()));
    if let NekoList(bs) = bindings.copy_value() {
        let mut r_env = let_set_bindings(bs,n_env.clone());
        if let Some(e) = r_env {
            let mut n_env = e;
            let mut list = NekoType::list(args);
            env.set_tco(n_env.clone());
            return list;
        }
    };
    return NekoType::err("let的绑定不合规".to_string());
}

fn let_set_bindings(mut args:Vec<NekoType>,env:Env) -> Option<Env> {
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
                let mut n = eval_ast(secend_arg,env.clone());
                env.set(s,n);
                return Some(env)
            }
        },
        _ => {None}
    }
}

fn if_(mut args:Vec<NekoType>,env:Env) -> NekoType {
    if args.len() >= 2 && args.len() <= 3 {
        let mut arg = args.get(0).
            unwrap_or(&NekoType::nil()).clone();
        let condition = eval(arg.clone(),env.clone());
        if let NekoNil = *condition.get_ref() {
            if args.len() == 3 {
                let mut arg3 = args.get(2).
                    unwrap_or(&NekoType::nil()).clone();
                env.set_tco(env.clone());
                return arg3.clone();
            } else {
                NekoType::nil()
            }
        } else {
            let mut arg2 = args.get(1).
                unwrap_or(&NekoType::nil()).clone();
            env.set_tco(env.clone());
            return arg2.clone();
        }
    } else {
        NekoType::err("不合法的if语句".to_string())
    }
}

fn progn(mut args:Vec<NekoType>,env:Env) -> NekoType {
    let mut result = NekoType::nil();
    let mut arg = args.remove(0);
    result = eval(arg,env.clone());
    if args.len() > 0 {
        let mut progn_fn = NekoType::symbol("progn".to_string());
        env.set_tco(env.clone());
        args.insert(0,progn_fn);
        return NekoType::list(args);
    } else {
        return result;
    }
}


fn lambda(mut args:Vec<NekoType>,env:Env) -> NekoType {
    if let Some(s) = args.get(0){
        if let NekoList(_) = *s.get_ref() {
           
        } else {
             return NekoType::err("第一个项必须为列表".to_string())
        }
    } else {
        return NekoType::err("必须项不存在".to_string())
    }
    let mut prlist:Vec<NekoType> = Vec::new();
    prlist.push(NekoType::symbol("FUNCTION".to_string()));
    prlist.append(&mut args.clone());
    let mut pralist = NekoType::list(prlist);
    let mut f = Function::
    new_ast(args,pr_str(pralist).as_str(),false);
    return NekoType::func(f);
}

fn atom(mut args:Vec<NekoType>,env:Env) -> NekoType {
    if args.len() > 0{
        let mut results:Vec<NekoType> = Vec::new();
        for arg in args {
            results.push(NekoType::atom(arg.clone()));
        }
        if results.len() == 1{
            return results.remove(0);
        } else {
            return NekoType::list(results);
        }
    } else {
        return NekoType::err("参数不能为空".to_string())
    }
    
}

fn atomp(mut args:Vec<NekoType>,env:Env) -> NekoType {
    if args.len() == 1 {
        let l = args.remove(0);
        if let NekoAtom(_) = *l.get_ref() {
            return NekoType::neko_true();
        } else {
            return NekoType::nil();
        }
    } else {
        return NekoType::err("只支持单一参数判定".to_string());
    }
}

fn deref(mut args:Vec<NekoType>,env:Env) -> NekoType {
    if args.len() > 0 {
        let mut results:Vec<NekoType> = Vec::new();
        for arg in args {
            //println!("{}",arg.get_type_str());
            if let NekoAtom(a) = arg.get_ref().borrow() {
                let mut atom = a.borrow_mut();
                let n = atom.get_neko();
                results.push(n);
            } else {
                return NekoType::err("参数不为原子".to_string())
            }
        }
        if results.len() == 1{
            return results.remove(0);
        } else {
            return NekoType::list(results);
        }
    } else {
        return NekoType::err("参数不能为空".to_string())
    }
}

fn reset(mut args:Vec<NekoType>,env:Env) -> NekoType {
    if args.len() == 2 {
        let atom_n = args.remove(0);
        let neko = args.remove(0);
        if let NekoAtom(a) = atom_n.get_ref().borrow() {
            let mut atom = a.borrow_mut();
            atom.set_neko(neko.clone());
            return neko;
        } else {
            return NekoType::err("奇参数不是Atom".to_string())
        }
    } else {
        return NekoType::err("参数数量不对".to_string());
    }
}

fn swap(mut args:Vec<NekoType>,env:Env) -> NekoType {
    if args.len() >= 2 {
        let atom_n = args.remove(0);
        let func_n = args.remove(0);
        if let NekoAtom(a) = atom_n.get_ref().borrow() {
            let mut atom = a.borrow_mut();
            if let NekoFn(f) = func_n.get_ref().borrow(){
                args.insert(0,atom.get_neko());
                let r = f.call(args,env);
                atom.set_neko(r.clone());
                return r
            }
        }
        return NekoType::err("输入参数类型不正确".to_string());
    } else {
        return NekoType::err("参数数量不对".to_string());
    }

    
}
