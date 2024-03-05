use crate::{lexer::lex, parser::parse};

#[test]
fn test_parser() {
    println!("{}", parse(lex(r"-1").unwrap()).unwrap());
    println!("{}", parse(lex(r"1-1").unwrap()).unwrap());
    println!("{}", parse(lex(r"1 - (10)").unwrap()).unwrap());
    println!("{}", parse(lex(r"- (10)").unwrap()).unwrap());
    println!(
        "{}",
        parse(lex(r"-(10 - 10 + 100 --- 10)").unwrap()).unwrap()
    );

    println!(
        "{}",
        parse(lex(r"1 | 10  & 12 | ( 1 & 1)").unwrap()).unwrap()
    );
}
