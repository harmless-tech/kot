use lazy_regex::{regex, regex_captures};

use crate::info::Type;

// Token, Index, Line Number
// Derive column number use str.lines(), then count through index
pub type LToken = (Token, usize, usize);
type TokenAdd = (Token, usize, usize);
type TokenError = (String, usize, usize);

#[derive(Debug)]
pub enum Token {
    // Custom
    Command(String), // #
    Ident(String),        // anything
    RawNumber(String),      // 000
    RawChar(char),           // 'a'
    RawString(String),       // "", #""#

    // Misc
    Illegal,
    Eof,    //TODO: Needed?
    Assign, // =

    // Operators
    Plus,           // +
    Subtract,       // -
    Multiplication, // *
    Division,       // /
    Remainder,      // %
    Equal,          // ==
    NotEqual,       // !=
    Bang,           // !
    LeftShift,      // <<
    RightShift,     // >>
    GreaterEqual,   // >=
    LessEqual,      // <=
    Greater,        // >
    Less,           // <
    And,            // &&
    Or,             // ||
    BitAnd,         // &
    BitOr,          // |
    BitExOr,        // ^

    // Structure
    Comma,             // ,
    Colon,             // :
    SemiColon,         // ;
    LeftParen,         // (
    RightParen,        // )
    LeftBracket,       // [
    RightBracket,      // ]
    LeftCurlyBracket,  // {
    RightCurlyBracket, // }

    // Keywords
    Use,        // use
    As,         // as
    Const,      // const
    Let,        // let
    Val,        // val
    TypeDef, // typedef
    Enum,       // enum
    Def,        // def
    Function,   // fn
    Override, // override
    For,        // for
    In,         // in
    Where,      // where
    DollarSign, // $
    Break,      // break
    If,         // if
    Else,       // else
    Match,      // match
    Return,     // return
    Test, // test
    Module, // mod
    Type,       // type
    Export, // export

    // Types are done in idents
//    // Types
//
//    // Void, // () // gets captured by ( and )
//    Boolean, // bool
//    // Textual
//    Char, // char
//    String, // str
//    // Ints
//    U8, //u8
//    U16, //u16
//    U32, //u32
//    U64, //u64
//    U128, //u128
//    USize, // usize
//    I8, //i8
//    I16, //i16
//    I32, //i32
//    I64, //i64
//    I128, //i128
//    ISize, // isize
//    F32, // f32
//    F64, // f64
}

#[rustfmt::skip]
pub fn lexer(input: &str) -> Result<Vec<LToken>, String> {
    let mut tokens: Vec<LToken> = Vec::new();
    let mut index: usize = 0;
    let mut line_num: usize = 0;

    while index < input.len() {
        let s = &input[index..];
        
        // Build Hints (Command)
        if regex!(r"^#").is_match(s) {
            match lex_command(&input[(index + 1)..]) {
                Ok((token, i_add, _)) => {
                    tokens.push((token, index, line_num));
                    index += i_add + 1
                }
                Err(err) => todo!(),
            }
        }

        // Comments
        else if regex!(r"^//").is_match(s) { todo!(); }
        else if regex!(r"^/\*").is_match(s) { todo!(); }

        // Operators
        if regex!(r"^\+").is_match(s) { tokens.push((Token::Plus, index, line_num)); index += 1; }
        else if regex!(r"^-").is_match(s) { tokens.push((Token::Subtract, index, line_num)); index += 1; }
        else if regex!(r"^\*").is_match(s) { tokens.push((Token::Multiplication, index, line_num)); index += 1; }
        else if regex!(r"^/").is_match(s) { tokens.push((Token::Division, index, line_num)); index += 1; }
        else if regex!(r"^%").is_match(s) { tokens.push((Token::Remainder, index, line_num)); index += 1; }
        else if regex!(r"^==").is_match(s) { tokens.push((Token::Equal, index, line_num)); index += 2; }
        else if regex!(r"^!=").is_match(s) { tokens.push((Token::NotEqual, index, line_num)); index += 2; }
        else if regex!(r"^!").is_match(s) { tokens.push((Token::Bang, index, line_num)); index += 1; }
        else if regex!(r"^<<").is_match(s) { tokens.push((Token::LeftShift, index, line_num)); index += 2; }
        else if regex!(r"^>>").is_match(s) { tokens.push((Token::RightShift, index, line_num)); index += 2; }
        else if regex!(r"^>=").is_match(s) { tokens.push((Token::GreaterEqual, index, line_num)); index += 2; }
        else if regex!(r"^<=").is_match(s) { tokens.push((Token::LessEqual, index, line_num)); index += 2; }
        else if regex!(r"^>").is_match(s) { tokens.push((Token::Greater, index, line_num)); index += 1; }
        else if regex!(r"^<").is_match(s) { tokens.push((Token::Less, index, line_num)); index += 1; }
        else if regex!(r"^&&").is_match(s) { tokens.push((Token::And, index, line_num)); index += 2; }
        else if regex!(r"^\|\|").is_match(s) { tokens.push((Token::Or, index, line_num)); index += 2; }
        else if regex!(r"^&").is_match(s) { tokens.push((Token::BitAnd, index, line_num)); index += 1; }
        else if regex!(r"^\|").is_match(s) { tokens.push((Token::BitOr, index, line_num)); index += 1; }
        else if regex!(r"^\^").is_match(s) { tokens.push((Token::BitExOr, index, line_num)); index += 1; }
        
        // Misc
        else if regex!(r"^=").is_match(s) { tokens.push((Token::Assign, index, line_num)); index += 1; }
        
        // Structure
        else if regex!(r"^,").is_match(s) { tokens.push((Token::Comma, index, line_num)); index += 1; }
        else if regex!(r"^:").is_match(s) { tokens.push((Token::Colon, index, line_num)); index += 1; }
        else if regex!(r"^;").is_match(s) { tokens.push((Token::SemiColon, index, line_num)); index += 1; }
        else if regex!(r"^\(").is_match(s) { tokens.push((Token::LeftParen, index, line_num)); index += 1; }
        else if regex!(r"^\)").is_match(s) { tokens.push((Token::RightParen, index, line_num)); index += 1; }
        else if regex!(r"^\[").is_match(s) { tokens.push((Token::LeftBracket, index, line_num)); index += 1; }
        else if regex!(r"^\]").is_match(s) { tokens.push((Token::RightBracket, index, line_num)); index += 1; }
        else if regex!(r"^\{").is_match(s) { tokens.push((Token::LeftCurlyBracket, index, line_num)); index += 1; }
        else if regex!(r"^\}").is_match(s) { tokens.push((Token::RightCurlyBracket, index, line_num)); index += 1; }
        
        // Keywords
        else if regex!(r"^use(\W|$)").is_match(s) { tokens.push((Token::Use, index, line_num)); index += 3; }
        else if regex!(r"^as(\W|$)").is_match(s) { tokens.push((Token::As, index, line_num)); index += 2; }
        else if regex!(r"^const(\W|$)").is_match(s) { tokens.push((Token::Const, index, line_num)); index += 5; }
        else if regex!(r"^let(\W|$)").is_match(s) { tokens.push((Token::Let, index, line_num)); index += 3; }
        else if regex!(r"^val(\W|$)").is_match(s) { tokens.push((Token::Val, index, line_num)); index += 3; }
        else if regex!(r"^typedef(\W|$)").is_match(s) { tokens.push((Token::TypeDef, index, line_num)); index += 7; }
        else if regex!(r"^enum(\W|$)").is_match(s) { tokens.push((Token::Enum, index, line_num)); index += 4; }
        else if regex!(r"^def(\W|$)").is_match(s) { tokens.push((Token::Def, index, line_num)); index += 3; }
        else if regex!(r"^fn(\W|$)").is_match(s) { tokens.push((Token::Function, index, line_num)); index += 2; }
        else if regex!(r"^override(\W|$)").is_match(s) { tokens.push((Token::Override, index, line_num)); index += 8; }
        else if regex!(r"^for(\W|$)").is_match(s) { tokens.push((Token::For, index, line_num)); index += 3; }
        else if regex!(r"^in(\W|$)").is_match(s) { tokens.push((Token::In, index, line_num)); index += 2; }
        else if regex!(r"^where(\W|$)").is_match(s) { tokens.push((Token::Where, index, line_num)); index += 5; }
        else if regex!(r"^\$").is_match(s) { tokens.push((Token::DollarSign, index, line_num)); index += 1; }
        else if regex!(r"^break(\W|$)").is_match(s) { tokens.push((Token::Break, index, line_num)); index += 5; }
        else if regex!(r"^if(\W|$)").is_match(s) { tokens.push((Token::If, index, line_num)); index += 2; }
        else if regex!(r"^else(\W|$)").is_match(s) { tokens.push((Token::Else, index, line_num)); index += 4; }
        else if regex!(r"^match(\W|$)").is_match(s) { tokens.push((Token::Match, index, line_num)); index += 5; }
        else if regex!(r"^return(\W|$)").is_match(s) { tokens.push((Token::Return, index, line_num)); index += 6; }
        else if regex!(r"^test(\W|$)").is_match(s) { tokens.push((Token::Test, index, line_num)); index += 4; }
        else if regex!(r"^mod(\W|$)").is_match(s) { tokens.push((Token::Module, index, line_num)); index += 3; }
        else if regex!(r"^type(\W|$)").is_match(s) { tokens.push((Token::Type, index, line_num)); index += 4; }
        else if regex!(r"^export(\W|$)").is_match(s) { tokens.push((Token::Export, index, line_num)); index += 6; }

        // Custom
        // Int
        else if let Some((_, cap, _, _, _, _)) =
            regex_captures!(r"^((\d+_?)+\.?(\d+_?)+([uifsize]+\d{0,3})?)(;|\s|$)", s) {
            tokens.push((Token::RawNumber(cap.to_string()), index, line_num));
            index += cap.len();
        }
        // Char
        else if let Some((_, cap)) = regex_captures!(r"^'(\\t|\\n|\\r|.)'", s) {
            let chr = match cap {
                "\\t" => '\t',
                "\\n" => '\n',
                "\\r" => '\r',
                _ => cap.chars().next().unwrap()
            };
            tokens.push((Token::RawChar(chr), index, line_num));
            index += cap.len() + 2; // Length of char plus ''
        }
        // String
        else if let Some((_, cap)) = regex_captures!(r#"^(#*)""#, s) {
            // cap: How many ### there are
        }
        
        // TODO: Detect String
        // TODO: Ident
        
        // --- Iter ---
        else if regex!(r"^\n").is_match(s) { line_num += 1; index += 1; }
        else if regex!(r"^\s").is_match(s) { index += 1; }
        else { panic!("Illegal!"); } // TODO
    }

    todo!();
}

fn lex_command(slice: &str) -> Result<TokenAdd, TokenError> {
    let mut found = false;
    let mut index = 0_usize;

    while index < slice.len() {
        let s = &slice[index..];
        index += 1;

        if regex!(r"^;").is_match(s) {
            found = true;
            break;
        }
        else if regex!(r"^\n").is_match(s) {
            break;
        }
    }

    if !found {
        return Err(("Command did not end with ';'.".to_string(), index, 0));
    }

    Ok((Token::Command(String::from(&slice[0..(index - 1)])), index, 0))
}

fn lex_string(slice: &str, hash_amount: usize) -> Result<(LToken, usize, usize), String> {
    let mut cap_string = String::new();
    // Find end

    // Handle special chars
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_command() {
        lex_command("set koi = true;").unwrap();
        lex_command("set koi = true").unwrap_err();
    }
}
