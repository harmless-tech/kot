#![allow(clippy::module_name_repetitions)]

pub mod data;
pub mod lexer;

// TODO: Library should be wasm compliant.

#[derive(Debug)]
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
