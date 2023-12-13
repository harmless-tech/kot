use crate::Pos;

pub type Tokens = Vec<PosToken>;

#[derive(Debug)]
pub struct PosToken {
    pub token: Token,
    pub pos: Pos,
}
impl PosToken {
    pub fn new(token: Token, line: usize, col: usize) -> Self {
        Self {
            token,
            pos: Pos { line, col },
        }
    }

    pub fn eof() -> Self {
        Self {
            token: Token::EOF,
            pos: Pos {
                line: usize::MAX,
                col: usize::MAX,
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    /// Starts with [_ A..Z a..z], then [_ - A..Z a..z 0..9]
    Ident(String),
    /// .
    Dot,
    /// #IDENT
    Macro(MacroToken),
    /// 213, -123, 123.0, -123.0, 123_000_000.00_00
    Number(String),
    /// 0x123abc, 0x123ABC
    NumberHex(String),
    /// 0b1101, 0b10101000
    NumberBinary(String),
    /// '\n', 'c'
    Character(char),
    /// "", "string" <br>
    /// \`back string\`, <br>
    /// \`back \ <br>
    /// string\` == "back string" <br>
    /// r"", r#" " "#
    String(String),
    // TODO: Remove old string types
    /// \`back string\`, <br>
    /// \`back \ <br>
    /// string\` == "back string"
    #[deprecated(note = "Use `String` instead")]
    BackString(String),
    /// r"", r#" " "#
    #[deprecated(note = "Use `String` instead")]
    RawString(String),
    EOF,

    /// (
    LParan,
    /// )
    RParan,
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
    /// ; Reserved!
    SemiColon,
    /// $ Reserved!
    DollarSign,

    /// const
    Const,
    /// let
    Let,
    /// var
    Var,

    /// true
    True,
    /// false
    False,

    /// enum Reserved!
    Enum,
    /// struct Reserved!
    Struct,
    /// rec
    Recursive,
    /// fn
    Function,
    /// return
    Return,
    // ret
    ScopeReturn,
    /// trait Reserved!
    Trait,
    /// impl Reserved!
    Implement,
    /// where Reserved!
    Where,

    /// try
    Try,
    /// if
    If,
    /// guard
    Guard,
    /// else
    Else,
    /// switch
    Switch,
    /// while
    While,
    /// for
    For,
    /// in
    In,
    /// break
    Break,

    /// mod
    Module,
    /// pub
    Public,

    /// ? (Optional type)
    QuestionMark,
    /// ?? Reserved!
    Coalesce,

    /// =
    Assign,
    /// =>
    AssignPath,
    /// ==
    Equal,

    /// !
    Not,
    /// !=
    NotEqual,
    /// <
    Less,
    /// <=
    LessEqual,
    /// \>
    Greater,
    /// \>=
    GreaterEqual,
    /// &&
    And,
    /// ||
    Or,

    /// *
    Multiply,
    /// *=
    MultiplyAssign,
    /// /
    Divide,
    /// /=
    DivideAssign,
    /// +
    Add,
    /// +=
    AddAssign,
    /// -
    Subtract,
    /// -=
    SubtractAssign,
    /// %
    Modulus,
    /// %=
    ModulusAssign,

    /// ~
    BitNegate,
    /// ~=
    BitNegateAssign,
    /// &
    BitAnd,
    /// |=
    BitAndAssign,
    /// |
    BitOr,
    /// |=
    BitOrAssign,
    /// ^
    BitXor,
    /// ^=
    BitXorAssign,
    /// <<
    BitShiftLeft,
    /// <<=
    BitShiftLeftAssign,
    /// \>\>
    BitShiftRight,
    /// \>\>=
    BitShiftRightAssign,

    /// ..<
    RangeExclusive,
    /// ..=
    RangeInclusive,
}

#[derive(Debug, Eq, PartialEq)]
pub enum MacroToken {
    /// #config IDENT VALUE
    Config,
    /// #use IDENT|STRING
    Use,

    /// #triplet STRING|STRING,STRING...
    Triplet,
    /// #os STRING|STRING,STRING...
    OS,
    /// #family STRING|STRING,STRING...
    Family,
    /// #arch STRING|STRING,STRING...
    Arch,

    /// #comptime {}
    CompTime,
    /// testtime {}
    TestTime,

    /// #bytes fill BYTE AMOUNT <br>
    /// #bytes from IDENT <br>
    /// #bytes from (VALUE)
    /// #bytes file "PATH"
    Bytes,
    /// #map, #map {}, #map { "STRING": VAL, "": VAL }
    Map,
    /// #set, #set [], #set [VAL, VAL]
    Set,
    /// #regex IDENT/STRING
    Regex,
}
impl TryFrom<&str> for MacroToken {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "config" => Ok(MacroToken::Config),
            "use" => Ok(MacroToken::Use),
            "triplet" => Ok(MacroToken::Triplet),
            "os" => Ok(MacroToken::OS),
            "family" => Ok(MacroToken::Family),
            "arch" => Ok(MacroToken::Arch),
            "comptime" => Ok(MacroToken::CompTime),
            "testtime" => Ok(MacroToken::TestTime),
            "bytes" => Ok(MacroToken::Bytes),
            "map" => Ok(MacroToken::Map),
            "set" => Ok(MacroToken::Set),
            "regex" => Ok(MacroToken::Regex),
            _ => Err(()),
        }
    }
}
