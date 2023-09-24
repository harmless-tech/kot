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
