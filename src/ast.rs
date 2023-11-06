use crate::Int;

// TODO: Expand and fix this while doing the parser.

pub type Ident = String;
pub type IdentFill = Vec<(usize, Ident)>;

#[derive(Debug)]
pub enum Ast {
    /// Vec<Ast>
    Block(Vec<Ast>),
    /// Ast
    Scope(Box<Ast>),
    /// Ident and Return
    Assign(Ident, Box<Ast>),

    /// Ident, String, or Command
    // RunCommand(Box<Ast>),
    /// Ident, String, or Command
    SpawnCommand(Box<Ast>),

    /// Ident or Integer
    Exit(Box<Ast>),
    /// Ident or String
    Panic(Box<Ast>),

    /// Triplets and Ast
    Triplets(Vec<String>, Box<Ast>),
    /// Arches and Ast
    Arches(Vec<String>, Box<Ast>),
    /// OSes and Ast
    OSes(Vec<String>, Box<Ast>),
    /// Families and Ast
    Families(Vec<String>, Box<Ast>),

    // BinOp(OP, AST, AST)
    // If(Box<Ast>, Box<Ast>, Option<Box<Ast>>),
    // IfLet(Ident, Box<Ast>, Box<Ast>, Option<Box<Ast>>),
    // Let(Ident, Box<Ast>),
    Type(AstType),
}

#[derive(Debug)]
pub enum AstType {
    Ident(Ident),
    String(String, IdentFill),
    Command(String, IdentFill),
    Integer(Int),
    // Boolean(bool),
    // Regex(), // TODO
    // Object(FxHashMap<String, PType>),
    // VOID: Interp only?
}
impl AstType {
    pub fn ident(ident: String) -> Box<Ast> {
        Box::new(Ast::Type(AstType::Ident(ident)))
    }

    pub fn string(string: String, ident_fill: IdentFill) -> Box<Ast> {
        Box::new(Ast::Type(AstType::String(string, ident_fill)))
    }

    pub fn command(command: String, ident_fill: IdentFill) -> Box<Ast> {
        Box::new(Ast::Type(AstType::Command(command, ident_fill)))
    }

    pub fn integer(int: Int) -> Box<Ast> {
        Box::new(Ast::Type(AstType::Integer(int)))
    }
}
