use crate::Ident;

#[derive(Debug, Eq, PartialEq)]
pub struct PosToken {
    pub token: Token,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Eof, // End of file

    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Colon,        // :
    Semicolon,    // ;
    Extend,       // ..

    Assign, // =

    Split,       // .
    StaticSplit, // ::
    Arrow,       // ->

    AllocatorMark,    // '
    Ref,              // .&
    Deref,            // .*
    PointerOrMathMul, // *
    Optional,         // ?
    Unwrap,           // .?
    ForceUnwrap,      // .!

    MathAdd,       // +
    MathAddAssign, // +=
    MathSub,       // -
    MathSubAssign, // -=
    // TODO: MathMul,       // *
    MathMulAssign, // *=
    MathDiv,       // /
    MathDivAssign, // /=
    MathMod,       // %
    MathModAssign, // %=

    BitNegate,           // ~
    BitNegateAssign,     // ~=
    BitAnd,              // &
    BitAndAssign,        // &=
    BitOr,               // |
    BitOrAssign,         // |=
    BitXor,              // ^
    BitXorAssign,        // ^=
    BitShiftLeft,        // <<
    BitShiftLeftAssign,  // <<=
    BitShiftRight,       // >>
    BitShiftRightAssign, // >>=

    BoolNot, // !
    BoolAnd, // &&
    BoolOr,  // ||
    BoolXor, // ^^

    CompareEqual,          // ==
    CompareNotEqual,       // !=
    CompareLess,           // <
    CompareLessOrEqual,    // <=
    CompareGreater,        // >
    CompareGreaterOrEqual, // >=

    RangeExclusive, // ..<
    RangeInclusive, // ..=

    // TODO: Do we capture?
    DocModuleComment(String), // //!
    LineComment(String),      // //
    DocBlockComment(String),  // /** */
    BlockComment(String),     // /* */
    // TODO: Only allow on first line? (index 0?)
    HashBang(String), // #!

    CompilerDirective(Ident), // @

    NumHex(String),    // 0xHex
    NumOctal(String),  // 0oOctal
    NumBinary(String), // 0bBinary
    Number(String),    // [-]0-9
    Char(char),        // 'Char'
    String(String),    // "String"
    RawString(String), // """RawString"""

    KNone, // none

    Id(Ident), // [_a-zA-Z][_a-zA-Z0-9]*

    KLet,    // let
    KMut,    // mut
    KConst,  // const
    KStatic, // static
    KAs,     // as
    KType,   // type

    KTrue,  // true
    KFalse, // false
    KIf,    // if
    KElse,  // else
    KFor,   // for
    KIn,    // in
    KWhile, // while

    KMatch, // match

    // TODO: self is first element with type Self?
    KPub,    // pub
    KInline, // inline TODO: Compiler Directive?
    KFn,     // fn
    KReturn, // return
    KRet,    // ret

    KStruct, // struct
    KEnum,   // enum
    KUnion,  // union
    KTrait,  // trait
    KImpl,   // impl

    KExtern, // extern

    KTest, // test
}
