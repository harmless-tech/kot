mod eval;
mod lexer;
mod parse_tokens;
mod parser;
mod tokens;
mod writer;

//TODO Maybe don't use files, so this can be used without a filesystem.
//TODO Debug is determined by the built binary.
pub fn compile(files: Vec<(String, String)>) -> Result<() /* Compiled structures. */, String> {
    /* List of names and compiled binary as the return types. */
    for (name, contents) in files {
        // Lexer
        println!("Start Lexer...");
        let contents = lexer::remove_comments(contents);
        println!("\n\nNo Comments:\n{}", contents);

        let contents = lexer::pre_process(contents);
        println!("\n\nPre-Process:\n{}", contents.0);
        println!("\n\nPre-Process Metadata:\n{}", contents.1);
        println!();

        println!("\n\nTokens:");
        let t_list = lexer::tokenize(name.as_str(), &contents.0).unwrap();
        println!("{:?}", t_list);
        println!("\n\nEnd of Lexer\n\n");

        // Parser
    }

    Ok(())
}

//TODO Export the compiled structure to the file provided.
pub fn export() -> bool {
    false
}

//TODO This should compile files then export them.
pub fn build(/* Build file */) {}
