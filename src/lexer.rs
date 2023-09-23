/// (Token, (Line, Col)))
pub type ExToken = (Token, (usize, usize));

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

    Let,
    Const,
    If,
    Else,
    True,
    False,

    RangeInclusive,
    RangeExclusive,
}

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
                tokens.push((Token::$x, (line, col)));
            }
        };
        ($x:ident, $($v:expr), *) => {
            {
                tokens.push((Token::$x($($v,)*), (line, col)));
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
    macro_rules! peak_ref {
        ($e:expr) => {
            match contents.get(index + $e) {
                Some(c) => c,
                None => &'\0',
            }
        };
    }
    macro_rules! peak {
        ($e:expr) => {
            *peak_ref!($e)
        };
    }
    macro_rules! token_peak {
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
        match (*c, peak!(1)) {
            ('#', next) => {
                // Comment
                let i = skip_comment(&contents, index);
                if next == 'k' && peak!(2) == 'o' && peak!(3) == 't' && peak!(4) == ' ' {
                    config.push(contents[(index + 5)..i].iter().collect())
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
            ('=', next) => token_peak!(next, '=', Equal | Assign), // =, ==
            ('!', next) => token_peak!(next, '=', NotEqual | Not), // !, !=
            ('<', next) => token_peak!(next, '=', LessEqual | Less), // <, <=
            ('>', next) => token_peak!(next, '=', GreaterEqual | Greater), // >, >=
            ('&', next) => {
                // &&
                if next == '&' {
                    token_i!(And);
                    index += 1;
                    col += 1;
                }
                else {
                    panic!("& is not a valid token, use && instead. {line}:{col}");
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
                    panic!("| is not a valid token, use || instead. {line}:{col}");
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

                        match peak!(cmd_index) {
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
                                    panic!("Command at {line}:{col} is malformed. Newline before an ending ` was detected. Try adding a \\ before the newline.")
                                }
                            },
                            '\0' => panic!("Command at {line}:{col} is malformed. EOF before an ending ` was detected."),
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
                        match peak!(str_index) {
                            '"' => {
                                if !backslash {
                                    break;
                                }
                                backslash = false;
                            }
                            '\\' => {
                                backslash = !backslash;
                            }
                            '\n' => panic!("String at {line}:{col} is malformed. Newline before an ending \" was detected."),
                            '\0' => panic!("String at {line}:{col} is malformed. EOF before an ending \" was detected."),
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

                let mut p = peak!(1);
                let mut hashes = 0_usize;
                loop {
                    match p {
                        '#' => hashes += 1,
                        '\0' => panic!("Raw String at {line}:{col} is malformed. EOF before a starting \" was detected."),
                        _ => break,
                    }
                    p = peak!(hashes + 1);
                }
                let hashes = hashes;

                if p != '"' {
                    panic!("Raw String at {line}:{col} is malformed. Missing a starting \".")
                }
                t_col += hashes + 1;

                let mut str_index = hashes + 1;
                let mut found_quote = false;
                let mut hashes_end = 0_usize;
                loop {
                    str_index += 1;
                    t_col += 1;
                    p = peak!(str_index);

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
                                    match peak!(str_index + 1) {
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
                        '\0' => panic!("Raw String at {line}:{col} is malformed. EOF before an ending \" was detected. It is also possible that you have uneven #'s."),
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
                match (next, peak!(2)) {
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
                        let word: String = contents[index..word_index].iter().collect();
                        token!(Dot, word);
                        col += word_index - index;
                        index = word_index;
                    }
                }
            }
            ('a'..='z' | 'A'..='Z' | '_' | '-' | '0'..='9', next) => {
                let word_index = get_word(&contents, index);
                let word: String = contents[index..word_index].iter().collect();

                macro_rules! insert_word {
                    () => {
                        match word.as_str() {
                            "let" => token!(Let),
                            "const" => token!(Const),
                            "if" => token!(If),
                            "else" => token!(Else),
                            "true" => token!(True),
                            "false" => token!(False),
                            _ => token!(Ident, word),
                        }
                    };
                }
                macro_rules! insert_int {
                    () => {{
                        token!(Int, word);
                    }};
                }

                match *c {
                    '-' => {
                        if next.is_ascii_digit() {
                            insert_word!();
                        }
                        else {
                            insert_int!();
                        }
                    }
                    '0'..='9' => insert_int!(),
                    _ => insert_word!(),
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
            _ => panic!("Unexpected token at {line}:{col}. ({c})"),
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

fn get_word(contents: &[char], mut index: usize) -> usize {
    while index < contents.len()
        && (contents[index].is_alphanumeric()
            || contents[index] == '_'
            || contents[index] == '-'
            || contents[index] == '.')
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
                assert!(l.0.eq(&t));
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
