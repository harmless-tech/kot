#![allow(unused_variables)] // TODO: Remove!

use crate::{
    ast::{Ast, Ast::Exit, Types},
    config::Config,
    lexer::{ExToken, Token},
    Pos,
};
use std::{iter::Peekable, vec::IntoIter};

// TODO: Improve error messages.

// TODO: Get rid of this and pass by args?
// TODO: Remove config since right now you can only configure the vm and outside stuff.
struct ParseData<'a> {
    tokens: Peekable<IntoIter<ExToken>>,
    config: &'a Config,
}

impl<'a> ParseData<'a> {
    fn new() -> Self {
        todo!()
    }

    fn next(&mut self) -> ExToken {
        match self.tokens.next() {
            None => ExToken::eof(),
            Some(t) => t,
        }
    }

    fn peek(&mut self) -> &ExToken {
        match self.tokens.peek() {
            None => &ExToken {
                token: Token::Eof,
                line: 0,
                col: 0,
            },
            Some(t) => t,
        }
    }
}

pub fn parse(tokens: Vec<ExToken>, config: &Config) {
    let mut data = ParseData {
        tokens: tokens.into_iter().peekable(),
        config,
    };

    let ast = p_root(&mut data);
    todo!();
}

fn p_root(data: &mut ParseData) -> Vec<Ast> {
    // TODO: Test blank kotfile.
    p(data)
}

/// Anything that can be on the global scope.
fn p(data: &mut ParseData) -> Vec<Ast> {
    let mut ast = Vec::new();

    let ex = data.next();
    let mut pos = ex.pos();
    let mut token = ex.token;

    while token != Token::Eof {
        match token {
            Token::Ident(id) => match data.peek().token {
                Token::LParen => todo!(), // TODO: Function Call
                _ => ast.push(Ast::RunCommand(Types::Ident(id))),
            },
            Token::Dot(id) => ast.push(p_dot(id, pos, data)),
            Token::Command(_) => {}
            Token::Function => {}
            Token::Let => {}
            Token::Const => {}
            Token::Guard => {}
            Token::If => {}
            Token::True => {}
            Token::False => {}
            _ => panic!(
                "Parser: Invalid token {:?} at ({}:{}).",
                token, pos.0, pos.1
            ),
        }

        let ex = data.next();
        pos = ex.pos();
        token = ex.token;
    }

    todo!()
}

fn p_dot(id: String, pos: Pos, data: &mut ParseData) -> Ast {
    match id.as_str() {
        "args" => todo!(),
        "regex" => todo!(),
        "cmd" => todo!(),
        "return" => todo!(),
        "spawn" => todo!(),
        "parallel" => todo!(),
        "triplet" => todo!(),
        "arch" => todo!(),
        "os" => todo!(),
        "family" => todo!(),
        "panic" => todo!(), // How to handle string vs raw string
        "exit" => p_dot_exit(pos, data.next()),
        _ => panic!(
            "Parser: Invalid dot (.) type {} at ({}:{}).",
            id, pos.0, pos.1
        ),
    }
}

fn p_dot_exit(pos: Pos, token: ExToken) -> Ast {
    match token {
        ExToken {
            token: Token::Ident(id),
            ..
        } => Exit(Types::Ident(id)),
        ExToken {
            token: Token::Int(i),
            line,
            col,
        } => {
            let i: i32 = match i.parse() {
                Ok(i) => i,
                Err(_) => panic!(
                    "Parser: Could not parse i32 after .exit. ({}:{})",
                    pos.0, pos.1
                ),
            };
            Exit(Types::Integer(i.into()))
        }
        token => panic!(
            "Parser: Must have i32 or identifier after .exit. ({}:{})",
            pos.0, pos.1
        ),
    }
}
