use crate::data::{Ast, Token, Typing};

// TODO: Support floats. uints?
// TODO: Error with position.
pub fn parse_number(num: &Token) -> anyhow::Result<Typing> {
    match num {
        Token::NumberDecimal(num) => {
            assert!(!num.contains('.'), "No float support yet!");
            Ok(Typing::Int64(i64::from_str_radix(num, 10)?))
        }
        Token::NumberHex(_) => todo!(),
        Token::NumberOctal(_) => todo!(),
        Token::NumberBinary(_) => todo!(),
        _ => unreachable!(),
    }
}
