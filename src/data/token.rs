use crate::{data::Ident, Pos};

#[derive(Debug, Eq, PartialEq)]
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

    /// $ Reserved!
    DollarSign,

    /// Starts with [_ UnicodeLetter], then [_ - UnicodeLetter UnicodeDigit]
    Ident(Ident),
    /// .
    IdentSplit,
    /// #IDENT
    Macro(Ident),

    /// 0..=9
    NumberDecimal(String),
    /// 0x 0..=F
    NumberHex(String),
    /// 0o 0..=8
    NumberOctal(String),
    /// 0b 0..=1
    NumberBinary(String),

    Character(char),
    // TODO: String, StringType
    String(),

    /// true
    True,
    ///  false
    False,

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
    /// ;
    SemiColon,

    /// const
    Const,
    /// let
    Let,
    /// var
    Var,

    /// as
    Cast,

    /// if
    If,
    /// guard
    Guard,
    /// else
    Else,

    /// for
    For,
    /// while
    While,

    /// fn
    Function,

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
impl Token {
    #[must_use]
    pub const fn is_number(&self) -> bool {
        matches!(
            self,
            Self::NumberDecimal(..)
                | Self::NumberHex(..)
                | Self::NumberOctal(..)
                | Self::NumberBinary(..)
        )
    }
}
