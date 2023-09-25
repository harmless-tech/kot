#![allow(unused_variables)] // TODO: Remove!

// TODO: Cache AST of kotfile with hash. (feature?)
// TODO: Parser must handle string and raw string transformations.
// TODO: Parallel process args and file.
// TODO: Benchmark with smallvec to see if it is any faster.
// TODO: Int math?

use kot::{args, lexer};

fn main() {
    println!("Hello, world!"); // TODO: Remove!
                               // TODO: Panic hook to kill entire program?

    // TODO: Implement threads? Is it worth?
    #[cfg(feature = "threads")]
    println!("WARN: Threads feature is not implemented and may be removed in the future.");

    let (entry_args, env_config) = args::collect_args();

    let raw_kotfile = std::fs::read_to_string("./test/kotfile2").unwrap();
    let (tokens, f_args) = lexer::lex(&raw_kotfile);
}
