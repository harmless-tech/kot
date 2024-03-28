use crate::{
    lexer::{lex, LexerError},
    Pos,
};

#[test]
fn test_dec_multi_period() {
    let lexer = lex(r"12.334.1");

    assert_eq!(
        lexer.err().unwrap().downcast_ref::<LexerError>().unwrap(),
        &LexerError::DecimalMoreThanOnePeriod {
            pos: Pos::new(1, 7)
        }
    );
}

#[test]
fn test_multi() {
    lex("1 == 1").unwrap();
    lex("1 != 11").unwrap();
    lex("1!=1").unwrap();
}
