pub enum Token {
    Var,
    Val,
    ID(String),
    Colon,
    Equal,

    TypeChar,
    TypeInt64,   // Converted from ValueNumber.
    TypeUInt64,  // Converted from ValueNumber.
    TypeFloat64, // Converted from ValueNumber.
    TypeByte,    // Converted from ValueNumber.
    TypeString,
    TypeBoolean,
    ValueChar(String), // ValueChar and ValueNumber have extra requirements to be checked by the parser.
    ValueNumber(String),
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
    BitwiseShiftRight,
}
