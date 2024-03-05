mod binary_ops;
mod unary_ops;

use crate::data::{Ast, BinaryOperation, Environment, PosAst, Typing, UnaryOperation};

// TODO: Change!!!
pub fn run(ast: &PosAst, interp: &mut Interpreter) -> anyhow::Result<Option<Typing>> {
    todo!()
}

#[derive(Debug)]
pub struct Interpreter {
    pub ast: PosAst,
    pub envir: Environment,
}
impl Interpreter {
    pub fn new(ast: PosAst) -> Self {
        Self {
            ast,
            envir: Environment::new(),
        }
    }

    pub fn new_with_environment(ast: PosAst, envir: Environment) -> Self {
        Self { ast, envir }
    }

    // TODO: Runs the entire ast.
    pub fn run(&mut self) -> anyhow::Result<Option<Typing>> {
        Self::run_tree(&self.ast, &mut self.envir)
    }

    fn run_tree(ast: &PosAst, envir: &mut Environment) -> anyhow::Result<Option<Typing>> {
        let PosAst { ast, pos } = ast;
        match ast {
            Ast::Root(ast) => Self::run_tree(ast, envir),
            Ast::UnaryOp(op, ast) => {
                let expr = Self::run_tree(ast, envir)?;
                match (op, expr) {
                    (UnaryOperation::Negate, Some(t)) => match t {
                        Typing::Int64(v) => Ok(Some(Typing::Int64(-v))),
                        _ => panic!(),
                    },
                    (UnaryOperation::BooleanNot, ..) => todo!(),
                    (UnaryOperation::BitwiseNot, ..) => todo!(),
                    (op, ..) => panic!(),
                }
            }
            Ast::BinOp(op, a1, a2) => {
                let expr1 = Self::run_tree(a1, envir)?;
                let expr2 = Self::run_tree(a2, envir)?;
                match (op, expr1, expr2) {
                    (BinaryOperation::Add, Some(v1), Some(v2)) => match (v1, v2) {
                        (Typing::Int64(i1), Typing::Int64(i2)) => Ok(Some(Typing::Int64(i1 + i2))),
                        _ => panic!(),
                    },
                    (op, ..) => panic!(),
                }
            }
            Ast::Value(val) => Ok(Some(val.clone())),
            _ => todo!(),
        }
    }
}
