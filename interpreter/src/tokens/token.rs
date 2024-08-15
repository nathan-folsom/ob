pub enum Token {
    Keyword(String),
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    Id(String),
    OpenParenthesis,
    CloseParenthesis,
    Reserved,
}
