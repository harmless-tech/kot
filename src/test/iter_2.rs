use crate::{
    data::{Ast, BinaryOperation, PosAst, PosToken, Token, Typing},
    lexer::lex,
    parser::parse,
    Pos,
};
use std::{fs::read_to_string, ops::Deref};

#[test]
fn kot_1() {
    let str = read_to_string("./test/iter_2/1.kot").unwrap();
    let lex = lex(&str).unwrap();

    assert_lexer!(
        [
            PosToken::new(Token::Let, Pos::new(1, 1)),
            PosToken::new(Token::Ident("item".to_string()), Pos::new(1, 5)),
            PosToken::new(Token::Colon, Pos::new(1, 9)),
            PosToken::new(Token::Ident("int".to_string()), Pos::new(1, 11)),
            PosToken::new(Token::Assign, Pos::new(1, 15)),
            PosToken::new(Token::NumberDecimal("1".to_string()), Pos::new(1, 17)),
            PosToken::new(Token::MathAdd, Pos::new(1, 19)),
            PosToken::new(Token::NumberDecimal("222".to_string()), Pos::new(1, 21)),
            PosToken::new(Token::MathMultiply, Pos::new(1, 25)),
            PosToken::new(Token::NumberDecimal("3".to_string()), Pos::new(1, 27)),
            PosToken::new(Token::MathDivide, Pos::new(1, 29)),
            PosToken::new(Token::NumberDecimal("7".to_string()), Pos::new(1, 30)),
            PosToken::new(Token::MathAdd, Pos::new(1, 32)),
            PosToken::new(Token::NumberDecimal("1".to_string()), Pos::new(1, 33)),
        ],
        lex
    );

    // let ast = parse(lex).unwrap();
    // if let PosAst {
    //     ast: Ast::Root(a),
    //     pos,
    // } = ast
    // {
    //     assert_eq!(pos, Pos::new(0, 0));
    //
    //     if let PosAst {
    //         ast: Ast::BinOp(BinaryOperation::Add, a1, a2),
    //         pos,
    //     } = *a
    //     {
    //         assert_eq!(pos, Pos::new(1, 3));
    //
    //         if let PosAst {
    //             ast: Ast::Value(Typing::Int64(int)),
    //             pos,
    //         } = *a1
    //         {
    //             assert_eq!(pos, Pos::new(1, 1));
    //             assert_eq!(int, 1);
    //         }
    //         else {
    //             panic!()
    //         }
    //
    //         if let PosAst {
    //             ast: Ast::Value(Typing::Int64(int)),
    //             pos,
    //         } = *a2
    //         {
    //             assert_eq!(pos, Pos::new(1, 5));
    //             assert_eq!(int, 2);
    //         }
    //         else {
    //             panic!()
    //         }
    //     }
    //     else {
    //         panic!()
    //     }
    // }
    // else {
    //     panic!();
    // }
}
