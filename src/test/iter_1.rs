use crate::{
    data::{Ast, BinaryOperation, PosAst, PosToken, Token, Typing},
    lexer::lex,
    parser::parse,
    Pos,
};
use std::{fs::read_to_string, ops::Deref};

#[test]
fn kot_1() {
    let str = read_to_string("./test/iter_1/1.kot").unwrap();
    let lex = lex(&str).unwrap();

    assert_lexer!(
        [
            PosToken::new(Token::NumberDecimal("1".to_string()), Pos::new(1, 1)),
            PosToken::new(Token::MathAdd, Pos::new(1, 3)),
            PosToken::new(Token::NumberDecimal("2".to_string()), Pos::new(1, 5)),
        ],
        lex
    );

    let ast = parse(lex).unwrap();
    if let PosAst {
        ast: Ast::Root(a),
        pos,
    } = ast
    {
        assert_eq!(pos, Pos::new(0, 0));

        if let PosAst {
            ast: Ast::BinOp(BinaryOperation::Add, a1, a2),
            pos,
        } = *a
        {
            assert_eq!(pos, Pos::new(1, 3));

            if let PosAst {
                ast: Ast::Value(Typing::Int64(int)),
                pos,
            } = *a1
            {
                assert_eq!(pos, Pos::new(1, 1));
                assert_eq!(int, 1);
            }
            else {
                panic!()
            }

            if let PosAst {
                ast: Ast::Value(Typing::Int64(int)),
                pos,
            } = *a2
            {
                assert_eq!(pos, Pos::new(1, 5));
                assert_eq!(int, 2);
            }
            else {
                panic!()
            }
        }
        else {
            panic!()
        }
    }
    else {
        panic!();
    }
}

#[test]
fn kot_2() {
    let str = read_to_string("./test/iter_1/2.kot").unwrap();
    let lex = lex(&str).unwrap();

    assert_lexer!(
        [
            PosToken::new(Token::LParentheses, Pos::new(1, 9)),
            PosToken::new(Token::NumberDecimal("1".to_string()), Pos::new(1, 10)),
            PosToken::new(Token::MathAdd, Pos::new(1, 11)),
            PosToken::new(Token::NumberDecimal("2".to_string()), Pos::new(1, 12)),
            PosToken::new(Token::RParentheses, Pos::new(1, 13)),
            PosToken::new(Token::MathMultiply, Pos::new(1, 15)),
            PosToken::new(Token::NumberDecimal("3".to_string()), Pos::new(3, 9)),
            PosToken::new(Token::MathAdd, Pos::new(3, 11)),
            PosToken::new(Token::NumberDecimal("10".to_string()), Pos::new(3, 13)),
            PosToken::new(Token::MathMultiply, Pos::new(3, 16)),
            PosToken::new(Token::NumberDecimal("70".to_string()), Pos::new(3, 17)),
        ],
        lex
    );

    let ast = parse(lex).unwrap();
    println!(":::POSAST:::\n{ast}");
    // TODO: Macro or something to test parser...
}
