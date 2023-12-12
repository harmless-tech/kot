use crate::Pos;

#[derive(Debug)]
pub struct PosToken {
    pub token: Token,
    pub pos: Pos,
}
impl PosToken {
    fn new(token: Token, line: usize, col: usize) -> Self {
        Self {
            token,
            pos: Pos { line, col },
        }
    }
}

#[derive(Debug)]
pub enum Token {
    /// Starts with [_ A..Z a..z], then [_ - A..Z a..z 0..9]
    Ident(String),
    /// #IDENT
    PoundIdent(PoundMacros),
    /// 213, -123, 123.0, -123.0, 123_000_000.00_00
    Number(String),
    /// 0x123abc, 0x123ABC
    NumberHex(String),
    /// 0b1101, 0b10101000
    NumberBinary(String),
    /// '\n', 'c'
    Charater(char),
    /// "", "string"
    String(String),
    /**
     * \`back string\`, <br>
     * \`back \ <br>
     *  string\` == "back string"
     */
    BackString(String),
    /// r"", r#" " "#
    RawString(String),
    /// ? (Optional type)
    QuestionMark,
    Eof,

    LParan,
    RParan,
    LBracket,
    RBracket,
    LCurly,
    RCurly,
    Comma,
    Colon,

    /// const
    Const,
    /// let
    Let,
    /// var
    Var,
    /// =
    Assign,

    /// true
    True,
    /// false
    False,

    /// Reserved!
    Enum,
    /// Reserved!
    Struct,
    /// fn
    Function,
    /// return
    Return,
    // ret
    ScopeReturn,
    /// Reserved!
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
    /// =>
    AssignPath,
    /// while
    While,
    /// for
    For,
    /// in
    In,

    /// mod
    Module,
    /// pub
    Public,

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
    /// >
    Greater,
    /// >=
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
    /// >>
    BitShiftRight,
    /// >>=
    BitShiftRightAssign,

    /// ..=
    RangeInclusive,
    /// ..<
    RangeExclusive,
}

#[derive(Debug)]
pub enum PoundMacros {
    Require,
    Config,
    Use,

    Triplet,
    OS,
    Family,
    Arch,

    CompTime,
    TestTime,

    Bytes,
    Map,
    Set,
    Regex,
}

struct Lexer {
    chars: Vec<char>,
    index: usize,
}
impl Lexer {
    fn new(item: &str) -> Self {
        Self {
            chars: item.chars().collect(),
            index: 0,
        }
    }

    fn get(&mut self) -> char {
        let c = self.peek_i(0);
        self.index += 1;
        c
    }

    fn peek(&self) -> char {
        self.peek_i(1)
    }

    fn peek_i(&self, offset: usize) -> char {
        match self.chars.get(self.index + offset) {
            Some(c) => *c,
            None => '\0',
        }
    }
}

fn lex(contents: &str) -> anyhow::Result<Vec<PosToken>> {
    let mut lexer = Lexer::new(contents);

    let mut tokens = Vec::new();
    let mut index = 0_usize;
    let mut line = 1_usize;
    let mut col = 1_usize;

    todo!()
}
