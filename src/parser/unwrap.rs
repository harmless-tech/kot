use crate::{
    ast::{AstType, IdentFill},
    lexer::{ExToken, Token},
    parser::p_template,
    Int,
};
use bitflags::bitflags;

// TODO: Impl display for better looking errors.
bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct TypeId: u8 {
        const Ident = (1 << 0);
        const String = (1 << 1);
        const RawString = (1 << 2);
        const Command = (1 << 3);
        const Integer = (1 << 4);
    }
}

pub(super) fn p_unwrap_type(token: ExToken, unwrap: TypeId) -> anyhow::Result<AstType> {
    if unwrap.contains(TypeId::Ident) {
        if let Token::Ident(str) = token.token {
            return Ok(AstType::Ident(str));
        }
    }
    if unwrap.contains(TypeId::String) {
        if let Token::String(str) = token.token {
            let (str, fill) = p_template(str)?;
            return Ok(AstType::String(str, fill));
        }
    }
    if unwrap.contains(TypeId::RawString) {
        // TODO: Allow config option to parse raw string?
        if let Token::RawString(str) = token.token {
            return Ok(AstType::String(str, IdentFill::new()));
        }
    }
    if unwrap.contains(TypeId::Command) {
        if let Token::Command(str) = token.token {
            let (str, fill) = p_template(str)?;
            return Ok(AstType::Command(str, fill));
        }
    }
    if unwrap.contains(TypeId::Integer) {
        if let Token::Int(str) = token.token {
            let int: Int = str.parse()?;
            return Ok(AstType::Integer(int));
        }
    }

    Err(TypeError::UnmatchedToken(token, unwrap).into())
}

#[derive(Debug)]
pub enum TypeError {
    UnmatchedToken(ExToken, TypeId),
}
impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::write;
        match self {
            TypeError::UnmatchedToken(token, unwrap) => write(
                f,
                format_args!(
                    "Token {:?} did not match ont of the wanted types {unwrap:?}. ({}:{})",
                    token.token, token.line, token.col
                ),
            ),
        }
    }
}
impl std::error::Error for TypeError {}

// TODO: Test
