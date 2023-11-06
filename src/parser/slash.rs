use crate::{
    ast::{Ast, AstType},
    lexer::{ExToken, Token, TokenSlash},
    parser::{
        p_block, p_template,
        unwrap::{p_unwrap_type, TypeId},
        ParseData, ParseResult,
    },
    platform, Pos,
};

pub(super) fn p_slash(id: TokenSlash, pos: Pos, data: &mut ParseData) -> ParseResult {
    match id {
        TokenSlash::Triplet => {
            let (strings, ast) = machine_check(data, None)?;
            Ok(Ast::Triplets(strings, ast))
        }
        TokenSlash::Os => {
            let (strings, ast) = machine_check(data, Some(platform::OSES))?;
            Ok(Ast::OSes(strings, ast))
        }
        TokenSlash::Family => {
            let (strings, ast) = machine_check(data, Some(platform::OS_FAMILIES))?;
            Ok(Ast::Families(strings, ast))
        }
        TokenSlash::Arch => {
            let (strings, ast) = machine_check(data, Some(platform::ARCHES))?;
            Ok(Ast::Arches(strings, ast))
        }
        TokenSlash::Run => todo!(),
        TokenSlash::Check => todo!(),
        TokenSlash::Regex => todo!(),
        TokenSlash::Object => todo!(),
        TokenSlash::Args => todo!(),
        TokenSlash::ArgsDef => todo!(),
        TokenSlash::Cmd => todo!(),
        TokenSlash::Parallel => todo!(),
        TokenSlash::Spawn => slash_spawn(data),
        TokenSlash::Panic => slash_panic(pos, data.next()),
        TokenSlash::Exit => slash_exit(pos, data.next()),
    }
}

// TODO: Allow spawning of one cmd without block?
fn slash_spawn(data: &mut ParseData) -> ParseResult {
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

    loop {
        let token = data.next();
        let pos = token.pos();
        match token.token {
            Token::Ident(id) => ast.push(Ast::SpawnCommand(AstType::ident(id))),
            Token::Command(cmd) => {
                let (str, fill) = p_template(cmd)?;
                ast.push(Ast::SpawnCommand(AstType::command(str, fill)))
            },
            Token::RCurly => break,
            Token::Eof => panic!("Parser: Reached EOF before closing .spawn block. ({}:{})", block_start_pos.0, block_start_pos.1),
            token => panic!("Parser: Invalid token in .spawn block, {:?}. Only commands and idents are allowed. ({}:{})", token, pos.0, pos.1),
        }
    }

    // Scope is not needed here since we cannot create vars.
    Ok(Ast::Block(ast))
}

fn machine_check(
    data: &mut ParseData,
    checklist: Option<&[&str]>,
) -> anyhow::Result<(Vec<String>, Box<Ast>)> {
    let mut strings = Vec::new();

    let ex = data.next();
    let mut pos = ex.pos();
    let mut token = ex.token;

    while let Token::String(t) = token {
        if let Some(check) = checklist {
            if check.contains(&t.as_str()) {
                strings.push(t);
            }
            else {
                panic!(
                    "Parser: Invalid option at {}:{}. Supported options are {:?}.",
                    pos.0, pos.1, check
                )
            }
        }
        else {
            strings.push(t);
        }

        let ex = data.next();
        pos = ex.pos();
        token = ex.token;
    }

    match token {
        Token::LCurly => {}
        _ => panic!(
            "Parser: Expected block start ({{) after .DOT [STRINGS]. Found token {:?}. ({}:{})",
            token, pos.0, pos.1
        ),
    };

    let ast = p_block(data)?;

    dbg!(&ast);

    match data.next() {
        ExToken {
            token: Token::RCurly,
            ..
        } => {}
        _ => panic!("Parser: Scope not closed. ({}:{})", pos.0, pos.1),
    }

    Ok((strings, Ast::Scope(ast.into()).into()))
}

fn slash_panic(pos: Pos, token: ExToken) -> ParseResult {
    let t = p_unwrap_type(token, TypeId::Ident | TypeId::String | TypeId::RawString)?;
    Ok(Ast::Panic(t))
}

fn slash_exit(pos: Pos, token: ExToken) -> ParseResult {
    let t = p_unwrap_type(token, TypeId::Ident | TypeId::Integer)?;
    Ok(Ast::Exit(t))
}
