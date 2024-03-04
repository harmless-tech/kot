use crate::{
    data::{PosToken, Token},
    lexer::lex,
    Pos,
};
use std::fs::read_to_string;

#[test]
fn kot_1() {
    let str = read_to_string("./test/iter_1/1.kot").unwrap();

    assert_lexer!(
        [
            PosToken::new(Token::Number("1".to_string(), 10), Pos::new(1, 1)),
            PosToken::new(Token::MathAdd, Pos::new(1, 3)),
            PosToken::new(Token::Number("1".to_string(), 10), Pos::new(1, 5)),
        ],
        &str
    );
}

#[test]
fn kot_2() {
    let str = read_to_string("./test/iter_1/2.kot").unwrap();

    assert_lexer!(
        [
            PosToken::new(Token::LParentheses, Pos::new(1, 9)),
            PosToken::new(Token::Number("1".to_string(), 10), Pos::new(1, 10)),
            PosToken::new(Token::MathAdd, Pos::new(1, 11)),
            PosToken::new(Token::Number("2".to_string(), 10), Pos::new(1, 12)),
            PosToken::new(Token::RParentheses, Pos::new(1, 13)),
            PosToken::new(Token::MathDivide, Pos::new(1, 15)),
            PosToken::new(Token::Number("3".to_string(), 10), Pos::new(3, 9)),
            PosToken::new(Token::MathAdd, Pos::new(3, 11)),
            PosToken::new(Token::Number("10".to_string(), 10), Pos::new(3, 13)),
            PosToken::new(Token::MathMultiply, Pos::new(3, 16)),
            PosToken::new(Token::Number("70".to_string(), 10), Pos::new(3, 17)),
        ],
        &str
    );
}
