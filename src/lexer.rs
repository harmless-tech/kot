use crate::Pos;

#[derive(Debug)]
pub struct PosToken {
    pub token: Token,
    pub pos: Pos,
}
impl PosToken {
    fn new(token: Token, line: usize, col: usize) -> Self {
        Self { token, pos: Pos { line, col } }
    }
}

#[derive(Debug)]
pub enum Token {
    Ident(String), // Could also be a type.
    PoundIdent(String),
    Number(String),
    String(String),
    BackString(String),
    RawString(String),
    Question,
    Eof,

    LParan,
    RParan,
    LBracket,
    RBracket,
    LCurly,
    RCurly,
    Comma,
    Colon,

    Const,
    Let,
    Var,
    /// =
    Assign,

    True,
    False,

    Enum,
    Struct,
    /// fn
    Function,
    Return,
    Where,

    If,
    Guard,
    Else,
    While,
    For,
    In,

    Module,
    Public,

    Equal,
    Not,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,

    Multiply,
    Divide,
    Add,
    Subtract,
    Modulus,

    BitNot,
    BitAnd,
    BitOr,
    /// <<
    BitShiftLeft,
    /// >>
    BitShiftRight,

    /// ..=
    RangeInclusive,
    /// ..<
    RangeExclusive,
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

fn lex(contents: &str) -> anyhow::Result<()> {
    let mut lexer = Lexer::new(contents);

//    let mut tokens = Vec::new();

    todo!()
}
