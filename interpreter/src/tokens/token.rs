#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Keyword(String),
    Integer(String),
    Float(String),
    String(String),
    Id(String),
    OpenParenthesis,
    CloseParenthesis,
    Reserved,
}
