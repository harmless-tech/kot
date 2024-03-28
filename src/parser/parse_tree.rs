use crate::{
    data::{Ast, BinaryOperation, PosAst, PosToken, Token, UnaryOperation},
    parser::{parse_item::parse_number, Parser},
};

// TODO: FIX!!!
pub(super) fn p_expression(parser: &mut Parser) -> anyhow::Result<PosAst> {
    p_boolor(parser)
}

// Template
// fn p_(parser: &mut Parser) -> anyhow::Result<PosAst> {
//     let expr = (parser)?;
//     match parser.peek() {
//         Some(PosToken { token: Token::, pos}) => crate::bin_op!(, pos, expr, , parser),
//         _ => Ok(expr),
//     }
// }

macro_rules! bin_op {
    ($t:ident, $p:ident, $e:ident, $f:ident, $par:ident) => {{
        let pos = *$p;
        $par.skip();
        let other_expr = $f($par)?;
        Ok(PosAst::new(
            Ast::BinOp(BinaryOperation::$t, $e.into(), other_expr.into()),
            pos,
        ))
    }};
}

macro_rules! unary_op {
    ($t:ident, $p:ident, $f:ident, $par:ident) => {{
        let pos = *$p;
        $par.skip();
        let other_expr = $f($par)?;
        Ok(PosAst::new(
            Ast::UnaryOp(UnaryOperation::$t, other_expr.into()),
            pos,
        ))
    }};
}

// TODO: Let/Const/Var
// TODO: Closures
// TODO: Assignments
// TODO: Ranges

fn p_boolor(parser: &mut Parser) -> anyhow::Result<PosAst> {
    let expr = p_boolxor(parser)?;
    match parser.peek() {
        Some(PosToken {
            token: Token::BoolOr,
            pos,
        }) => bin_op!(BooleanOr, pos, expr, p_boolor, parser),
        _ => Ok(expr),
    }
}

fn p_boolxor(parser: &mut Parser) -> anyhow::Result<PosAst> {
    let expr = p_booland(parser)?;
    match parser.peek() {
        Some(PosToken {
            token: Token::BoolXor,
            pos,
        }) => bin_op!(BooleanXor, pos, expr, p_boolxor, parser),
        _ => Ok(expr),
    }
}

fn p_booland(parser: &mut Parser) -> anyhow::Result<PosAst> {
    let expr = p_compare(parser)?;
    match parser.peek() {
        Some(PosToken {
            token: Token::BoolAnd,
            pos,
        }) => bin_op!(BooleanAnd, pos, expr, p_booland, parser),
        _ => Ok(expr),
    }
}

fn p_compare(parser: &mut Parser) -> anyhow::Result<PosAst> {
    let expr = p_bitor(parser)?;
    match parser.peek() {
        Some(PosToken {
            token: Token::CompareEqual,
            pos,
        }) => bin_op!(Equal, pos, expr, p_compare, parser),
        Some(PosToken {
            token: Token::CompareNotEqual,
            pos,
        }) => bin_op!(NotEqual, pos, expr, p_compare, parser),
        Some(PosToken {
            token: Token::CompareLess,
            pos,
        }) => bin_op!(Less, pos, expr, p_compare, parser),
        Some(PosToken {
            token: Token::CompareGreater,
            pos,
        }) => bin_op!(Greater, pos, expr, p_compare, parser),
        Some(PosToken {
            token: Token::CompareLessEqual,
            pos,
        }) => bin_op!(LessEqual, pos, expr, p_compare, parser),
        Some(PosToken {
            token: Token::CompareGreaterEqual,
            pos,
        }) => bin_op!(GreaterEqual, pos, expr, p_compare, parser),
        _ => Ok(expr),
    }
}

fn p_bitor(parser: &mut Parser) -> anyhow::Result<PosAst> {
    let expr = p_bitxor(parser)?;
    match parser.peek() {
        Some(PosToken {
            token: Token::BitOr,
            pos,
        }) => bin_op!(BitwiseOr, pos, expr, p_bitor, parser),
        _ => Ok(expr),
    }
}

fn p_bitxor(parser: &mut Parser) -> anyhow::Result<PosAst> {
    let expr = p_bitand(parser)?;
    match parser.peek() {
        Some(PosToken {
            token: Token::BitXor,
            pos,
        }) => bin_op!(BitwiseXor, pos, expr, p_bitxor, parser),
        _ => Ok(expr),
    }
}

fn p_bitand(parser: &mut Parser) -> anyhow::Result<PosAst> {
    let expr = p_bitshift(parser)?;
    match parser.peek() {
        Some(PosToken {
            token: Token::BitAnd,
            pos,
        }) => bin_op!(BitwiseAnd, pos, expr, p_bitand, parser),
        _ => Ok(expr),
    }
}

fn p_bitshift(parser: &mut Parser) -> anyhow::Result<PosAst> {
    let expr = p_additive(parser)?;
    match parser.peek() {
        Some(PosToken {
            token: Token::BitLeft,
            pos,
        }) => bin_op!(BitwiseShiftLeft, pos, expr, p_bitshift, parser),
        Some(PosToken {
            token: Token::BitRight,
            pos,
        }) => bin_op!(BitwiseShiftRight, pos, expr, p_bitshift, parser),
        _ => Ok(expr),
    }
}

fn p_additive(parser: &mut Parser) -> anyhow::Result<PosAst> {
    let expr = p_multiplicative(parser)?;
    match parser.peek() {
        Some(PosToken {
            token: Token::MathAdd,
            pos,
        }) => bin_op!(Add, pos, expr, p_additive, parser),
        Some(PosToken {
            token: Token::MathSubtract,
            pos,
        }) => bin_op!(Subtract, pos, expr, p_additive, parser),
        _ => Ok(expr),
    }
}

fn p_multiplicative(parser: &mut Parser) -> anyhow::Result<PosAst> {
    let expr = p_unary(parser)?;
    match parser.peek() {
        Some(PosToken {
            token: Token::MathMultiply,
            pos,
        }) => bin_op!(Multiply, pos, expr, p_multiplicative, parser),
        Some(PosToken {
            token: Token::MathDivide,
            pos,
        }) => bin_op!(Divide, pos, expr, p_multiplicative, parser),
        Some(PosToken {
            token: Token::MathModulus,
            pos,
        }) => bin_op!(Modulus, pos, expr, p_multiplicative, parser),
        _ => Ok(expr),
    }
}

fn p_unary(parser: &mut Parser) -> anyhow::Result<PosAst> {
    match parser.peek() {
        Some(PosToken {
            token: Token::BitNot,
            pos,
        }) => unary_op!(BooleanNot, pos, p_unary, parser),
        Some(PosToken {
            token: Token::BoolNot,
            pos,
        }) => unary_op!(BitwiseNot, pos, p_unary, parser),
        Some(PosToken {
            token: Token::MathSubtract,
            pos,
        }) => unary_op!(Negate, pos, p_unary, parser),
        _ => p_primary(parser),
    }
}

fn p_primary(parser: &mut Parser) -> anyhow::Result<PosAst> {
    match parser.peek() {
        Some(PosToken {
            token: Token::Ident(id),
            ..
        }) => todo!(),
        Some(PosToken {
            token: Token::LParentheses,
            ..
        }) => {
            parser.skip();
            let expr = p_expression(parser)?;
            match parser.peek() {
                Some(PosToken {
                    token: Token::RParentheses,
                    ..
                }) => {
                    parser.skip();
                    Ok(expr)
                }
                item => panic!("Invalid token: {item:?}"),
            }
        }
        Some(PosToken { token, pos }) if token.is_number() => {
            let wrapped = parse_number(token)?;
            let ret = PosAst::new(Ast::Value(wrapped), *pos);
            parser.skip();
            Ok(ret)
        }
        item => panic!("Bad data!!! {item:?}"),
    }
}
