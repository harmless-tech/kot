use crate::{
    data::{Ast, BinaryOperation, PosAst, PosToken, Token, UnaryOperation},
    parser::{parse_item::parse_number, Parser},
};

// TODO: FIX!!!
pub(super) fn p_expression(parser: &mut Parser) -> anyhow::Result<PosAst> {
    p_additive(parser)
}

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
            token: Token::Number(num, radix),
            pos,
        }) => {
            let wrapped = parse_number(num, *radix)?;
            let ret = PosAst::new(Ast::Value(wrapped), *pos);
            parser.skip();
            Ok(ret)
        }
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
        item => panic!("Bad data!!! {item:?}"),
    }
}
