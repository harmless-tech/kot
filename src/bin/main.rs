// TODO: Should have special module on non-wasm platforms for file system access?

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    // let string = std::fs::read_to_string("./test/iter_1/large.tmp")?;

    let lex = kot::lex("1 + ------1000000")?;
    let parse = kot::parse(lex)?;

    let mut int = kot::Interpreter::new(parse);
    let result = int.run()?;

    println!("Got final value: {result:?}");

    Ok(())
}
