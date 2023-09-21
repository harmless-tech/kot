#![allow(dead_code)] // TODO: Remove!!!

mod ast;
mod lexer;
mod platform;

// TODO: Cache AST of kotfile with hash. (feature?)

#[cfg(feature = "i64")]
type Int = i64;
#[cfg(not(feature = "i64"))]
type Int = i32;

fn main() {
    println!("Hello, world!");
}
