use std::{ops::Range, usize};

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
    /// .
    Dot,
    /// #IDENT
    PoundIdent(MacroToken),
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
    /// \`back string\`, <br>
    /// \`back \ <br>
    /// string\` == "back string"
    BackString(String),
    /// r"", r#" " "#
    RawString(String),
    Eof,

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

#[derive(Debug)]
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
    Bytes,
    /// #map, #map {}, #map { "STRING": VAL, "": VAL }
    Map,
    /// #set, #set [], #set [VAL, VAL]
    Set,
    /// #regex IDENT/STRING
    Regex,
}

struct Lexer {
    chars: Vec<char>,
    index: usize,
    line: usize,
    col: usize,
}
impl Lexer {
    fn new(item: &str) -> Self {
        Self {
            chars: item.chars().collect(),
            index: 0,
            line: 1,
            col: 1,
        }
    }

    fn within(&self) -> bool {
        self.index < self.chars.len()
    }

//    fn get(&mut self) -> char {
//        let c = self.peek_i( 0);
//        self.index += 1;
//        c
//    }

    fn range(&self, range: Range<usize>) -> String {
        self.chars[range].iter().collect()
    }

    fn peek(&self) -> char {
        self.peek_i( 0)
    }

    fn peek_i(&self, offset: usize) -> char {
        match self.chars.get(self.index + offset) {
            Some(c) => *c,
            None => '\0',
        }
    }

    fn i(&mut self, i: usize) {
        self.index += i;
        self.col += i;
    }

    fn newline(&mut self) {
        self.index += 1;
        self.line += 1;
        self.col = 1;
    }
}

pub fn lex(contents: &str) -> anyhow::Result<Vec<PosToken>> {
    let lexer = &mut Lexer::new(contents);
    let mut tokens = Vec::new();

    macro_rules! token {
        ($x:ident) => {{
            tokens.push(PosToken::new(Token::$x, lexer.line, lexer.col));
        }};
        ($x:ident,$i:expr) => {{
            tokens.push(PosToken::new(Token::$x, lexer.line, lexer.col));
            lexer.i($i);
        }};
    }
    macro_rules! token1 {
        ($x:ident) => {{
            token!($x, 1);
        }};
    }
    macro_rules! token2 {
        ($x:ident) => {{
            token!($x, 2);
        }};
    }
    macro_rules! token3 {
        ($x:ident) => {{
            token!($x, 3);
        }};
    }

    while lexer.within() {
        match (lexer.peek(), lexer.peek_i(1)) {
            // Comments
            ('/', '/') => remove_single_line_comment(lexer), // Single line
            ('/', '*') => remove_multi_line_comment(lexer), // Multi line
            // Singles
            ('(', _) => token1!(LParan),
            (')', _) => token1!(RParan),
            ('[', _) => token1!(LBracket),
            (']', _) => token1!(RBracket),
            ('{', _) => token1!(LCurly),
            ('}', _) => token1!(RCurly),
            (',', _) => token1!(Comma),
            (':', _) => token1!(Colon),
            (';', _) => token1!(SemiColon),
            ('$', _) => token1!(DollarSign),
            // ? ??
            ('?', '?') => token2!(Coalesce),
            ('?', _) => token1!(QuestionMark),
            // = => ==
            ('=', '=') => token2!(Equal),
            ('=', '>') => token2!(AssignPath),
            ('=', _) => token1!(Assign),
            // ! !=
            ('!', '=') => token2!(NotEqual),
            ('!', _) => token1!(Not),
            // < <= << <<=
            ('<', '<') => match lexer.peek_i(2) {
                '=' => token3!(BitShiftLeftAssign),
                _ => token2!(BitShiftLeft),
            },
            ('<', '=') => token2!(LessEqual),
            ('<', _) => token1!(Less),
            // > >= >> >>=
            ('>', '>') => match lexer.peek_i(2) {
                '=' => token3!(BitShiftRightAssign),
                _ => token2!(BitShiftRight),
            },
            ('>', '=') => token2!(GreaterEqual),
            ('>', _) => token1!(Greater),
            // && & &=
            ('&', '&') => token2!(And),
            ('&', '=') => token2!(BitAndAssign),
            ('&', _) => token1!(BitAnd),
            // || | |=
            ('|', '|') => token2!(Or),
            ('|', '=') => token2!(BitOrAssign),
            ('|', _) => token1!(BitOr),
            // * *=
            ('*', '=') => token2!(MultiplyAssign),
            ('*', _) => token1!(Multiply),
            // / /=
            ('/', '=') => token2!(DivideAssign),
            ('/', _) => token1!(Divide),
            // + +=
            ('+', '=') => token2!(AddAssign),
            ('+', _) => token1!(Add),
            // - -=
            ('-', '=') => token2!(SubtractAssign),
            ('-', next) if !('0'..='9').contains(&next) => token1!(Subtract),
            // % %=
            ('%', '=') => token2!(ModulusAssign),
            ('%', _) => token1!(Modulus),
            // ~ ~=
            ('~', '=') => token2!(BitNegateAssign),
            ('~', _) => token1!(BitNegate),
            // ^ ^=
            ('^', '=') => token2!(BitXorAssign),
            ('^', _) => token1!(BitXor),
            // ..< ..=
            ('.', '.') => match lexer.peek_i(2) {
                '<' => token3!(RangeExclusive),
                '=' => token3!(RangeInclusive),
                _ => panic!("Lexer: Token '<' or '=' required after \"..\" at {}:{}.", lexer.line, lexer.col), // TODO: Return error instead of panic
            },
            // Numbers
            ('0', 'x') => todo!(),
            ('0', 'b') => todo!(),
            ('-' | '0'..='9', _) => todo!(),
            // Char and Strings
            ('\'', _) => todo!(),
            ('"', _) => todo!(),
            ('`', _) => todo!(),
            ('r', '#' | '"') => todo!(),
            // Macros
            ('#', 'a'..='z') => todo!(),
            ('#', next) => panic!("Lexer: Unexpected token '{next}' after macro tag at {}:{}.", lexer.line, lexer.col), // TODO: Return error instead of panic
            // Ident
            ('.', next)
                if ('a'..='z').contains(&next) || ('A'..='Z').contains(&next) || next == '_' =>
            {
                token1!(Dot)
            }
            ('_' | 'a'..='z' | 'A'..='Z', _) => todo!(),
            // Whitespace, No Newline
            (' ' | '\t' | '\r', _) => lexer.i(1),
            // Whitespace, Newline
            ('\n', _) => lexer.newline(),
            _ => panic!("Lexer: Unexpected token at {}:{}.", lexer.line, lexer.col), // TODO: Return error instead of panic
        }
    }

    token!(Eof);
    Ok(tokens)
}

fn remove_single_line_comment(lexer: &mut Lexer) {
    lexer.i(2);
    while lexer.within() {
        match lexer.peek() {
            '\n' => {
                lexer.newline();
                return;
            }
            _ => lexer.i(1),
        }
    }
}

// TODO: No panic!
fn remove_multi_line_comment(lexer: &mut Lexer) {
    let (line, col) = (lexer.line, lexer.col);

    lexer.i(2);
    while lexer.within() {
        match (lexer.peek(), lexer.peek_i(1)) {
            ('*', '/') => {
                lexer.i(2);
                return;
            }
            ('\n', _) => lexer.newline(),
            _ => lexer.i(1),
        }
    }

    panic!("Lexer: Multi-line comment, starting at {line}:{col}, does not have an ending \"*/\".");
}

// TODO: More Testing
#[cfg(test)]
mod test {
    use super::lex;

    #[test]
    #[should_panic]
    fn panic_multi_line() {
        let _ = lex(r##"/* This comment does not have an ending!!!"##);
    }
}
