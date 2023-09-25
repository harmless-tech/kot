use crate::Int;
use rustc_hash::FxHashMap;

// TODO: Expand and fix this while doing the parser.

pub type Ident = String;
pub type RAst = Box<Ast>;

pub enum Ast {
    /// Command or Ident
    RunCommand(Types),

    Id(Ident),
    DotEx(DotExTypes), // TODO
    If(Box<Ast>, Box<Ast>, Option<Box<Ast>>),
    IfLet(Ident, Box<Ast>, Box<Ast>, Option<Box<Ast>>),
    Let(Ident, Box<Ast>),
}

pub enum Types {
    Ident(String),
    String(String),
    Command(String),
    Integer(Int),
    Boolean(bool),
    Regex(), // TODO
    Object(FxHashMap<String, Types>),
}

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
