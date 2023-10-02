use crate::{
    ast::{Ast, Types},
    lexer::{ExToken, Token},
    parser::{ParseData, ParseResult},
    Pos,
};

pub(super) fn p_dot(id: String, pos: Pos, data: &mut ParseData) -> ParseResult {
    match id.as_str() {
        "args" => todo!(),
        "regex" => todo!(),
        "cmd" => todo!(),
        "return" => todo!(),
        "inject" => todo!(), // Inject object fields into scope above. TODO: This should require the flag unsafe_inject.
        "spawn" => todo!(),
        "parallel" => todo!(),
        "triplet" => todo!(),
        "arch" => todo!(),
        "os" => todo!(),
        "family" => todo!(),
        "panic" => dot_panic(pos, data.next()), // How to handle string vs raw string
        "exit" => dot_exit(pos, data.next()),
        _ => panic!(
            "Parser: Invalid dot (.) type {} at ({}:{}).",
            id, pos.0, pos.1
        ),
    }
}

fn dot_panic(pos: Pos, token: ExToken) -> ParseResult {
    Ok(match token {
        ExToken {
            token: Token::Ident(id),
            ..
        } => Ast::Panic(Types::Ident(id)),
        ExToken {
            token: Token::String(str),
            ..
        } => {
            // TODO: Parse String.
            Ast::Panic(Types::String(str))
        }
        ExToken {
            token: Token::RawString(raw),
            ..
        } => {
            // TODO: Allow option for parsing raw string.
            Ast::Panic(Types::String(raw))
        }
        token => panic!(
            "Parser: Must have string, raw string, or identifier after .panic. ({}:{})",
            pos.0, pos.1
        ),
    })
}

fn dot_exit(pos: Pos, token: ExToken) -> ParseResult {
    Ok(match token {
        ExToken {
            token: Token::Ident(id),
            ..
        } => Ast::Exit(Types::Ident(id)),
        ExToken {
            token: Token::Int(i),
            line,
            col,
        } => {
            let i: i32 = match i.parse() {
                Ok(i) => i,
                Err(_) => panic!("Parser: Could not parse i32 after .exit. ({line}:{col})"),
            };
            Ast::Exit(Types::Integer(i.into()))
        }
        _ => panic!(
            "Parser: Must have i32 or identifier after .exit. ({}:{})",
            pos.0, pos.1
        ),
    })
}
