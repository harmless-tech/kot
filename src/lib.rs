#![allow(dead_code)] // TODO: Remove!!!

#[cfg(feature = "i64")]
type Int = i64;
#[cfg(not(feature = "i64"))]
type Int = i32;

pub mod args;
pub mod ast;
pub mod config;
pub mod lexer;
pub mod parser;
pub mod platform;
pub mod runner;

#[cfg(test)]
mod test {
    use crate::lexer;

    #[test]
    fn repo_kotfile() {
        let raw_kotfile = std::fs::read_to_string("./kotfile").unwrap();
        let (_tokens, _f_args) = lexer::lex(&raw_kotfile);

        // TODO: Test with cargo-test-repo-kotfile cmd.
    }
}
