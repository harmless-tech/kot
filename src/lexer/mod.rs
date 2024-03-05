use crate::{
    data::{PosToken, Token},
    Pos,
};
use std::{collections::VecDeque, iter::Fuse, str::Chars};

#[derive(Debug)]
struct Lexer<'a> {
    iter: Fuse<Chars<'a>>,
    tmp: VecDeque<char>,
    index: usize,
    line: usize,
    col: usize,
}
impl<'a> Lexer<'a> {
    fn new(item: &'a str) -> Self {
        Self {
            iter: item.chars().fuse(),
            tmp: VecDeque::new(),
            index: 0,
            line: 1,
            col: 1,
        }
    }

    fn within(&mut self) -> bool {
        if !self.tmp.is_empty() {
            return true;
        }

        match self.iter.next() {
            Some(c) => {
                self.tmp.push_back(c);
                true
            }
            None => false,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.peek_i(0)
    }

    fn peek_i(&mut self, offset: usize) -> Option<char> {
        while offset >= self.tmp.len() {
            match self.iter.next() {
                Some(c) => self.tmp.push_back(c),
                None => return None,
            }
        }

        self.tmp.get(offset).copied()
    }

    fn get(&mut self) -> Option<char> {
        let a = self.tmp.pop_front();
        let item = match &a {
            None => self.iter.next(),
            _ => a,
        };

        match &item {
            Some(c) => match c {
                '\0' => {}
                '\n' => self.newline(),
                _ => self.increment(1),
            },
            None => {}
        }

        item
    }

    fn skip_i(&mut self, amount: usize) {
        for _ in 0..amount {
            let _ = self.get();
        }
    }

    const fn current_pos(&self) -> Pos {
        Pos::new(self.line, self.col)
    }

    fn increment(&mut self, i: usize) {
        self.index += i;
        self.col += i;
    }

    fn newline(&mut self) {
        self.index += 1;
        self.line += 1;
        self.col = 1;
    }
}

#[derive(Debug)]
struct PunchToken {
    tokens: Vec<PosToken>,
}
impl PunchToken {
    const fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    fn add(&mut self, lexer: &mut Lexer, token: Token) {
        self.add_i(lexer, token, 0);
    }

    fn add1(&mut self, lexer: &mut Lexer, token: Token) {
        self.add_i(lexer, token, 1);
    }

    fn add2(&mut self, lexer: &mut Lexer, token: Token) {
        self.add_i(lexer, token, 2);
    }

    fn add3(&mut self, lexer: &mut Lexer, token: Token) {
        self.add_i(lexer, token, 3);
    }

    fn add_i(&mut self, lexer: &mut Lexer, token: Token, consume: usize) {
        self.tokens.push(PosToken::new(token, lexer.current_pos()));
        lexer.skip_i(consume);
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum LexerError {
    DecimalBadToken { c: char, pos: Pos },
    DecimalMoreThanOnePeriod { pos: Pos },
}
impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DecimalBadToken { c, pos } => {
                write!(f, "Lexer: Could not lex decimal, bad token '{c}' at {pos}.")
            }
            Self::DecimalMoreThanOnePeriod { pos } => {
                write!(
                    f,
                    "Lexer: Could not lex decimal, more than one period at {pos}."
                )
            }
        }
    }
}
impl std::error::Error for LexerError {}

pub fn lex(contents: &str) -> anyhow::Result<Vec<PosToken>> {
    let mut lexer = Lexer::new(contents);
    let lexer = &mut lexer;
    let mut tokens = PunchToken::new();

    while lexer.within() {
        let (c1, c2, c3) = (
            map_opt_char(lexer.peek().as_ref()),
            map_opt_char(lexer.peek_i(1).as_ref()),
            map_opt_char(lexer.peek_i(2).as_ref()),
        );
        match (c1, c2, c3) {
            // Comments
            ('/', '/', _) => todo!(), // Single line
            ('/', '*', _) => todo!(), // Multi line

            // TODO: Method Call
            ('(', _, _) => tokens.add1(lexer, Token::LParentheses),
            (')', _, _) => tokens.add1(lexer, Token::RParentheses),

            ('.', '.', '<') => tokens.add3(lexer, Token::RangeExclusive),
            ('.', '.', '=') => tokens.add3(lexer, Token::RangeInclusive),
            ('<', '<', '=') => tokens.add3(lexer, Token::AssignBitLeft),
            ('>', '>', '=') => tokens.add3(lexer, Token::AssignBitRight),

            ('*', '=', _) => tokens.add2(lexer, Token::AssignMathMultiply),
            ('/', '=', _) => tokens.add2(lexer, Token::AssignMathDivide),
            ('%', '=', _) => tokens.add2(lexer, Token::AssignMathModulus),
            ('+', '=', _) => tokens.add2(lexer, Token::AssignMathAdd),
            ('-', '=', _) => tokens.add2(lexer, Token::AssignMathSubtract),
            ('~', '=', _) => tokens.add2(lexer, Token::AssignBitNot),
            ('&', '=', _) => tokens.add2(lexer, Token::AssignBitAnd),
            ('^', '=', _) => tokens.add2(lexer, Token::AssignBitXor),
            ('|', '=', _) => tokens.add2(lexer, Token::AssignBitOr),
            ('&', '&', _) => tokens.add2(lexer, Token::BoolAnd),
            ('^', '^', _) => tokens.add2(lexer, Token::BoolXor),
            ('|', '|', _) => tokens.add2(lexer, Token::BoolOr),
            ('=', '=', _) => tokens.add2(lexer, Token::CompareEqual),
            ('!', '=', _) => tokens.add2(lexer, Token::CompareNotEqual),
            ('<', '=', _) => tokens.add2(lexer, Token::CompareLessEqual),
            ('>', '=', _) => tokens.add2(lexer, Token::CompareGreaterEqual),
            ('<', '<', _) => tokens.add2(lexer, Token::BitLeft),
            ('>', '>', _) => tokens.add2(lexer, Token::BitRight),

            ('=', _, _) => tokens.add1(lexer, Token::Assign),
            ('*', _, _) => tokens.add1(lexer, Token::MathMultiply),
            ('/', _, _) => tokens.add1(lexer, Token::MathDivide),
            ('%', _, _) => tokens.add1(lexer, Token::MathModulus),
            ('+', _, _) => tokens.add1(lexer, Token::MathAdd),
            ('-', _, _) => tokens.add1(lexer, Token::MathSubtract),
            ('!', _, _) => tokens.add1(lexer, Token::BoolNot),
            ('<', _, _) => tokens.add1(lexer, Token::CompareLess),
            ('>', _, _) => tokens.add1(lexer, Token::CompareGreater),
            ('~', _, _) => tokens.add1(lexer, Token::BitNot),
            ('&', _, _) => tokens.add1(lexer, Token::BitAnd),
            ('^', _, _) => tokens.add1(lexer, Token::BitXor),
            ('|', _, _) => tokens.add1(lexer, Token::BitOr),

            ('0', 'x', _) => todo!(), // Hex
            ('0', 'o', _) => todo!(), // Octal
            ('0', 'b', _) => todo!(), // Binary
            ('0'..='9', _, _) => tokens.tokens.push(get_decimal(lexer)?), // Decimal

            ('\'', _, _) => todo!(),                            // Char
            ('"', _, _) | ('#' | 'r', '"' | '#', _) => todo!(), // String

            // ('', _, _) => tokens.add1(lexer, Token::),
            (' ' | '\t' | '\r' | '\n', _, _) => lexer.skip_i(1),
            _ => todo!(),
        }
    }

    match tokens.tokens.last() {
        Some(last) => {
            if !last.token.eq(&Token::Eof) {
                tokens.tokens.push(PosToken::eof(lexer.current_pos()));
            }
        }
        None => tokens.tokens.push(PosToken::eof(lexer.current_pos())),
    }

    Ok(tokens.tokens)
}

fn map_opt_char(opt_char: Option<&char>) -> char {
    opt_char.map_or('\0', |c| *c)
}

fn get_decimal(lexer: &mut Lexer) -> anyhow::Result<PosToken> {
    let pos = lexer.current_pos();
    let mut builder = String::new();

    let mut period = false;

    while lexer.within() {
        let (c1, c2) = (
            map_opt_char(lexer.peek().as_ref()),
            map_opt_char(lexer.peek_i(1).as_ref()),
        );
        match (c1, c2) {
            ('0'..='9', _) => {
                builder.push(c1);
                lexer.skip_i(1);
            }
            ('.', '0'..='9') => {
                if period {
                    return Err(LexerError::DecimalMoreThanOnePeriod {
                        pos: lexer.current_pos(),
                    }
                    .into());
                }

                period = true;
                builder.push(c1);
                lexer.skip_i(1);
            }
            ('_', '0'..='9') => {
                lexer.skip_i(1);
            } // Skip
            _ => break,
        }
    }

    if builder.is_empty() {
        unreachable!();
    }

    Ok(PosToken::new(Token::Number(builder, 10), pos))
}
