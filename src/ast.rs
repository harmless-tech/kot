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
    /// Ident or Command
    RunCommand(AstType),
    /// Ident or Command
    SpawnCommand(AstType),
    /// Ident or Integer
    Exit(AstType),
    /// Ident or String
    Panic(AstType),
    // If(Box<Ast>, Box<Ast>, Option<Box<Ast>>),
    // IfLet(Ident, Box<Ast>, Box<Ast>, Option<Box<Ast>>),
    // Let(Ident, Box<Ast>),
}

#[derive(Debug)]
pub enum AstType {
    Ident(String),
    String(String, IdentFill), // TODO: Allow templating!
    Command(String, IdentFill),
    Integer(Int),
    // Boolean(bool),
    // Regex(), // TODO
    // Object(FxHashMap<String, PType>),
}
