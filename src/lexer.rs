use crate::Pos;
use std::fmt::{write, Display, Formatter};

#[derive(Debug)]
pub struct ExToken {
    pub token: Token,
    pub line: usize,
    pub col: usize,
}
impl ExToken {
    fn new(token: Token, line: usize, col: usize) -> Self {
        Self { token, line, col }
    }

    pub fn eof() -> Self {
        Self {
            token: Token::Eof,
            line: 0,
            col: 0,
        }
    }

    pub fn pos(&self) -> Pos {
        (self.line, self.col)
    }
}

//TODO: Not needed?
impl Display for ExToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write(
            f,
            format_args!("{:?} at ({}:{})", self.token, self.line, self.col),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Ident(String),
    Dot(String),
    Command(String),
    Int(String),
    String(String),
    RawString(String),
    Eof,

    LParen,
    RParen,
    LCurly,
    RCurly,
    Comma,
    Colon,
    DollarSign,

    Assign,
    Equal,
    Not,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,

    Function,
    Let,
    Const,
    Guard,
    If,
    Else,
    True,
    False,

    RangeInclusive,
    RangeExclusive,

    Plus,
}

// TODO: Use struct instead of macros to allow big functions to become smaller ones.
// TODO: No panic!!!!

/// (Vec of Tokens, Vec of configs)
pub fn lex(content: &str) -> (Vec<ExToken>, Vec<String>) {
    let contents: Vec<char> = content.chars().collect();

    let mut tokens: Vec<ExToken> = Vec::new();
    let mut config: Vec<String> = Vec::new();

    let mut index = 0_usize;
    let mut line = 1_usize;
    let mut col = 1_usize;

    macro_rules! token {
        ($x:ident) => {
            {
                tokens.push(ExToken::new(Token::$x, line, col));
            }
        };
        ($x:ident, $($v:expr), *) => {
            {
                tokens.push(ExToken::new(Token::$x($($v,)*), line, col));
            }
        };
    }
    macro_rules! token_i {
        ($x:ident) => {{
            token!($x);
            index += 1;
            col += 1;
        }};
    }
    macro_rules! peek_ref {
        ($e:expr) => {
            match contents.get(index + $e) {
                Some(c) => c,
                None => &'\0',
            }
        };
    }
    macro_rules! peek {
        ($e:expr) => {
            *peek_ref!($e)
        };
    }
    macro_rules! token_peek {
        ($next:ident, $e:expr, $x1:ident | $x2:ident) => {
            if $next == $e {
                token!($x1);
                index += 2;
                col += 2;
            }
            else {
                token_i!($x2);
            }
        };
    }

    while index < content.len() {
        let c = &contents[index];
        match (*c, peek!(1)) {
            ('#', next) => {
                // Comment
                let i = skip_comment(&contents, index);
                if next == 'k' && peek!(2) == 'o' && peek!(3) == 't' && peek!(4) == ' ' {
                    config.push(contents[(index + 5)..i].iter().collect());
                }
                index = i;
            }
            ('(', _) => token_i!(LParen),
            (')', _) => token_i!(RParen),
            ('{', _) => token_i!(LCurly),
            ('}', _) => token_i!(RCurly),
            (',', _) => token_i!(Comma),
            (':', _) => token_i!(Colon),
            ('$', _) => token_i!(DollarSign),
            ('+', _) => token_i!(Plus),
            ('=', next) => token_peek!(next, '=', Equal | Assign), // =, ==
            ('!', next) => token_peek!(next, '=', NotEqual | Not), // !, !=
            ('<', next) => token_peek!(next, '=', LessEqual | Less), // <, <=
            ('>', next) => token_peek!(next, '=', GreaterEqual | Greater), // >, >=
            ('&', next) => {
                // &&
                if next == '&' {
                    token_i!(And);
                    index += 1;
                    col += 1;
                }
                else {
                    panic!("Lexer: & is not a valid token, use && instead. {line}:{col}");
                }
            }
            ('|', next) => {
                // ||
                if next == '|' {
                    token_i!(Or);
                    index += 1;
                    col += 1;
                }
                else {
                    panic!("Lexer: | is not a valid token, use || instead. {line}:{col}");
                }
            }
            ('`', next) => {
                // Command
                if next == '`' {
                    token!(Command, "".to_string());
                    index += 2;
                    col += 2;
                }
                else {
                    let mut t_line = line;
                    let mut t_col = col;

                    let mut cmd_index = 0_usize;
                    let mut backslash = false;
                    loop {
                        cmd_index += 1;
                        t_col += 1;

                        match peek!(cmd_index) {
                            '`' => break,
                            '\\' => {
                                backslash = true;
                            }
                            '\r' => {}
                            '\n' => {
                                t_line += 1;
                                t_col = 0;

                                if backslash {
                                    backslash = false;
                                }
                                else {
                                    panic!("Lexer: Command at {line}:{col} is malformed. Newline before an ending ` was detected. Try adding a \\ before the newline.")
                                }
                            },
                            '\0' => panic!("Lexer: Command at {line}:{col} is malformed. EOF before an ending ` was detected."),
                            _ => backslash = false,
                        }
                    }

                    let word: String = contents[(index + 1)..(index + cmd_index)].iter().collect();
                    token!(Command, word);
                    index += cmd_index + 1;
                    line = t_line;
                    col = t_col + 1;
                }
            }
            ('"', next) => {
                // String
                if next == '"' {
                    token!(String, "".to_string());
                    index += 2;
                    col += 2;
                }
                else {
                    let mut str_index = 0_usize;
                    let mut backslash = false;
                    loop {
                        str_index += 1;
                        match peek!(str_index) {
                            '"' => {
                                if !backslash {
                                    break;
                                }
                                backslash = false;
                            }
                            '\\' => {
                                backslash = !backslash;
                            }
                            '\n' => panic!("Lexer: String at {line}:{col} is malformed. Newline before an ending \" was detected."),
                            '\0' => panic!("Lexer: String at {line}:{col} is malformed. EOF before an ending \" was detected."),
                            _ => backslash = false,
                        }
                    }

                    let word: String = contents[(index + 1)..(index + str_index)].iter().collect();
                    token!(String, word);
                    index += str_index + 1;
                    col = str_index + 1;
                }
            }
            ('r', '#' | '"') => {
                // Raw String
                let mut t_line = line;
                let mut t_col = col;

                let mut p = peek!(1);
                let mut hashes = 0_usize;
                loop {
                    match p {
                        '#' => hashes += 1,
                        '\0' => panic!("Lexer: Raw String at {line}:{col} is malformed. EOF before a starting \" was detected."),
                        _ => break,
                    }
                    p = peek!(hashes + 1);
                }
                let hashes = hashes;

                if p != '"' {
                    panic!("Lexer: Raw String at {line}:{col} is malformed. Missing a starting \".")
                }
                t_col += hashes + 1;

                let mut str_index = hashes + 1;
                let mut found_quote = false;
                let mut hashes_end = 0_usize;
                loop {
                    str_index += 1;
                    t_col += 1;
                    p = peek!(str_index);

                    match p {
                        '"' => {
                            if hashes == 0 {
                                break;
                            }

                            found_quote = true;
                            hashes_end = 0;
                        },
                        '#' => {
                            if found_quote {
                                hashes_end += 1;
                                if hashes_end == hashes {
                                    match peek!(str_index + 1) {
                                        '#' => {
                                            found_quote = false;
                                            hashes_end = 0;
                                        }
                                        _ => break
                                    }
                                }
                            }
                        }
                        '\n' => {
                            found_quote = false;
                            t_line += 1;
                            t_col = 0;
                        }
                        '\0' => panic!("Lexer: Raw String at {line}:{col} is malformed. EOF before an ending \" was detected. It is also possible that you have uneven #'s."),
                        _ => found_quote = false,
                    }
                }

                let word: String = contents[(index + hashes + 2)..(index + str_index - hashes)]
                    .iter()
                    .collect();
                token!(RawString, word);
                index += str_index + 1;
                line = t_line;
                col = t_col + 1;
            }
            ('.', next) => {
                // Dot (. .. ..=)
                match (next, peek!(2)) {
                    ('.', '=') => {
                        token!(RangeInclusive);
                        index += 3;
                        col += 3;
                    }
                    ('.', _) => {
                        token!(RangeExclusive);
                        index += 2;
                        col += 2;
                    }
                    _ => {
                        // TODO: This can skip over EOF?
                        let word_index = get_word(&contents, index + 1);
                        let word: String = contents[(index + 1)..word_index].iter().collect();
                        token!(Dot, word);
                        col += word_index - index;
                        index = word_index;
                    }
                }
            }
            ('-' | '0'..='9', _) => {
                let int_index = get_int(&contents, index);
                let int: String = contents[index..int_index].iter().collect();

                token!(Int, int);

                col += int_index - index;
                index = int_index;
            }
            ('a'..='z' | 'A'..='Z' | '_', _) => {
                let word_index = get_word(&contents, index);
                let word: String = contents[index..word_index].iter().collect();

                match word.as_str() {
                    "fn" => token!(Function),
                    "let" => token!(Let),
                    "const" => token!(Const),
                    "guard" => token!(Guard),
                    "if" => token!(If),
                    "else" => token!(Else),
                    "true" => token!(True),
                    "false" => token!(False),
                    _ => token!(Ident, word),
                }

                col += word_index - index;
                index = word_index;
            }
            (' ' | '\t' | '\r', _) => {
                // Whitespace (No newline)
                index += 1;
                col += 1;
            }
            ('\n', _) => {
                // Newline
                index += 1;
                line += 1;
                col = 1;
            }
            _ => panic!("Lexer: Unexpected token at {line}:{col}. ({c})"),
        }
    }

    token!(Eof);
    (tokens, config)
}

fn skip_comment(contents: &[char], mut index: usize) -> usize {
    while index < contents.len() && contents[index] != '\n' {
        index += 1;
    }
    index
}

fn get_int(contents: &[char], mut index: usize) -> usize {
    while index < contents.len() && (contents[index].is_numeric() || contents[index] == '-') {
        index += 1;
    }
    index
}

fn get_word(contents: &[char], mut index: usize) -> usize {
    while index < contents.len()
        && (contents[index].is_alphanumeric() || contents[index] == '_' || contents[index] == '.')
    {
        index += 1;
    }
    index
}

// TODO: Improve lexer testing.
#[cfg(test)]
mod test {
    use crate::lexer::{lex, Token};
    use std::fs;

    macro_rules! assert_lexer {
        ($t1:expr, $c1:expr, $l1:expr) => {{
            let t: &[Token] = &$t1;
            let c: &[&str] = &$c1;
            let l = lex($l1);

            for (l, t) in l.0.iter().zip(t) {
                assert!(l.token.eq(&t));
            }

            for (l, c) in l.1.iter().zip(c) {
                assert!(l.eq(&c.to_string()));
            }
        }};
    }

    #[test]
    fn lex_raw_str_1() {
        assert_lexer!(
            [
                Token::Let,
                Token::Ident("raw".to_string()),
                Token::Assign,
                Token::RawString(r#"""#.to_string())
            ],
            [],
            r###"let raw = r##"""##"###
        );
    }

    #[test]
    fn lex_kotfilelexer() {
        let file = fs::read_to_string("./test/kotfilelexer").unwrap();
        dbg!(lex(file.as_str()));
    }

    // #[test]
    // fn lex_kotfile2() {
    //     let file = fs::read_to_string("./test/kotfile2").unwrap();
    //     dbg!(lex(file.as_str()));
    // }
}
