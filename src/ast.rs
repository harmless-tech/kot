use crate::{Ident, token::Token};

pub type NodeAst = Box<Ast>;
pub type BlockAst = Vec<PosAst>;

#[derive(Debug, PartialEq)]
pub struct PosAst {
    pub ast: Ast,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub enum Ast {
    /// Always at the top of an Ast.
    Root(BlockAst),
    Scope(BlockAst), // TODO: Environment!

    UnaryOp {
        op: UnaryOp,
        eval: NodeAst,
    },
    BinaryOp {
        op: BinaryOp,
        eval1: NodeAst,
        eval2: NodeAst,
    },

    Assignment {
        id: Ident,
        eval: NodeAst,
    },

    // Value(Typing),
    ConstOptional(NodeAst),
    ConstNone,
    ConstU8(u8),
    ConstU16(u16),
    ConstU32(u32),
    ConstU64(u64),
    ConstUSize(usize),
    ConstU128(u128),
    ConstI8(i8),
    ConstI16(i16),
    ConstI32(i32),
    ConstI64(i64),
    ConstISize(isize),
    ConstI128(i128),
    ConstF32(f32),
    ConstF64(f64),
    ConstBool(bool),
    ConstChar(char),
    ConstString(String), // TODO: Combine all string with idents using binop.
}

#[derive(Debug, Eq, PartialEq)]
pub enum UnaryOp {}

#[derive(Debug, Eq, PartialEq)]
pub enum BinaryOp {}

// TODO: This might be not good. Only use for primitive types maybe!
#[derive(Debug, Eq, PartialEq)]
pub enum Typing {
    Optional(Box<Typing>),
    None,
    Complex(Ident), // Non primitive type
    U8,
    U16,
    U32,
    U64,
    USize,
    U128,
    I8,
    I16,
    I32,
    I64,
    ISize,
    I128,
    F32,
    F64,
    Bool,
    Char,
    String,
}
impl Typing {
    pub fn from_ast(ast: &Ast) -> Option<Typing> {
        match ast {
            Ast::Root(_) => None,
            Ast::Scope(_) => None,
            Ast::UnaryOp { .. } => None,
            Ast::BinaryOp { .. } => None,
            Ast::Assignment { .. } => None,

            // TODO: ???
            Ast::ConstOptional(t) => Some(Typing::Optional(Typing::from_ast(t).unwrap().into())),
            Ast::ConstNone => Some(Typing::None),
            Ast::ConstU8(_) => Some(Typing::U8),
            Ast::ConstU16(_) => Some(Typing::U16),
            Ast::ConstU32(_) => Some(Typing::U32),
            Ast::ConstU64(_) => Some(Typing::U64),
            Ast::ConstUSize(_) => Some(Typing::USize),
            Ast::ConstU128(_) => Some(Typing::U128),
            Ast::ConstI8(_) => Some(Typing::I8),
            Ast::ConstI16(_) => Some(Typing::I16),
            Ast::ConstI32(_) => Some(Typing::I32),
            Ast::ConstI64(_) => Some(Typing::I64),
            Ast::ConstISize(_) => Some(Typing::ISize),
            Ast::ConstI128(_) => Some(Typing::I128),
            Ast::ConstF32(_) => Some(Typing::F32),
            Ast::ConstF64(_) => Some(Typing::F64),
            Ast::ConstBool(_) => Some(Typing::Bool),
            Ast::ConstChar(_) => Some(Typing::Char),
            Ast::ConstString(_, _) => Some(Typing::String),
        }
    }
}
