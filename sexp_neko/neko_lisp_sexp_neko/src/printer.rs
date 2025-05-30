use alloc::{vec::Vec, string::String, boxed::Box};
use core::fmt::Write;
use crate::types::NekoType;
use crate::types::NekoType::*;

pub fn pr_str(neko:NekoType) -> String {
    let mut output = String::new();
    match neko {
        NekoInt(n) => {output = n.to_string();},
        NekoFloat(f) => {output = f.to_string();},
        NekoSymbol(s) => {output = s;},
        NekoString(s) => {output = s;},
        NekoNil => {output = "nil".to_string();},
        NekoErr(e) => {write!(&mut output,"Error: {}", e);},
        NekoList(v) => {
            let mut sv:Vec<String> = Vec::new();
            for n in v {
                sv.push(pr_str(n));
            }
            write!(&mut output,"({})", sv.join(" "));
        },
        _ => {output = "未实现".to_string();}
    }
    return output;
}
