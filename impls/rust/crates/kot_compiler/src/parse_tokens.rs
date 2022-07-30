use std::collections::HashMap; // TODO: Better alternative?

pub enum Operation {
    Concat,
    Negate,
    And,
    Or,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Equals,
    NotEquals,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNegate,
    BitwiseShiftLeft,
    BitwiseShiftRight,
}

pub enum Type {
    Int64,
    UInt64,
    Float64,
    Byte,
    Char,
    String,
    Boolean,
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Object,
}

pub enum Value {
    Int64(i64), //TODO Should these types be wrapped in a box?
    UInt64(u64),
    Float64(f64),

    Byte(u8),
    Char(char),
    String(String),
    Boolean(bool),

    Array(Vec<Value>),
    Tuple(Vec<Value>),
    Object(HashMap<String, Value>),
}

pub enum Expression {
    Value(Value),
    ID(String),
    Function(String, Box<Expression>), //TODO: Maybe not...
    Cast(Box<Expression>, Type),
    Not(Box<Expression>),
    Binop(Operation, Box<Expression>, Box<Expression>), //TODO: These should eval to enum Value in the end.
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    FunctionCall(Box<Expression>, Box<Expression>),
    Data(String, Type, Box<Expression>),
    Interface(String, HashMap<String, Value>),
}
