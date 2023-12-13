// TODO: More testing

use crate::{lexer::lex, token::Token};

#[test]
fn only_single_line_comment() {
    let tokens = lex(r##"// Single line comment life
                                                      // // // // Single
                                                      // Comment!!!       "##)
    .unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token, Token::EOF);
}

#[test]
#[should_panic]
fn panic_multi_line_comment() {
    let _ = lex(r##"/* This comment does not have an ending!!!"##);
}

#[test]
fn get_hex() {
    let tokens = lex(r##"0x1aAfFd90398536_24438f___12DDFffff"##).unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(
        tokens[0].token,
        Token::NumberHex("1aaffd9039853624438f12ddfffff".to_string())
    );
}

#[test]
#[should_panic]
fn panic_get_hex_empty() {
    let _ = lex(r##"0x"##);
}

#[test]
#[should_panic]
fn panic_get_hex_bad_char() {
    let _ = lex(r##"0xgg"##);
}

#[test]
fn get_binary() {
    let tokens = lex(r##"0b1010__10000_01_01_011111___11101"##).unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(
        tokens[0].token,
        Token::NumberBinary("101010000010101111111101".to_string())
    );
}

#[test]
#[should_panic]
fn panic_get_binary_empty() {
    let _ = lex(r##"0b"##);
}

#[test]
#[should_panic]
fn panic_get_binary_bad_char() {
    let _ = lex(r##"0baa"##);
}

#[test]
fn get_numbers() {
    let tokens = lex(r##"100 -110 3 1 -1 -0 -12 234__123 1_322 -1_234567689"##).unwrap();
    assert_eq!(tokens.len(), 11);
    assert_eq!(tokens[0].token, Token::Number("100".to_string()));
    assert_eq!(tokens[1].token, Token::Number("-110".to_string()));
    assert_eq!(tokens[2].token, Token::Number("3".to_string()));
    assert_eq!(tokens[3].token, Token::Number("1".to_string()));
    assert_eq!(tokens[4].token, Token::Number("-1".to_string()));
    assert_eq!(tokens[5].token, Token::Number("-0".to_string()));
    assert_eq!(tokens[6].token, Token::Number("-12".to_string()));
    assert_eq!(tokens[7].token, Token::Number("234123".to_string()));
    assert_eq!(tokens[8].token, Token::Number("1322".to_string()));
    assert_eq!(tokens[9].token, Token::Number("-1234567689".to_string()));
}

#[test]
fn blank() {
    let _ = lex(r##""##);
}
