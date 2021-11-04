#[derive(Debug)]
pub enum Value {
    Int64(i64),
    UInt64(u64),
    Float64(f64),

    Byte(u8),
    Char(char),
    String(String),
    Boolean(bool),

    Array(), //TODO
    Tuple(),
    Object(),
}

#[derive(Debug)]
pub enum Operations {}

#[derive(Debug)]
pub enum Expressions {}
