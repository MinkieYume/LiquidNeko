use alloc::{vec::Vec, string::String, boxed::Box};

#[derive(Debug, Clone, PartialEq)]
pub enum NekoType {
    NekoSymbol(String),
    NekoInt(i64),
    NekoFloat(f64),
    NekoString(String),
    NekoChar(char),
    NekoBool(bool),
    NekoTriBool(Option<bool>),
    NekoKeyword(String),
    NekoList(Vec<NekoType>),
    NekoErr(String),
    NekoNil
}
