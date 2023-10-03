#![allow(unused_variables)] // TODO: Remove!

mod dot;
mod unwrap;

// Export error types.
pub use unwrap::{TypeError, TypeId};

use crate::{
    ast::{Ast, Ast::Block, AstType, IdentFill},
    config::Config,
    lexer::{ExToken, Token},
};
use std::{cell::OnceCell, iter::Peekable, vec::IntoIter};

type ParseResult = anyhow::Result<Ast>;
type ParseResultLast = anyhow::Result<(Ast, ExToken)>;

// TODO: Improve error messages.
// TODO: No panic!!!!

// TODO: Get rid of this struct and pass by args?
// TODO: Remove config since right now you can only configure the interpreter and outside stuff?
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
    let (ast, token) = p_block(data)?;
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

fn p_block(data: &mut ParseData) -> ParseResultLast {
    let mut ast = Vec::new();

    let ex = data.next();
    let mut pos = ex.pos();
    let mut token = ex.token;

    while token != Token::Eof {
        match token {
            Token::Ident(id) => match data.peek().token {
                Token::LParen => todo!(), // TODO: Function Call
                _ => ast.push(Ast::RunCommand(AstType::Ident(id))),
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
                let (a, t) = p_block(data)?;
                ast.push(Ast::Scope(a.into()));
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

fn p_template(mut tmpl: String) -> anyhow::Result<(String, IdentFill)> {
    use aho_corasick::{AhoCorasickBuilder, AhoCorasickKind, Match, MatchKind};

    let cell = OnceCell::new();
    let ac = cell.get_or_init(|| {
        AhoCorasickBuilder::new()
            .kind(Some(AhoCorasickKind::DFA))
            .match_kind(MatchKind::LeftmostFirst)
            .build(["{{", "}}"])
            .unwrap()
    });

    // We assume that this is order correctly.
    let m: Vec<Match> = ac.find_iter(&tmpl).collect();
    if m.len() & 1 != 0 {
        panic!("TODO: Uneven templ");
    }
    let m: Vec<_> = m.chunks_exact(2).collect();

    const START: usize = 0;
    const END: usize = 1;

    let mut sub = 0;
    let mut ident_fills = Vec::new();

    for m in m {
        let s = m[START];
        let e = m[END];

        match (s.pattern().as_usize(), e.pattern().as_usize()) {
            (START, END) => {
                let pos = s.start() - sub;
                let mut section: String = tmpl.drain(pos..(e.end() - sub)).collect();
                sub += section.len();

                let ident: String = section.drain(2..(section.len() - 2)).collect();
                let ident = ident.trim();

                if ident.is_empty() {
                    panic!("TODO: No ident in Tmpl");
                }
                if !ident
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '.' && !ident.starts_with('.'))
                {
                    panic!("TODO: Not valid ident in Tmpl");
                }

                ident_fills.insert(0, (pos, String::from(ident)));
            }
            (START, START) => panic!("TODO: Tmpl in Tmpl"),
            (END, END) => panic!("TODO: No start Tmpl ({{{{)"),
            (END, START) => panic!("TODO: End Tmpl comes before start Tmpl"),
            _ => panic!("TODO: This should never happen"),
        }
    }

    Ok((tmpl, ident_fills))
}

#[cfg(test)]
mod test {
    use crate::parser::p_template;

    #[test]
    fn templates() {
        let _ =
            p_template("This strin {{   aaa     }} {{abc}} {{cc.cc.cc}}{{a}}".to_string()).unwrap();
        let _ =
            p_template("This strin {{   aaa     }} {{abc}} {{cc.cc.cc}}{{a}}".to_string()).unwrap();
        let _ =
            p_template("This strin {{   aaa     }} {{abc}} {{cc.cc.cc}}{{a}}".to_string()).unwrap();
        let _ =
            p_template("This strin {{   aaa     }} {{abc}} {{cc.cc.cc}}{{a}}".to_string()).unwrap();
        let _ =
            p_template("This strin {{   aaa     }} {{abc}} {{cc.cc.cc}}{{a}}".to_string()).unwrap();
    }
}
