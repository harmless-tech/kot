use std::borrow::Cow;

use crate::{Float, Int};

pub type Ident = String;
pub type Idents = Vec<String>;
pub type IdentFill = Vec<(usize, Ident)>;

type BAst = Box<Ast>;
type VAst = Vec<Ast>;

#[derive(Debug)]
pub enum Ast {
    /// The start of the program. (Global scope)
    Program(VAst),
    /// {} (Local Scope)
    Block(VAst),
    
    // TODO
    While(),
    For(),
    
    Function { vars: Idents, captures: Idents, env: BAst },
    FunctionCall(Ident, VAst),
    
    // TODO TYPES
    TypeIdent(Ident), // COW type
    TypeBool(Box<Types>),
    TypeInt(Box<Types>),
    TypeFloat(Box<Types>),
    TypeChar(Box<Types>),
    // Conver this to a complete string and put into type
    TypeString(String, IdentFill),
    TypeArray(VAst),
    TypeSet(Box<Types>),
    TypeMap(Box<Types>),
    TypeRange(Box<Types>),
    TypeRegex(Box<Types>),
}

// TODO: Needs clone on write stuff.
#[derive(Debug)]
pub enum Types {
    // This is useful for closures which can be big and cannot be written to.
    // Also can point to other stuff and when that item is written to, then clone.
    // TODO: Punch down to lowest ident and then point to it...
    // TODO: Or just copy ident if its type is ident...
    Ident(Ident),
    
    // Copy
    Bool(bool),
    // Copy
    Int(Int),
    // Copy
    Float(Float),
    // Copy
    Char(char),
    // COW
    String(String),
    // COW
    Array(Vec<Types>),
    // COW
    Set(),
    // COW
    Map(),
    // COW
    Range(),
    // COW
    Regex(),
    
    // Never Clone or Copy
    Closure(Ast), // TODO: environment
}
