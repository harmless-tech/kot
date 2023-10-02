#![allow(unused_variables)] // TODO: Remove!

// TODO: Cache AST of kotfile with hash. (feature?)
// TODO: Parser must handle string and raw string transformations.
// TODO: Parallel process args and file.
// TODO: Benchmark with smallvec to see if it is any faster.
// TODO: Int math?

use kot::{args, config::Config, lexer, parser::parse};

fn main() -> anyhow::Result<()> {
    println!("TODO: Hello, world!"); // TODO: Remove!
                                     // TODO: Panic hook to kill entire program?

    // TODO: Implement threads? Is it worth?
    #[cfg(feature = "threads")]
    println!("WARN: Threads feature is not implemented and may be removed in the future.");

    // Load args and env var
    let (entry_args, env_config) = args::collect_args();

    // Load kotfile
    // TODO: Should search for kotfile from a list!
    let raw_kotfile = std::fs::read_to_string("./test/kotfiledev").unwrap();

    // Lex kotfile
    let (tokens, f_args) = lexer::lex(&raw_kotfile);

    // Config: File -> Env -> Args.
    let mut config = Config::from_config_slice(&f_args)?;
    if let Some(e) = env_config {
        config.configure(&e)?;
    }
    config.configure_slice(&entry_args.kot)?;
    check_config(&config);
    let config = config;

    // Parse kotfile
    let ast = parse(tokens, &config)?;
    dbg!(ast);

    // Cache?
    // Run AST

    Ok(())
}

fn check_config(config: &Config) {
    if config.version {
        // TODO: Git rev?
        println!("{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }
    #[cfg(not(feature = "i64"))]
    if config.require_i64 {
        panic!("Require i64 config option is on, but the kot binary was not built with the feature i64.");
    }
}
