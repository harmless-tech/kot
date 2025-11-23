use crate::token::PosToken;
use std::{collections::VecDeque, iter::Fuse, str::Chars};

macro_rules! force_char {
    ($c:expr) => {
        $c.map_or('\0', |c| c)
    };
}

macro_rules! whitespace {
    () => {
        ' ' | '\t' | '\n' | '\r'
    };
}

#[derive(Debug)]
struct Lexer<'a> {
    iter: Fuse<Chars<'a>>,
    tmp: VecDeque<char>,
    index: usize,
    line: usize,
    col: usize,
}
impl<'a> Lexer<'a> {
    fn new(unit: &'a str) -> Self {
        Self {
            iter: unit.chars().fuse(),
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

    #[inline(always)]
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
        let c = self.tmp.pop_front();
        let item = match &c {
            None => self.iter.next(),
            _ => c,
        };

        match item {
            Some(c) => match c {
                '\0' => {}
                '\n' => self.newline(),
                _ => self.increment(1),
            },
            None => {}
        }

        item
    }

    fn skip_i(&mut self, amt: usize) {
        for _ in 0..amt {
            let _ = self.get();
        }
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

pub fn lex(contents: &str) -> anyhow::Result<Vec<PosToken>> {
    let mut lexer = Lexer::new(contents);
    let lexer = &mut lexer;
    let mut tokens = Vec::new(); // TODO: Punch tokens? Maybe macro instead...

    while lexer.within() {
        let (c1, c2, c3) = (
            force_char!(lexer.peek_i(0)),
            force_char!(lexer.peek_i(1)),
            force_char!(lexer.peek_i(2)),
        );

        match (c1, c2, c3) {
            ('/', '/', '!') => todo!(), // Doc Module Comment
            ('/', '/', _) => todo!(),   // Line Comment
            ('/', '*', '*') => todo!(), // Doc Block Comment
            ('/', '*', _) => todo!(),   // Block Comment
            ('#', '!', _) => todo!(),   // Shebang

            ('-', '>', _) => todo!(), // Arrow

            ('.', '.', '<') => todo!(), // Range Exclusive
            ('.', '.', '=') => todo!(), // Range Inclusive

            ('<', '<', '=') => todo!(), // Bitwise Shift Left Assign
            ('<', '<', _) => todo!(),   // Bitwise Shift Left
            ('>', '>', '=') => todo!(), // Bitwise Shift Right Assign
            ('>', '>', _) => todo!(),   // Bitwise Shift Right

            ('=', '=', _) => todo!(), // Compare Equal
            ('!', '=', _) => todo!(), // Compare Not Equal
            ('<', '=', _) => todo!(), // Compare Less Than Or Equal
            ('<', _, _) => todo!(),   // Compare Less Than
            ('>', '=', _) => todo!(), // Compare Greater Than Or Equal
            ('>', _, _) => todo!(),   // Compare Greater Than

            ('!', _, _) => todo!(),   // Boolean Not
            ('&', '&', _) => todo!(), // Boolean And
            ('|', '|', _) => todo!(), // Boolean Or
            ('^', '^', _) => todo!(), // Boolean Xor

            ('~', '=', _) => todo!(), // Bitwise Negate Assign
            ('~', _, _) => todo!(),   // Bitwise Negate
            ('&', '=', _) => todo!(), // Bitwise And Assign
            ('&', _, _) => todo!(),   // Bitwise And
            ('|', '=', _) => todo!(), // Bitwise Or Assign
            ('|', _, _) => todo!(),   // Bitwise Or
            ('^', '=', _) => todo!(), // Bitwise Xor Assign
            ('^', _, _) => todo!(),   // Bitwise Xor

            ('+', '=', _) => todo!(), // Math Add Assign
            ('+', _, _) => todo!(),   // Math Add
            ('-', '=', _) => todo!(), // Math Sub Assign
            ('-', _, _) => todo!(),   // Math Sub
            ('*', '=', _) => todo!(), // Math Mul Assign
            ('*', _, _) => todo!(),   // Pointer or Math Mul
            ('/', '=', _) => todo!(), // Math Div Assign
            ('/', _, _) => todo!(),   // Math Div
            ('%', '=', _) => todo!(), // Math Mod Assign
            ('%', _, _) => todo!(),   // Math Mod

            ('\'', _, _) => todo!(),  // Allocator Mark
            ('.', '&', _) => todo!(), // Reference
            ('.', '*', _) => todo!(), // Dereference
            ('.', '?', _) => todo!(), // Optional Unwrap
            ('.', '!', _) => todo!(), // Optional Force Unwrap
            ('?', _, _) => todo!(),   // Optional

            ('(', _, _) => todo!(), // Left Paren
            (')', _, _) => todo!(), // Right Paren
            ('[', _, _) => todo!(), // Left Brace
            (']', _, _) => todo!(), // Right Brace
            ('{', _, _) => todo!(), // Left Bracket
            ('}', _, _) => todo!(), // Right Bracket
            (',', _, _) => todo!(), // Comma
            (';', _, _) => todo!(), // Semicolon
            ('=', _, _) => todo!(), // Assign

            (':', ':', _) => todo!(), // Static Split
            ('.', '.', _) => todo!(), // Extend
            (':', _, _) => todo!(),   // Colon
            ('.', _, _) => todo!(),   // Split

            ('"', '"', '"') => todo!(), // RawString
            ('"', _, _) => todo!(),     // String
            ('c', '"', _) => todo!(),   // Char

            ('0', 'x', _) => todo!(),     // Hex Number
            ('0', 'o', _) => todo!(),     // Octal Number
            ('0', 'b', _) => todo!(),     // Binary Number
            ('0'..='9', _, _) => todo!(), // Number

            ('@', _, _) => todo!(), // Compiler Directive

            ('_' | 'a'..='z' | 'A'..='Z', _, _) => todo!(), // Ident

            (whitespace!(), _, _) => lexer.skip_i(1),
            ('\0', _, _) => todo!(), // Eof
            _ => todo!(),            // TODO: Eof?
        }
    }

    todo!();

    Ok(tokens)
}
