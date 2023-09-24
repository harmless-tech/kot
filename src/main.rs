#![allow(dead_code)] // TODO: Remove!!!

mod ast;
mod config;
mod entry;
mod lexer;
mod parser;
mod platform;

// TODO: Cache AST of kotfile with hash. (feature?)
// TODO: Parser must handle string and raw string transformations.
// TODO: Parallel process args and file.
// TODO: Benchmark with smallvec to see if it is any faster.

#[cfg(feature = "i64")]
type Int = i64;
#[cfg(not(feature = "i64"))]
type Int = i32;

fn main() {
    println!("Hello, world!"); // TODO: Remove!
                               // TODO: Panic hook to kill entire program?

    let entry_args = {
        #[cfg(feature = "threads")]
        {
            std::thread::spawn(entry::collect_args)
        }
        #[cfg(not(feature = "threads"))]
        entry::collect_args()
    };

    // TODO: Lexer

    #[cfg(feature = "threads")]
    let entry_args = entry_args.join().unwrap();
    #[cfg(debug_assertions)]
    dbg!(&entry_args);
}
