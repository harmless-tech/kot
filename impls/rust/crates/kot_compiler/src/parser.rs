use crate::tokens::Token;

pub fn parse_tokens(mut tokens: Vec<Token>) {
    parse::root(&mut tokens);
}

mod parse {
    use crate::{
        parse_tokens::{Expression, Type},
        tokens::Token,
    };

    //TODO None of this works due to Token::LineNum. Maybe it should be removed for the mvp?
    pub fn root(tokens: &mut Vec<Token>) -> Result<Vec<Expression>, String> {
        let mut current_line = 0;
        let mut expressions: Vec<Expression> = Vec::new();

        while !tokens.is_empty() {
            match tokens.pop() {
                Some(Token::Data) => data(tokens, &mut expressions)?,
                Some(Token::Interface) => interface(tokens, &mut expressions)?,
                Some(Token::LineNum(num)) => current_line = num,
                _ => return Err("Token not supported. (root)".to_string()),
            }
        }

        Ok(expressions)
    }

    //TODO Maybe this should be switched to a match statement.
    fn data(tokens: &mut Vec<Token>, expressions: &mut Vec<Expression>) -> Result<(), String> {
        if let Some(Token::ID(name)) = tokens.pop() {
            if let Some(Token::Colon) = tokens.pop() {
                let d_type = data_type(tokens)?;

                if let Some(Token::Equals) = tokens.pop() {
                    let d_expr = expression(tokens)?;

                    expressions.push(Expression::Data(name, d_type, Box::new(d_expr)));

                    return Ok(());
                }

                return Err("No equals before expression. (data)".to_string());
            }
        }

        Err("No id and/or type splitter. (data)".to_string())
    }

    fn interface(tokens: &mut Vec<Token>, expressions: &mut Vec<Expression>) -> Result<(), String> {
        todo!()
    }

    fn data_type(tokens: &mut Vec<Token>) -> Result<Type, String> {
        Err("Not Impl".to_string())
    }
    fn type_name(tokens: &mut Vec<Token>) -> Result<Type, String> {
        todo!()
    }
    fn expression(tokens: &mut Vec<Token>) -> Result<Expression, String> {
        todo!()
    }
}

//TODO Better tests.
#[cfg(test)]
mod tests {
    use crate::lexer::{pre_process, remove_comments, tokenize};
    use std::{fs::File, io::Read};

    #[test]
    fn test_example_file() {
        let mut f_str = String::new();
        let mut file = File::open("../../../../specs/0/example.kot").unwrap();
        file.read_to_string(&mut f_str).unwrap();

        let contents = remove_comments(f_str);
        let contents = pre_process(contents);
        let t_list = tokenize("example.kot", &contents.0).unwrap();

        //TODO Parse function here.
        println!("\n\nParse tree:\n{}", 120);
    }
}
