pub enum Token {
    Var,
    Val,
    ID(String),
    Colon,
    Equal,

    TypeChar,
    TypeInt64, // Converted from ValueInt or ValueNegativeInt.
    TypeUInt64, // Converted from ValueInt.
    TypeFloat64, // Converted from ValueInt, ValueNegativeInt, or ValueFloat.
    TypeByte, // Converted from ValueInt.
    TypeString,
    TypeBoolean,
    ValueChar(char),
    ValueInt(u64),
    ValueNegativeInt(i64),
    ValueFloat(f64),
    ValueString(String),
    ValueBoolean(bool),

    TypeExtArray,
    LeftBracket,
    RightBracket,
    Comma,

    TypeObject,
    LeftCurlyBrace,
    RightCurlyBrace,

    Function,
    LeftParentheses,
    RightParentheses,

    Concat,
    Negate,
    And,
    Or,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    DoubleEquals,
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
    BitwiseShiftRight
}

pub enum TokenValue {

}
