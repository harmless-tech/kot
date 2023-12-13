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

#[derive(Debug, Eq, PartialEq)]
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
    Bytes,
    /// #map, #map {}, #map { "STRING": VAL, "": VAL }
    Map,
    /// #set, #set [], #set [VAL, VAL]
    Set,
    /// #regex IDENT/STRING
    Regex,
}

type Tokens = Vec<PosToken>;

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

    fn range(&self, range: Range<usize>) -> String {
        self.chars[range].iter().collect()
    }

    fn peek(&self) -> char {
        self.peek_i(0)
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
        let (c1, c2) = (lexer.peek(), lexer.peek_i(1));
        match (c1, c2) {
            // Comments
            ('/', '/') => skip_single_line_comment(lexer), // Single line
            ('/', '*') => skip_multi_line_comment(lexer),  // Multi line
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
            ('-', next) if !next.is_ascii_digit() => token1!(Subtract),
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
                _ => panic!(
                    "Lexer: Token '<' or '=' required after \"..\" at {}:{}.",
                    lexer.line, lexer.col
                ), // TODO: Return error instead of panic
            },
            // Numbers
            ('0', 'x') => get_hex(lexer, &mut tokens),
            ('0', 'b') => get_binary(lexer, &mut tokens),
            ('-' | '0'..='9', _) => get_number(lexer, &mut tokens),
            // Char and Strings
            ('\'', _) => get_char(lexer, &mut tokens),
            ('"', _) => get_string(lexer, &mut tokens),
            ('`', _) => get_back_string(lexer, &mut tokens),
            ('r', '#' | '"') => get_raw_string(lexer, &mut tokens),
            // Macros
            ('#', 'a'..='z') => todo!(),
            ('#', next) => panic!(
                "Lexer: Unexpected token '{next}' after macro tag at {}:{}.",
                lexer.line, lexer.col
            ), // TODO: Return error instead of panic
            // Ident
            ('.', next) if next.is_ascii_alphabetic() || next == '_' => {
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

    token!(EOF);
    Ok(tokens)
}

fn skip_single_line_comment(lexer: &mut Lexer) {
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
fn skip_multi_line_comment(lexer: &mut Lexer) {
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

// TODO: No panic
fn get_hex(lexer: &mut Lexer, tokens: &mut Tokens) {
    let (line, col) = (lexer.line, lexer.col);

    let mut builder = String::new();

    lexer.i(2);
    while lexer.within() {
        let c = lexer.peek();
        match c {
            '0'..='9' | 'a'..='f' => {
                builder.push(c);
                lexer.i(1);
            }
            'A'..='F' => {
                builder.push(
                    c.to_lowercase()
                        .next()
                        .expect("Lexer: Impossible char conversion!"),
                );
                lexer.i(1);
            }
            '_' => lexer.i(1), // Skip
            _ => break,
        }
    }

    if builder.is_empty() {
        panic!("Lexer: Hex number starting at {line}:{col} has no token at {}:{}. Format like 0x['0'..='9'|'a'..='f'|'A'..='F'|'_']", lexer.line, lexer.col);
    }

    tokens.push(PosToken::new(Token::NumberHex(builder), line, col));
}

// TODO: No panic
fn get_binary(lexer: &mut Lexer, tokens: &mut Tokens) {
    let (line, col) = (lexer.line, lexer.col);

    let mut builder = String::new();
    lexer.i(2);
    while lexer.within() {
        let c = lexer.peek();
        match c {
            '0' | '1' => {
                builder.push(c);
                lexer.i(1);
            }
            '_' => lexer.i(1), // Skip
            _ => break,
        }
    }

    if builder.is_empty() {
        panic!("Lexer: Binary number starting at {line}:{col} has no token at {}:{}. Format like 0b['0'|'1'|'_']", lexer.line, lexer.col);
    }

    tokens.push(PosToken::new(Token::NumberBinary(builder), line, col));
}

// TODO: No panic
fn get_number(lexer: &mut Lexer, tokens: &mut Tokens) {
    let (line, col) = (lexer.line, lexer.col);
    let mut builder = String::new();

    let c = lexer.peek();
    if c == '-' {
        builder.push(c);
        lexer.i(1);
    }

    while lexer.within() {
        let c = lexer.peek();
        match c {
            '0'..='9' | '.' => {
                builder.push(c);
                lexer.i(1);
            }
            '_' => lexer.i(1), // Skip
            _ => break,
        }
    }

    if builder.is_empty() {
        // TODO: I do not think this can be triggered. Remove.
        panic!("Lexer: Number starting at {line}:{col} has no token at {}:{}. Format like ['-'|'0'..='9']['0'..='9'|'_']", lexer.line, lexer.col);
    }

    tokens.push(PosToken::new(Token::Number(builder), line, col));
}

// TODO: No panic
fn get_char(lexer: &mut Lexer, tokens: &mut Tokens) {
    let (line, col) = (lexer.line, lexer.col);
    let mut builder = String::new();

    lexer.i(1);
    while lexer.within() {
        let c = lexer.peek();
        match c {
            '\'' => {
                lexer.i(1);
                break;
            }
            '\0' => {
                panic!("Lexer: Hit EOF when getting char at {line}:{col}. Missing closing \"'\".")
            }
            _ => {
                builder.push(c);
                lexer.i(1);
            }
        }
    }

    let mut c = '\0';
    if builder.is_empty() {
        panic!("Lexer: Char at {line}:{col} is empty.");
    }
    else if builder.len() > 1 {
        if builder.eq("\n") {
            c = '\n';
        }
        else if builder.eq("\t") {
            c = '\t';
        }
    }
    else {
        c = builder
            .chars()
            .next()
            .expect("Lexer: Missing char is impossible.");
    }

    tokens.push(PosToken::new(Token::Charater(c), line, col))
}

// TODO: No panic
fn get_string(lexer: &mut Lexer, tokens: &mut Tokens) {
    let (line, col) = (lexer.line, lexer.col);

    let mut builder = String::new();
    let mut backslash = false;

    lexer.i(1);
    while lexer.within() {
        let c = lexer.peek();
        match (c, backslash) {
            ('"', false) => { lexer.i(1); break; },
            ('"', true) => { backslash = false; builder.push(c); lexer.i(1); },
            ('n', true) => { backslash = false; builder.push('\n'); lexer.i(1); },
            ('t', true) => { backslash = false; builder.push('\t'); lexer.i(1); },
            ('\\', false) => { backslash = true; lexer.i(1); },
            ('\\', true) => { backslash = false; builder.push(c); lexer.i(1); },
            (_, true) => panic!("Lexer: Hit a random '\\' at {}:{} when getting string at {line}:{col}.", lexer.line, lexer.col),
            ('\n', _) => panic!("Lexer: Hit newline at {}:{} when getting string at {line}:{col}. Missing closing '\"'? Or use a raw string to et multiline support. (r\"\")", lexer.line, lexer.col),
            ('\0', _) => panic!("Lexer: Hit EOF when getting string at {line}:{col}. Missing closing '\"'."),
            _ => { builder.push(c); lexer.i(1); },
        }
    }

    tokens.push(PosToken::new(Token::String(builder), line, col));
}

// TODO: No panic
fn get_back_string(lexer: &mut Lexer, tokens: &mut Tokens) {
    let (line, col) = (lexer.line, lexer.col);

    let mut builder = String::new();
    let mut backslash = false;

    lexer.i(1);
    while lexer.within() {
        let c = lexer.peek();
        match (c, backslash) {
            (t, true) if !t.is_whitespace() => backslash = false,
            ('`', _) => {
                lexer.i(1);
                break;
            }
            ('\n', _) => {
                builder.push(c);
                lexer.newline();
            }
            ('\0', _) => panic!(
                "Lexer: Hit EOF when getting back string at {line}:{col}. Missing closing '`'."
            ),
            _ => {
                builder.push(c);
                lexer.i(1);
            }
        }
    }

    tokens.push(PosToken::new(Token::String(builder), line, col));
}

// TODO: No panic
fn get_raw_string(lexer: &mut Lexer, tokens: &mut Tokens) {
    let (line, col) = (lexer.line, lexer.col);
    lexer.i(1);

    let mut hash_amount = 0;
    while lexer.within() && lexer.peek() == '#' {
        hash_amount += 1;
        lexer.i(1);
    }
    if !lexer.within() {
        panic!(
            "Lexer: Raw string hashes go until EOF at {line}:{col}. Try closing the raw string."
        );
    }

    let mut builder = String::new();
    while lexer.within() {
        let c = lexer.peek();
        match c {
            '"' => {
                let mut backup = String::from(c);
                lexer.i(1);

                let mut matches = true;
                for _ in 0..hash_amount {
                    let c = lexer.peek();
                    if c == '#' {
                        backup.push(c);
                        lexer.i(1);
                    } else {
                        matches = false;
                        break;
                    }
                }

                if matches {
                    if lexer.peek() == '#' {
                        panic!("Lexer: Raw string at {line}:{col} contains a '\"'['#'] that is wider then its own at {}:{}.", lexer.line, lexer.col);
                    }
                    break;
                }
                else {
                    builder.push_str(backup.as_str());
                }
            }
            '\n' => { builder.push(c); lexer.newline(); },
            '\0' => panic!("Lexer: Hit EOF when getting raw string at {line}:{col}. Missing closing '\"'['#']."),
            _ => { builder.push(c); lexer.i(1); },
        }
    }

    tokens.push(PosToken::new(Token::String(builder), line, col));
}

// TODO: More Testing
#[cfg(test)]
mod test {
    use crate::lexer::Token;

    use super::lex;

    #[test]
    fn only_single_line_comment() {
        let tokens = lex(r##"// Single line comment life
                                                      // // // // Single
                                                      // Comment!!!       "##)
        .unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::EOF);
    }

    #[test]
    #[should_panic]
    fn panic_multi_line_comment() {
        let _ = lex(r##"/* This comment does not have an ending!!!"##);
    }

    #[test]
    fn get_hex() {
        let tokens = lex(r##"0x1aAfFd90398536_24438f___12DDFffff"##).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens[0].token,
            Token::NumberHex("1aaffd9039853624438f12ddfffff".to_string())
        );
    }

    #[test]
    #[should_panic]
    fn panic_get_hex_empty() {
        let _ = lex(r##"0x"##);
    }

    #[test]
    #[should_panic]
    fn panic_get_hex_bad_char() {
        let _ = lex(r##"0xgg"##);
    }

    #[test]
    fn get_binary() {
        let tokens = lex(r##"0b1010__10000_01_01_011111___11101"##).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens[0].token,
            Token::NumberBinary("101010000010101111111101".to_string())
        );
    }

    #[test]
    #[should_panic]
    fn panic_get_binary_empty() {
        let _ = lex(r##"0b"##);
    }

    #[test]
    #[should_panic]
    fn panic_get_binary_bad_char() {
        let _ = lex(r##"0baa"##);
    }

    #[test]
    fn get_numbers() {
        let tokens = lex(r##"100 -110 3 1 -1 -0 -12 234__123 1_322 -1_234567689"##).unwrap();
        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0].token, Token::Number("100".to_string()));
        assert_eq!(tokens[1].token, Token::Number("-110".to_string()));
        assert_eq!(tokens[2].token, Token::Number("3".to_string()));
        assert_eq!(tokens[3].token, Token::Number("1".to_string()));
        assert_eq!(tokens[4].token, Token::Number("-1".to_string()));
        assert_eq!(tokens[5].token, Token::Number("-0".to_string()));
        assert_eq!(tokens[6].token, Token::Number("-12".to_string()));
        assert_eq!(tokens[7].token, Token::Number("234123".to_string()));
        assert_eq!(tokens[8].token, Token::Number("1322".to_string()));
        assert_eq!(tokens[9].token, Token::Number("-1234567689".to_string()));
    }

    #[test]
    fn blank() {
        let _ = lex(r##""##);
    }
}
