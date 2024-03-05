use crate::Pos;

// TODO: String with size limited to u16?
pub type Ident = String;

type Bst = Box<PosAst>;
type Vst = Vec<PosAst>;

#[derive(Debug)]
pub struct PosAst {
    pub ast: Ast,
    pub pos: Pos,
}
impl PosAst {
    #[must_use]
    pub const fn new(ast: Ast, pos: Pos) -> Self {
        Self { ast, pos }
    }
}
impl std::fmt::Display for PosAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ast)
    }
}

#[derive(Debug)]
pub enum Ast {
    // TODO: Should use Vst and Maybe should just be a block.
    Root(Bst),
    Block(Vst),

    UnaryOp(UnaryOperation, Bst),
    BinOp(BinaryOperation, Bst, Bst),

    // TODO
    Assignment(Ident, Bst),
    // TODO: Should this just be a Op and a Assignment, probably.
    #[deprecated]
    AssignmentOp(Ident, AssignmentOperation, Bst),

    Value(Typing),
}
impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root(a) => write!(f, "FakeGlobal... {a}"),
            Self::UnaryOp(op, a) => write!(f, "{op:?} {{ {a} }}"),
            Self::BinOp(op, a1, a2) => write!(f, "{op:?} {{ {a1}, {a2} }}"),
            Self::Value(val) => write!(f, "{val:?}"),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub enum UnaryOperation {
    Negate,
    BooleanNot,
    BitwiseNot,
}

#[derive(Debug)]
pub enum BinaryOperation {
    Multiply,
    Divide,
    Modulus,
    Add,
    Subtract,

    BooleanAnd,
    BooleanXor,
    BooleanOr,

    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    BitwiseShiftLeft,
    BitwiseShiftRight,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
}

#[derive(Debug)]
#[deprecated]
pub enum AssignmentOperation {}

#[derive(Debug)]
pub enum RawTyping {}

#[derive(Clone, Debug)]
pub enum Typing {
    Int64(i64),
    UInt64(u64),
    Float64(f64),

    UInt8(u8),
    Boolean(bool),

    Character(char),
    // TODO: Missing Filler (Should be moved to struct?)
    String(Box<ChoppedString>),

    Ident(Ident),
    // Closure
}

#[derive(Clone, Debug)]
pub struct ChoppedString {
    string: String,
    /// [(Ident, Place position)] sorted in reverse order.
    fill: Vec<(Ident, usize)>,
}
