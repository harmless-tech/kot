use crate::{
    ast::{Ast, AstType},
    lexer::{ExToken, Token},
    parser::{
        p_template,
        unwrap::{p_unwrap_type, TypeId},
        ParseData, ParseResult,
    },
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
// TODO: Shortcut for blocks.
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
            Token::Ident(id) => ast.push(Ast::SpawnCommand(AstType::Ident(id))),
            Token::Command(cmd) => {
                let (str, fill) = p_template(cmd)?;
                ast.push(Ast::SpawnCommand(AstType::Command(str, fill)))
            },
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
    let t = p_unwrap_type(token, TypeId::Ident | TypeId::String | TypeId::RawString)?;
    Ok(Ast::Panic(t))
}

fn dot_exit(pos: Pos, token: ExToken) -> ParseResult {
    let t = p_unwrap_type(token, TypeId::Ident | TypeId::Integer)?;
    Ok(Ast::Exit(t))
}
