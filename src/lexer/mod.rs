#[cfg(test)]
mod test;

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
            ('#', 'a'..='z') => {
                let (line, col) = (lexer.line, lexer.col);
                lexer.i(1);
                match MacroToken::try_from(get_ident(lexer).as_str()) {
                    Ok(t) => tokens.push(PosToken::new(Token::Macro(t), line, col)),
                    Err(_) => panic!("Lexer: Unknown macro at {line}:{col}."),
                }
            }
            ('#', next) => panic!(
                "Lexer: Unexpected token '{next}' after macro tag at {}:{}.",
                lexer.line, lexer.col
            ), // TODO: Return error instead of panic
            // Ident Split
            ('.', next) if next.is_ascii_alphabetic() || next == '_' => {
                token1!(Dot)
            }
            // Ident and Keywords
            ('_' | 'a'..='z' | 'A'..='Z', _) => {
                let (line, col) = (lexer.line, lexer.col);
                let ident = get_ident(lexer);
                match ident.as_str() {
                    "const" => tokens.push(PosToken::new(Token::Const, line, col)),
                    "let" => tokens.push(PosToken::new(Token::Let, line, col)),
                    "var" => tokens.push(PosToken::new(Token::Var, line, col)),
                    "true" => tokens.push(PosToken::new(Token::True, line, col)),
                    "false" => tokens.push(PosToken::new(Token::False, line, col)),
                    "enum" => tokens.push(PosToken::new(Token::Enum, line, col)),
                    "struct" => tokens.push(PosToken::new(Token::Struct, line, col)),
                    "rec" => tokens.push(PosToken::new(Token::Recursive, line, col)),
                    "fn" => tokens.push(PosToken::new(Token::Function, line, col)),
                    "return" => tokens.push(PosToken::new(Token::Return, line, col)),
                    "ret" => tokens.push(PosToken::new(Token::ScopeReturn, line, col)),
                    "trait" => tokens.push(PosToken::new(Token::Trait, line, col)),
                    "impl" => tokens.push(PosToken::new(Token::Implement, line, col)),
                    "where" => tokens.push(PosToken::new(Token::Where, line, col)),
                    "try" => tokens.push(PosToken::new(Token::Try, line, col)),
                    "if" => tokens.push(PosToken::new(Token::If, line, col)),
                    "guard" => tokens.push(PosToken::new(Token::Guard, line, col)),
                    "else" => tokens.push(PosToken::new(Token::Else, line, col)),
                    "switch" => tokens.push(PosToken::new(Token::Switch, line, col)),
                    "while" => tokens.push(PosToken::new(Token::While, line, col)),
                    "for" => tokens.push(PosToken::new(Token::For, line, col)),
                    "in" => tokens.push(PosToken::new(Token::In, line, col)),
                    "break" => tokens.push(PosToken::new(Token::Break, line, col)),
                    "mod" => tokens.push(PosToken::new(Token::Module, line, col)),
                    "pub" => tokens.push(PosToken::new(Token::Public, line, col)),
                    s => tokens.push(PosToken::new(Token::Ident(s.to_string()), line, col)),
                }
            }
            // Whitespace, No Newline
            (' ' | '\t' | '\r', _) => lexer.i(1),
            // Whitespace, Newline
            ('\n', _) => lexer.newline(),
            _ => panic!(
                "Lexer: Unexpected token at {}:{}. ({c1})",
                lexer.line, lexer.col
            ), // TODO: Return error instead of panic
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

    tokens.push(PosToken::new(Token::Character(c), line, col))
}

// TODO: Should string replacement {{  }} get processed in the mod or parser?
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

    lexer.i(1);

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

// TODO: No panic
fn get_ident(lexer: &mut Lexer) -> String {
    let (line, col) = (lexer.line, lexer.col);
    let mut builder = String::new();

    while lexer.within() {
        let c = lexer.peek();
        if c.is_alphanumeric() || c == '_' || c == '-' {
            builder.push(c);
            lexer.i(1);
        }
        else {
            break;
        }
    }

    if builder.is_empty() {
        // TODO: I do not think this can be triggered. Remove.
        panic!("Lexer: Ident starting at {line}:{col} is somehow empty.");
    }

    builder
}
