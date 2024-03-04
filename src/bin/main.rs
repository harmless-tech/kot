use kot::lexer;

// TODO: Should have special module on non-wasm platforms for file system access?

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let string = std::fs::read_to_string("./test/iter_1/large.tmp")?;
    let _ = lexer::lex(&string);

    Ok(())
}
