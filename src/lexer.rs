/// (Token, (Line, Col)))
pub type ExToken = (Token, (usize, usize));

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Ident(String),
    Dot(String),
    Command(String),
    Int(String),
    String(String),
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
    macro_rules! peak_i {
        ($e:expr) => {
            *match contents.get(index + $e) {
                Some(c) => c,
                None => &'\0',
            }
        };
    }
    macro_rules! peak {
        () => {
            peak_i!(1)
        };
    }
    macro_rules! token_peak {
        ($e:expr, $x1:ident, $x2:ident) => {
            if peak!() == $e {
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
        match c {
            '#' => {
                // Comment
                let i = skip_comment(&contents, index);
                if peak_i!(1) == 'k' && peak_i!(2) == 'o' && peak_i!(3) == 't' && peak_i!(4) == ' '
                {
                    config.push(contents[(index + 5)..i].iter().collect())
                }
                index = i;
            }
            '(' => token_i!(LParen),
            ')' => token_i!(RParen),
            '{' => token_i!(LCurly),
            '}' => token_i!(RCurly),
            ',' => token_i!(Comma),
            ':' => token_i!(Colon),
            '$' => token_i!(DollarSign),
            '=' => token_peak!('=', Equal, Assign),   // =, ==
            '!' => token_peak!('=', NotEqual, Not),   // !, !=
            '<' => token_peak!('=', LessEqual, Less), // <, <=
            '>' => token_peak!('=', GreaterEqual, Greater), // >, >=
            '&' => {
                // &&
                if peak!() == '&' {
                    token_i!(And);
                    index += 1;
                    col += 1;
                }
                else {
                    panic!("Unexpected token at {line}:{col}");
                }
            }
            '|' => {
                // ||
                if peak!() == '|' {
                    token_i!(Or);
                    index += 1;
                    col += 1;
                }
                else {
                    panic!("Unexpected token at {line}:{col}");
                }
            }
            '`' => {
                // Command
                todo!();
            }
            '"' => {
                // String
                // TODO: r becomes ident!!! (backtrack through #'s to find r)
                todo!();
            }
            '.' => {
                // Dot (. .. ..=)
                match (peak!(), peak_i!(2)) {
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
            'a'..='z' | 'A'..='Z' | '_' | '-' | '0'..='9' => {
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
                        if peak!().is_ascii_digit() {
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
            ' ' | '\t' | '\r' => {
                // Whitespace (No newline)
                index += 1;
                col += 1;
            }
            '\n' => {
                // Newline
                index += 1;
                line += 1;
                col = 1;
            }
            _ => panic!("Unexpected token at {line}:{col}"),
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

#[cfg(test)]
mod test {
    use crate::lexer::lex;
    use std::fs;

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
