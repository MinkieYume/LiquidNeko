#[derive(Debug, Clone, PartialEq)]
pub enum NekoType {
    NekoInt(i64),
    NekoSymbol(String),
    NekoFloat(f64),
    NekoString(String),
    NekoChar(char),
    NekoList(Vec<NekoType>),
    NekoErr(String),
    NekoNil
}
