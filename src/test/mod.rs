#[macro_use]
mod macros {
    macro_rules! assert_lexer {
        ($t1:expr, $l1:ident) => {{
            let t: &[PosToken] = &$t1;
            let l = &$l1;

            for (l, t) in l.iter().zip(t) {
                assert_eq!(l, t);
            }
        }};
    }
}

mod iter_1;
mod iter_2;
mod lexer;
mod parser;
