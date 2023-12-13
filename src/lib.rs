#![deny(unsafe_code)]
#![allow(dead_code)] // TODO: Remove!!!

// TODO: Allow lexer, parser, interpreter to be added as a feature independently.
pub mod entry_args;
pub mod lexer;
pub mod platform;

// TODO: Struct that is generated at compile time that holds feature info.
// TODO: Kot uses COW semantics. (COW)

// TODO: Move?
/// 64 bit types
#[cfg(not(feature = "32-bit-types"))]
pub type TypeInt = i64;
// 64 bit types
//#[cfg(not(feature = "32-bit-types"))]
//pub type TypeUInt = u64;
/// 64 bit types
#[cfg(not(feature = "32-bit-types"))]
pub type TypeFloat = f64;

/// 32 bit types
#[cfg(feature = "32-bit-types")]
pub type TypeInt = i32;
// 32 bit types
//#[cfg(feature = "32-bit-types")]
//pub type TypeUInt = u32;
/// 32 bit types
#[cfg(feature = "32-bit-types")]
pub type TypeFloat = f32;

// TODO: Move somewhere better?
#[derive(Debug)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}
