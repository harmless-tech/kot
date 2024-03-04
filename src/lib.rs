#![deny(unsafe_code)]
#![allow(dead_code)] // TODO: Remove!!!

// TODO: Allow lexer, parser, interpreter to be added as a feature independently.
#[cfg(feature = "ast")]
pub mod ast;
#[cfg(feature = "arg-parse")]
pub mod entry_args;
#[cfg(feature = "lexer")]
pub mod lexer;
#[cfg(feature = "parser")]
pub mod parser;
mod platform;
#[cfg(feature = "tokens")]
pub mod token;
#[cfg(feature = "interpreter")]
pub mod interp;

// TODO: Struct that is generated at compile time that holds feature info.
// TODO: Kot uses COW semantics. (COW)

// TODO: Move?
/// 64 bit types
#[cfg(not(feature = "32-bit-types"))]
pub type Int = i64;
// 64 bit types
//#[cfg(not(feature = "32-bit-types"))]
//pub type TypeUInt = u64;
/// 64 bit types
#[cfg(not(feature = "32-bit-types"))]
pub type Float = f64;

/// 32 bit types
#[cfg(feature = "32-bit-types")]
pub type Int = i32;
// 32 bit types
//#[cfg(feature = "32-bit-types")]
//pub type TypeUInt = u32;
/// 32 bit types
#[cfg(feature = "32-bit-types")]
pub type Float = f32;

// TODO: Move somewhere better?
#[derive(Debug)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}
