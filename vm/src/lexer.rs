use lazy_regex::{regex, regex_captures};

use crate::info::Type;

// Token, Index, Line Number
// Derive column number use str.lines(), then count through index
type LToken = (Token, usize, usize);

#[derive(Debug)]
enum Token {
    // Custom
    Ident(String),        // anything
    OutsideIdent(String), // anything::anything
    Number(String),      // 000
    Char(char),           // 'a'
    String(String),       // "", #""#

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
    Type,       // type
    Enum,       // enum
    Def,        // def
    Function,   // fn
    For,        // for
    In,         // in
    Where,      // where
    DollarSign, // $
    Break,      // break
    If,         // if
    Else,       // else
    Match,      // match
    Return,     // return

    // Types
    TypeInfo(Type),
}

#[rustfmt::skip]
pub fn lexer(input: &str) {
    let mut tokens: Vec<LToken> = Vec::new();
    let mut index: usize = 0;

    let mut line_num: usize = 0;
    let mut column_num: usize = 0;

    while index < input.len() {
        let s = &input[index..];
        
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
        else if regex!(r"^use(\s|$)").is_match(s) { tokens.push((Token::Use, index, line_num)); index += 3; }
        else if regex!(r"^as(\s|$)").is_match(s) { tokens.push((Token::As, index, line_num)); index += 2; }
        else if regex!(r"^const(\s|$)").is_match(s) { tokens.push((Token::Const, index, line_num)); index += 5; }
        else if regex!(r"^let(\s|$)").is_match(s) { tokens.push((Token::Let, index, line_num)); index += 3; }
        else if regex!(r"^val(\s|$)").is_match(s) { tokens.push((Token::Val, index, line_num)); index += 3; }
        else if regex!(r"^type(\s|$)").is_match(s) { tokens.push((Token::Type, index, line_num)); index += 4; }
        else if regex!(r"^enum(\s|$)").is_match(s) { tokens.push((Token::Enum, index, line_num)); index += 4; }
        else if regex!(r"^def(\s|$)").is_match(s) { tokens.push((Token::Def, index, line_num)); index += 3; }
        else if regex!(r"^fn(\s|$)").is_match(s) { tokens.push((Token::Function, index, line_num)); index += 2; }
        else if regex!(r"^for(\s|$)").is_match(s) { tokens.push((Token::For, index, line_num)); index += 3; }
        else if regex!(r"^in(\s|$)").is_match(s) { tokens.push((Token::In, index, line_num)); index += 2; }
        else if regex!(r"^where(\s|$)").is_match(s) { tokens.push((Token::Where, index, line_num)); index += 5; }
        else if regex!(r"^\$").is_match(s) { tokens.push((Token::DollarSign, index, line_num)); index += 1; }
        else if regex!(r"^break(;|\s|$)").is_match(s) { tokens.push((Token::Break, index, line_num)); index += 5; }
        else if regex!(r"^if(\s|$)").is_match(s) { tokens.push((Token::If, index, line_num)); index += 2; }
        else if regex!(r"^else(\s|$)").is_match(s) { tokens.push((Token::Else, index, line_num)); index += 4; }
        else if regex!(r"^match(\s|$)").is_match(s) { tokens.push((Token::Match, index, line_num)); index += 5; }
        else if regex!(r"^return(;|\s|$)").is_match(s) { tokens.push((Token::Return, index, line_num)); index += 6; }
        
        // Types
        // TODO: !!!
        // Can be placed in front of anything except letters
        
        // Custom
        // Int
        else if let Some((cap, _, _, _, _, _)) =
            regex_captures!(r"^((\d+_?)+\.?(\d+_?)+([uifsize]+\d{0,3})?)(;|\s|$)", s) {
            tokens.push((Token::Number(cap.to_string()), index, line_num));
            index += cap.len();
        }
        // Char
        
        
        // TODO: Detect Char
        // TODO: Detect String
        // TODO: OutsideIdent
        // TODO: Ident
        
        // --- Iter ---
        else if regex!(r"^\n").is_match(s) { line_num += 1; index += 1; }
        else if regex!(r"^\s").is_match(s) { index += 1; }
        else { panic!("Illegal!"); } // TODO
    }
}
