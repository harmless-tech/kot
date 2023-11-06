#![allow(unused_variables)] // TODO: Remove!

mod ops;
mod slash;
mod unwrap;

// Export error types.
pub use unwrap::{TypeError, TypeId};

use crate::{
    ast::{Ast, AstType, IdentFill},
    config::Config,
    lexer::{ExToken, Token},
    parser::ops::p_assign,
};
use std::{cell::OnceCell, iter::Peekable, vec::IntoIter};

type ParseResult = anyhow::Result<Ast>;
type ParseResultLast = anyhow::Result<(Ast, ExToken)>;

// TODO: Improve error messages.
// TODO: No panic!!!!

// TODO: Get rid of this struct and pass by args?
// TODO: Remove config since right now you can only configure the interpreter and outside stuff?
// TODO: Maybe this should just be a vector that can be push and pulled from.
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
    let ast = p_block(data)?;
    match data.peek() {
        ExToken {
            token: Token::Eof, ..
        } => {}
        ExToken {
            token: Token::RCurly,
            line,
            col,
        } => panic!(
            "Parser: Invalid token {:?} at ({}:{}). A scope was closed that was never opened.",
            Token::RCurly,
            line,
            col
        ),
        ex => panic!(
            "Parser: Invalid token {:?} at ({}:{}). Expected Eof.",
            ex.token, ex.line, ex.col
        ),
    }
    Ok(ast)
}

// TODO: p_block opt!!!

// `let ExToken { token: TOKEN, .. } = data.next else { panic!() };` Should never panic.
fn p_block(data: &mut ParseData) -> ParseResult {
    let mut ast: Vec<Ast> = Vec::new();

    loop {
        let look = data.peek();
        match &look.token {
            Token::Ident(id) => {
                let ExToken {
                    // TODO: Macro?
                    token: Token::Ident(id),
                    line,
                    col,
                } = data.next()
                else {
                    panic!()
                };

                let peek = data.peek();
                match peek.token {
                    Token::Assign => ast.push(p_assign(id, (line, col), data)?),
                    Token::LParen => todo!(), // TODO: Function Call
                    _ => ast.push(Ast::Type(AstType::Ident(id))),
                }
            }
            Token::Slash(id) => {
                let ExToken {
                    token: Token::Slash(id),
                    line,
                    col,
                } = data.next()
                else {
                    panic!()
                };
                ast.push(slash::p_slash(id, (line, col), data)?)
            }
            Token::Command(_) => todo!(),
            Token::Function => todo!(),
            Token::Return => todo!(),
            Token::Let => todo!(),
            Token::Const => todo!(),
            Token::Guard => todo!(),
            Token::If => todo!(),
            // TODO: true/false?
            Token::LCurly => {
                let ExToken {
                    token: Token::LCurly,
                    line,
                    col,
                } = data.next()
                else {
                    panic!()
                };
                let a = p_block(data)?;
                ast.push(Ast::Scope(a.into()));
                match data.next().token {
                    Token::RCurly => {}
                    _ => panic!("Parser: Scope not closed. ({line}:{col})"),
                }
            }
            Token::Eof => break,
            _ => break,
        }
    }

    Ok(Ast::Block(ast))
}

// TODO: Should either eval a statement or make a p_block.
fn p_block_opt(data: &mut ParseData) -> ParseResult {
    todo!()
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
    use crate::{
        config::Config,
        lexer::lex,
        parser::{p_template, parse},
    };
    use std::fs;

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

    #[test]
    fn parse_kotfile3() {
        let file = fs::read_to_string("./test/kotfile3").unwrap();
        let (tokens, conf_strings) = lex(file.as_str());
        let config = Config::from_config_slice(&conf_strings).unwrap();

        parse(tokens, &config).unwrap();
    }
}
