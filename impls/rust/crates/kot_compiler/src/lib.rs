mod eval;
mod kot_types;
mod lexer;
mod parser;
mod tokens;
mod writer;

//TODO Maybe don't use files, so this can be used without a filesystem.
//TODO Debug is determined by the built binary.

pub fn compile(/* List of names and contents */) -> () /* List of names and compiled binary */ {}

//TODO Maybe this should only be in the binary.
//TODO Build file has a different format.
pub fn build(/* Build file */) {}
