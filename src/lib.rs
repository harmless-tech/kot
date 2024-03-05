#![allow(clippy::module_name_repetitions)]
// TODO: Remove!!!
#![allow(unused)]
//

pub mod data;
mod lexer;

mod interpreter;
mod parser;
#[cfg(test)]
mod test;

pub use interpreter::Interpreter;
pub use lexer::{lex, LexerError};
pub use parser::parse;

// TODO: Library should be wasm compliant.

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pos {
    line: usize,
    col: usize,
}
impl Pos {
    #[must_use]
    pub const fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}
impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{})", self.line, self.col)
    }
}
