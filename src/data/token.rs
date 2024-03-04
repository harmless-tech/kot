use crate::Pos;

#[derive(Debug)]
pub struct PosToken {
    pub token: Token,
    pub pos: Pos,
}
impl PosToken {
    #[must_use]
    pub const fn new(token: Token, pos: Pos) -> Self {
        Self { token, pos }
    }

    #[must_use]
    pub const fn eof(pos: Pos) -> Self {
        Self {
            token: Token::Eof,
            pos,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Eof,

    /// ; Reserved!
    SemiColon,
    /// $ Reserved!
    DollarSign,

    /// Starts with [_ UnicodeLetter], then [_ - UnicodeLetter UnicodeDigit]
    Ident(String),
    /// #IDENT
    Macro(),

    Number(String),
    NumberHex(String),
    NumberBinary(String),

    Character(char),
    String(),

    /// (
    LParentheses,
    /// )
    RParentheses,
    /// [
    LBracket,
    /// ]
    RBracket,
    /// {
    LCurly,
    /// }
    RCurly,
    /// ,
    Comma,
    /// :
    Colon,

    /// const
    Const,
    /// let
    Let,

    /// ..<
    RangeExclusive,
    /// ..=
    RangeInclusive,

    /// =
    Assign,
    /// *=
    AssignMathMultiply,
    /// /=
    AssignMathDivide,
    /// %=
    AssignMathModulus,
    /// +=
    AssignMathAdd,
    /// -=
    AssignMathSubtract,
    /// <<=
    AssignBitLeft,
    /// >>=
    AssignBitRight,
    /// ~=
    AssignBitNot,
    /// &=
    AssignBitAnd,
    /// ^=
    AssignBitXor,
    /// |=
    AssignBitOr,

    /// *
    MathMultiply,
    /// /
    MathDivide,
    /// %
    MathModulus,
    /// +
    MathAdd,
    /// -
    MathSubtract,

    /// true
    BoolTrue,
    /// false
    BoolFalse,
    /// !
    BoolNot,
    /// &&
    BoolAnd,
    /// ^^
    BoolXor,
    /// ||
    BoolOr,

    /// ==
    CompareEqual,
    /// !=
    CompareNotEqual,
    /// <
    CompareLess,
    /// <=
    CompareLessEqual,
    /// >
    CompareGreater,
    /// >=
    CompareGreaterEqual,

    /// ~
    BitNot,
    /// <<
    BitLeft,
    /// >>
    BitRight,
    /// &
    BitAnd,
    /// ^
    BitXor,
    /// |
    BitOr,
}
