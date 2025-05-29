use crate::types::NekoType;
use crate::types::NekoType::*;

pub fn pr_str(neko:NekoType) -> String {
    match neko {
        NekoInt(n) => n.to_string(),
        NekoSymbol(s) => s,
        NekoNil => "nil".to_string(),
        NekoErr(e) => format!("Error: {}", e),
        NekoList(v) => {
            let mut sv:Vec<String> = Vec::new();
            for n in v {
                sv.push(pr_str(n));
            }
            sv.join(" ")
        },
        _ => "未实现".to_string()
    }
}
