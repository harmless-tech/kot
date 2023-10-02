use crate::Int;
use rustc_hash::FxHashMap;

// TODO: Expand and fix this while doing the parser.

pub type Ident = String;
pub type RAst = Box<Ast>;

#[derive(Debug)]
pub enum Ast {
    /// Ast and Some(Next Ast)
    Block(Vec<Ast>),
    /// Ast
    Scope(RAst),
    /// Ident or Command
    RunCommand(Types),
    /// Ident or Integer
    Exit(Types),
    /// Ident or String
    Panic(Types),

    Id(Ident),
    DotEx(DotExTypes), // TODO
    If(Box<Ast>, Box<Ast>, Option<Box<Ast>>),
    IfLet(Ident, Box<Ast>, Box<Ast>, Option<Box<Ast>>),
    Let(Ident, Box<Ast>),
}

#[derive(Debug)]
pub enum Types {
    Ident(String),
    String(String), // TODO: Allow templating!
    Command(String),
    Integer(Int),
    Boolean(bool),
    Regex(), // TODO
    Object(FxHashMap<String, Types>),
}

#[derive(Debug)]
pub enum DotExTypes {
    Triplet,
    OS,
    Family,
    Arch,
    Regex,
    Args,
    Cmd,
    Parallel,
}
