#![allow(unused_variables)] // TODO: Remove!

mod dot;

use crate::{
    ast::{Ast, Ast::Block, Types},
    config::Config,
    lexer::{ExToken, Token},
};
use std::{iter::Peekable, vec::IntoIter};

type ParseResult = anyhow::Result<Ast>;
type ParseResultLast = anyhow::Result<(Ast, ExToken)>;

// TODO: Improve error messages.
// TODO: No panic!!!!

// TODO: Handle {{}} in strings.

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

pub fn parse(tokens: Vec<ExToken>, config: &Config) -> ParseResult {
    let mut data = ParseData {
        tokens: tokens.into_iter().peekable(),
        config,
    };

    p_root(&mut data)
}

fn p_root(data: &mut ParseData) -> ParseResult {
    // TODO: Test blank kotfile.
    let (ast, token) = p(data)?;
    match token {
        ExToken {
            token: Token::Eof, ..
        } => {}
        ex => panic!(
            "Parser: Invalid token {:?} at ({}:{}). Expected Eof.",
            ex.token, ex.line, ex.col
        ),
    }
    Ok(ast)
}

/// Anything that can be on the global scope.
fn p(data: &mut ParseData) -> ParseResultLast {
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
            Token::Dot(id) => ast.push(dot::p_dot(id, pos, data)?),
            Token::Command(_) => todo!(),
            Token::Function => todo!(),
            Token::Let => todo!(),
            Token::Const => todo!(),
            Token::Guard => todo!(),
            Token::If => todo!(),
            Token::True => todo!(),
            Token::False => todo!(),
            Token::LCurly => {
                let (a, t) = p(data)?;
                ast.push(Ast::Scope(Box::new(a)));
                match t {
                    ExToken {
                        token: Token::RCurly,
                        ..
                    } => {}
                    _ => panic!("Parser: Scope not closed. ({}:{})", pos.0, pos.1),
                }
            }
            _ => break,
        }

        let ex = data.next();
        pos = ex.pos();
        token = ex.token;
    }

    Ok((
        Block(ast),
        ExToken {
            token,
            line: pos.0,
            col: pos.1,
        },
    ))
}
