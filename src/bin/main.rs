use kot::lexer;

fn main() -> anyhow::Result<()> {
    let temp_arg = std::env::args().nth(1).unwrap();

    let _lexed = {
        let file = std::fs::read_to_string(temp_arg).unwrap();
        lexer::lex(&file)
    };

    todo!()
}
