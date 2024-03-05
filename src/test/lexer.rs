use crate::{
    lexer::{lex, LexerError},
    Pos,
};

macro_rules! assert_lexer {
    ($t1:expr, $l1:expr) => {{
        let t: &[PosToken] = &$t1;
        let l = lex($l1).unwrap();

        for (l, t) in l.iter().zip(t) {
            assert_eq!(l, t);
        }
    }};
}

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
