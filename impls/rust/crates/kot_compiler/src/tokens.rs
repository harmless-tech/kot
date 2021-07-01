#[derive(Debug)]
pub enum Token {
    Val,
    ID(String),
    Colon,
    Equal,
    Comma,

    LeftParentheses,
    RightParentheses,
    LeftBracket,
    RightBracket,
    LeftCurlyBrace,
    RightCurlyBrace,

    TypeChar,
    TypeInt64,   // Converted from ValueNumber.
    TypeUInt64,  // Converted from ValueNumber.
    TypeFloat64, // Converted from ValueNumber.
    TypeByte,    // Converted from ValueNumber.
    TypeString,
    TypeBoolean,
    TypeObject,
    // An array will have brackets after the type.
    Function,
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
    BitwiseShiftRight,

    ValueChar(String), // ValueChar and ValueNumber have extra requirements to be checked by the parser later on.
    ValueNumber(String),
    ValueString(String),
    ValueBoolean(bool),
}
