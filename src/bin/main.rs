// TODO: Cache AST of kotfile with hash. (feature?)
// TODO: Parser must handle string and raw string transformations.
// TODO: Parallel process args and file.
// TODO: Benchmark with smallvec to see if it is any faster.

use kot::{args, lexer};

fn main() {
    println!("Hello, world!"); // TODO: Remove!
                               // TODO: Panic hook to kill entire program?

    let entry_args = {
        #[cfg(feature = "threads")]
        {
            std::thread::spawn(args::collect_args)
        }
        #[cfg(not(feature = "threads"))]
        args::collect_args()
    };

    let lexed = {
        #[cfg(feature = "threads")]
        {
            std::thread::spawn(|| {
                // TODO: Temp
                let raw_kotfile = std::fs::read_to_string("./test/kotfile2").unwrap();
                lexer::lex(&raw_kotfile)
            })
        }
        #[cfg(not(feature = "threads"))]
        {
            // TODO: Temp
            let raw_kotfile = std::fs::read_to_string("./test/kotfile2").unwrap();
            lexer::lex(&raw_kotfile)
        }
    };

    #[cfg(feature = "threads")]
    let entry_args = entry_args.join().unwrap();
    #[cfg(debug_assertions)]
    dbg!(&entry_args);

    #[cfg(feature = "threads")]
    let lexed = lexed.join().unwrap();
}
