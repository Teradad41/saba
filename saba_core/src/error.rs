use alloc::string::String;

// エラー理由の列挙型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Network(String),
    UnexpectedInput(String),
    InvalidUI(String),
    Other(String),
}
