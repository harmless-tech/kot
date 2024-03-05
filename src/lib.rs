#![allow(clippy::module_name_repetitions)]
// TODO: Remove!!!
#![allow(unused)]
//

pub mod data;
pub mod lexer;

mod parser;
#[cfg(test)]
mod test;

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
