use crate::Int;
use rustc_hash::FxHashMap;

// TODO: Expand and fix this while doing the parser.

type Ident = String;

pub enum Ast {
    Id(Ident),
    DotEx(DotExTypes), // TODO
    If(Box<Ast>, Box<Ast>, Option<Box<Ast>>),
    IfLet(Ident, Box<Ast>, Box<Ast>, Option<Box<Ast>>),
    Let(Ident, Box<Ast>),
}

pub enum Types {
    String(String),
    Command(String),
    Integer(Int),
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
