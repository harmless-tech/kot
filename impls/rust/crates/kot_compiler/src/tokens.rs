#[derive(Debug)]
pub enum Token {
    Data, // AKA let
    Interface,
    ComplyWith,
    Colon,
    Assign,
    Comma,
    QuestionMark,

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
    TypeObject, // An array will have brackets after the type and therefore does not need its own token.

    Function,
    Cast,
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

    ValueChar(char),
    ValueNumber(String), // ValueNumber have extra requirements to be checked by the parser later on.
    ValueString(String),
    ValueBoolean(bool),

    ID(String),
    LineNum(usize), // For compiler use only.
}
