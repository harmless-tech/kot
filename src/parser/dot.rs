use crate::{
    ast::{Ast, Types},
    lexer::{ExToken, Token},
    parser::{ParseData, ParseResult},
    Pos,
};

pub(super) fn p_dot(id: String, pos: Pos, data: &mut ParseData) -> ParseResult {
    match id.as_str() {
        "object" => todo!(),
        "args" => todo!(),
        "regex" => todo!(),
        "cmd" => todo!(),
        "return" => todo!(),
        "inject" => todo!(), // Inject object fields into current scope or inject scope vars into scope above. TODO: This should require the flag unsafe_inject.
        "spawn" => dot_spawn(data),
        "parallel" => todo!(),
        "try" => todo!(),
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

// TODO: Allow spawning of one cmd without block.
fn dot_spawn(data: &mut ParseData) -> ParseResult {
    let block_start_pos = match data.next() {
        ExToken {
            token: Token::LCurly,
            line,
            col,
        } => (line, col),
        ex => panic!(
            "Parser: Expected block start ({{) after .spawn. ({}:{})",
            ex.line, ex.col
        ),
    };

    let mut ast = Vec::new();

    let ex = data.next();
    let mut pos = ex.pos();
    let mut token = ex.token;

    while token != Token::RCurly {
        match token {
            Token::Ident(id) => ast.push(Ast::SpawnCommand(Types::Ident(id))),
            // TODO: Parse command.
            Token::Command(cmd) => ast.push(Ast::SpawnCommand(Types::Command(cmd))),
            Token::Eof => panic!("Parser: Reached EOF before closing .spawn block. ({}:{})", block_start_pos.0, block_start_pos.1),
            token => panic!("Parser: Invalid token in .spawn block, {:?}. Only commands and idents are allowed. ({}:{})", token, pos.0, pos.1),
        }

        let ex = data.next();
        pos = ex.pos();
        token = ex.token;
    }

    Ok(Ast::Block(ast))
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
