use crate::tokens::Token;
use lazy_regex::{regex, regex_captures};

fn err_message<T>(file_name: &str, line_num: usize, message: &str) -> Result<T, String> {
    Err(format!(
        "File: {}, Line: {}: {}",
        file_name,
        line_num + 1,
        message
    ))
}

pub fn remove_comments(contents: String) -> String {
    let mut lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

    let mut in_quotes = false;
    let mut quote_size = 0_usize;
    let mut in_block_comment = false;
    let mut initial_block_loc = 0_usize;

    for line in lines.iter_mut() {
        if !line.starts_with("#") {
            let mut index = 0_usize;
            let mut block_start_line = false;

            while index < line.len() {
                let s = &(line.clone())[index..line.len()];

                if in_quotes {
                    if let Some(cap) = regex_captures!("^\"#*", s) {
                        // Exit string.
                        if quote_size == cap.len() {
                            in_quotes = false;
                            index += quote_size;
                        }
                        else {
                            index += cap.len();
                        }
                    }
                    else {
                        index += 1;
                    }
                }
                else if in_block_comment {
                    if regex!("^\\*/").is_match(s) {
                        // Exit block comment.
                        in_block_comment = false;
                        index += 2;

                        if block_start_line {
                            line.drain(initial_block_loc..index);
                        }
                        else {
                            line.drain(0..index);
                        }

                        index = 0;
                    }
                    else {
                        index += 1;
                    }
                }
                else if let Some(cap) = regex_captures!("^#*\"", s) {
                    // Enter string.
                    in_quotes = true;
                    quote_size = cap.len();
                    index += cap.len();
                }
                else if regex!("^/\\*").is_match(s) {
                    // Enter block comment.
                    in_block_comment = true;
                    initial_block_loc = index;
                    block_start_line = true;
                    index += 2;
                }
                else if regex!("^//").is_match(s) {
                    // Remove one line comment.
                    line.drain(index..line.len());
                    // Do not change index.
                }
                else {
                    index += 1;
                }
            }

            if in_block_comment {
                line.drain(0..line.len());
            }
        }
    }

    lines.join("\n")
}

pub fn pre_process(contents: String) -> (String, String) {
    let mut lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

    let mut metadata = Vec::new();
    for line in lines.iter_mut() {
        if line.starts_with("#") && !(line.starts_with("#\"")) {
            metadata.push(line.drain(1..line.len()).collect::<String>());
            line.drain(0..1);
        }
    }

    (lines.join("\n"), metadata.join("\n"))
}

#[rustfmt::skip]
pub fn tokenize(file_name: &str, contents: &String) -> Result<Vec<Token>, String> {
    let mut token_list = Vec::new();
    let mut index = 0_usize;
    let mut line_num = 0_usize;

    while index < contents.len() {
        let s = &contents[index..contents.len()];

        if regex!("^data\\s").is_match(s) { token_list.push(Token::Data); index += 4; } // Data
        else if regex!("^val\\s").is_match(s) { token_list.push(Token::Data); index += 3; } // Data
        else if regex!("^let\\s").is_match(s) { token_list.push(Token::Data); index += 3; } // Data
        else if regex!("^interface\\s").is_match(s) { token_list.push(Token::Interface); index += 9; } // Interface
        else if regex!("^comply\\s").is_match(s) { token_list.push(Token::ComplyWith); index += 6; } // ComplyWith
        else if regex!("^:").is_match(s) { token_list.push(Token::Colon); index += 1; } // Colon
        else if regex!("^=").is_match(s) { token_list.push(Token::Assign); index += 1; } // Assign
        else if regex!("^,").is_match(s) { token_list.push(Token::Comma); index += 1; } // Comma
        else if regex!("^\\?").is_match(s) { token_list.push(Token::QuestionMark); index += 1; } // Question Mark

        else if regex!("^\\(").is_match(s) { token_list.push(Token::LeftParentheses); index += 1; } // Left Parentheses
        else if regex!("^\\)").is_match(s) { token_list.push(Token::RightParentheses); index += 1; } // Right Parentheses
        else if regex!("^\\[").is_match(s) { token_list.push(Token::LeftBracket); index += 1; } // Left Bracket
        else if regex!("^\\]").is_match(s) { token_list.push(Token::RightBracket); index += 1; } // Right Bracket
        else if regex!("^\\{").is_match(s) { token_list.push(Token::LeftCurlyBrace); index += 1; } // Left Curly Brace
        else if regex!("^}").is_match(s) { token_list.push(Token::RightCurlyBrace); index += 1; } // Right Curly Brace

        else if regex!("^char\\W").is_match(s) { token_list.push(Token::TypeChar); index += 4; } // Type Char
        else if regex!("^chr\\W").is_match(s) { token_list.push(Token::TypeChar); index += 3; } // Type Char
        else if regex!("^int\\W").is_match(s) { token_list.push(Token::TypeInt64); index += 3; } // Type Int
        else if regex!("^i64\\W").is_match(s) { token_list.push(Token::TypeInt64); index += 3; } // Type Int
        else if regex!("^uint\\W").is_match(s) { token_list.push(Token::TypeUInt64); index += 4; } // Type UInt
        else if regex!("^u64\\W").is_match(s) { token_list.push(Token::TypeUInt64); index += 3; } // Type UInt
        else if regex!("^float\\W").is_match(s) { token_list.push(Token::TypeFloat64); index += 5; } // Type Float
        else if regex!("^f64\\W").is_match(s) { token_list.push(Token::TypeFloat64); index += 3; } // Type Float
        else if regex!("^byte\\W").is_match(s) { token_list.push(Token::TypeByte); index += 4; } // Type Byte
        else if regex!("^u8\\W").is_match(s) { token_list.push(Token::TypeByte); index += 2; } // Type Byte
        else if regex!("^string\\W").is_match(s) { token_list.push(Token::TypeString); index += 6; } // Type String
        else if regex!("^str\\W").is_match(s) { token_list.push(Token::TypeString); index += 3; } // Type String
        else if regex!("^boolean\\W").is_match(s) { token_list.push(Token::TypeBoolean); index += 7; } // Type Boolean
        else if regex!("^bool\\W").is_match(s) { token_list.push(Token::TypeBoolean); index += 4; } // Type Boolean
        else if regex!("^object\\W").is_match(s) { token_list.push(Token::TypeObject); index += 6; } // Type Object
        else if regex!("^obj\\W").is_match(s) { token_list.push(Token::TypeObject); index += 3; } // Type Object

        else if regex!("^fn\\W").is_match(s) { token_list.push(Token::Function); index += 3; } // Function
        else if regex!("^as\\W").is_match(s) { token_list.push(Token::Cast); index += 2; } // Cast
        else if regex!("^\\.\\.").is_match(s) { token_list.push(Token::Concat); index += 2; } // Concat

        else if regex!("^![^=]").is_match(s) { token_list.push(Token::Negate); index += 1; } // Negate
        else if regex!("^&&").is_match(s) { token_list.push(Token::And); index += 2; } // And
        else if regex!("^\\|\\|").is_match(s) { token_list.push(Token::Or); index += 2; } // Or
        else if regex!("^\\+").is_match(s) { token_list.push(Token::Plus); index += 1; } // Plus
        else if regex!("^-[\\s]+").is_match(s) { token_list.push(Token::Minus); index += 1; } // Minus
        else if regex!("^\\*").is_match(s) { token_list.push(Token::Multiply); index += 1; } // Multiply
        else if regex!("^/").is_match(s) { token_list.push(Token::Divide); index += 1; } // Divide
        else if regex!("^%").is_match(s) { token_list.push(Token::Modulus); index += 1; } // Modulus
        else if regex!("^==").is_match(s) { token_list.push(Token::Equals); index += 2; } // Equals
        else if regex!("^!=").is_match(s) { token_list.push(Token::NotEquals); index += 2; } // Not Equals
        else if regex!("^>[^=]").is_match(s) { token_list.push(Token::Greater); index += 1; } // Greater
        else if regex!("^<[^=]").is_match(s) { token_list.push(Token::Less); index += 1; } // Less
        else if regex!("^>=").is_match(s) { token_list.push(Token::GreaterEqual); index += 2; } // Greater Equal
        else if regex!("^<=").is_match(s) { token_list.push(Token::LessEqual); index += 2; } // Less Equal

        else if regex!("^&").is_match(s) { token_list.push(Token::BitwiseAnd); index += 1; } // Bitwise And
        else if regex!("^\\|").is_match(s) { token_list.push(Token::BitwiseOr); index += 1; } // Bitwise Or
        else if regex!("^\\^").is_match(s) { token_list.push(Token::BitwiseXor); index += 1; } // Bitwise Xor
        else if regex!("^~").is_match(s) { token_list.push(Token::BitwiseNegate); index += 1; } // Bitwise Negate
        else if regex!("^<<").is_match(s) { token_list.push(Token::BitwiseShiftLeft); index += 1; } // Bitwise Shift Left
        else if regex!("^>>").is_match(s) { token_list.push(Token::BitwiseShiftRight); index += 1; } // Bitwise Shift Right

        else if regex!("^\'").is_match(s) { // Value Char
            token_list.push(lex_char(file_name, &mut line_num, s, &mut index)?);
        }
        else if let Some(cap) = regex_captures!("^[\\d_]+[.]?[\\d_]*", s) { // Value Number
            token_list.push(Token::ValueNumber(cap.to_string()));
            index += cap.len();
        }
        else if regex!("^#*\"").is_match(s) { // Value String
            token_list.push(lex_string(file_name, &mut line_num, s, &mut index)?);
        }
        else if let Some((_, cap)) = regex_captures!("^(true|false)\\s", s) { // Value Boolean
            if cap.eq("true") { token_list.push(Token::ValueBoolean(true)); }
            else if cap.eq("false")  { token_list.push(Token::ValueBoolean(false)); }
            else { err_message(file_name, line_num, "Lexer failed to id boolean.")?; }

            index += cap.len();
        }

        else if let Some(cap) = regex_captures!("^[^\\d\\s]{1}[\\w.]*", s) { // ID
            token_list.push(Token::ID(cap.to_string()));
            index += cap.len();
        }

        else if regex!("^\n").is_match(s) {
            line_num += 1;

            if let Some(last) = token_list.last() {
                match last {
                    Token::LineNum(_) => { token_list.pop(); }
                    _ => {}
                }
            }
            token_list.push(Token::LineNum(line_num));

            index += 1;
        }
        else { index += 1; }
    }

    Ok(token_list)
}

#[rustfmt::skip]
fn lex_char(file_name: &str, line_num: &mut usize, s: &str, index: &mut usize) -> Result<Token, String> {
    if regex!("^\'\'\'").is_match(s) { *index += 3; Ok(Token::ValueChar('\'')) }
    else if regex!("^\'\r\n\'").is_match(s) { *line_num += 1; *index += 4; Ok(Token::ValueChar('\n')) }
    else if regex!("^\'\n\'").is_match(s) { *line_num += 1; *index += 3; Ok(Token::ValueChar('\n')) }
    else if s.starts_with("\'\\n\'") { *index += 4; Ok(Token::ValueChar('\n')) }
    else if s.starts_with("\'\\t\'") { *index += 4; Ok(Token::ValueChar('\t')) }
    else if regex!("^\'.\'").is_match(s) { *index += 3; Ok(Token::ValueChar(s.chars().nth(1).unwrap())) }
    else { err_message(file_name, *line_num, "Lexer failed to id char or char is actually a string type.") }
}

#[rustfmt::skip]
fn lex_string(file_name: &str, line_num: &mut usize, s: &str, index: &mut usize) -> Result<Token, String> {
    let cap = match regex_captures!("^#*\"", s) {
        Some(s) => s,
        None => {
            return err_message(
                file_name,
                *line_num,
                "Lexer failed to process starting quote.",
            )
        }
    };
    let quote_size = cap.len();

    let mut found = false;
    let mut i = quote_size;
    while !found && i < s.len() {
        let s = &s[i..s.len()];

        if let Some(cap) = regex_captures!("^\"#*", s) {
            if cap.len() == quote_size { found = true; }
            else { i += cap.len(); }
        }
        else { i += 1; }
    }

    if !found {
        err_message(file_name, *line_num, "Lexer failed to find ending quote.")
    }
    else {
        let q_str = s[quote_size..i].to_string();
        *line_num += q_str.matches("\n").count();

        *index += i + quote_size;
        Ok(Token::ValueString(q_str))
    }
}

//TODO Better tests.
#[cfg(test)]
mod tests {
    use crate::lexer::{pre_process, remove_comments, tokenize};
    use std::{fs::File, io::Read};

    #[test]
    fn test_example_file() {
        let mut f_str = String::new();
        let mut file = File::open("../../../../specs/0/example.kot").unwrap();
        file.read_to_string(&mut f_str).unwrap();

        let contents = remove_comments(f_str);
        println!("\n\nNo Comments:\n\n{}", contents);

        let contents = pre_process("example.kot", contents).unwrap();
        println!("\n\nPre-Process:\n\n{}", contents.0);
        println!("\n\nPre-Process Metadata:\n\n{}", contents.1);
        println!();

        println!("\n\nTokens: \n");
        let t_list = tokenize("example.kot", &contents.0).unwrap();
        println!("{:?}", t_list);
    }
}

//<editor-fold desc="OLD">
// pub fn pre_process_entry(
//     write_data: &mut WriteData,
//     file_name: &str,
//     lines: &mut Vec<String>,
// ) -> Result<Vec<String>, String> {
//     let mut imports = Vec::new();
//     imports.push(String::from(file_name));
//
//     for (index, line) in lines.iter_mut().enumerate() {
//         if line.starts_with("BUILD")
//             || line.starts_with("#INFO")
//             || line.starts_with("#REQUIRE")
//             || line.starts_with("#IMPORT")
//         {
//             let spilt: Vec<&str> = line.split_whitespace().collect();
//             let spilt: Vec<String> = spilt.iter().map(|s| s.to_string()).collect();
//
//             if spilt.get(0).unwrap().eq("#BUILD") {
//                 if spilt.len() == 3 {
//                     if spilt.get(1).unwrap().eq("binary_name") {
//                         write_data.build_data.0 = spilt.get(2).unwrap().clone();
//                     }
//                     else {
//                         return err_message(
//                             file_name,
//                             index,
//                             "'#BUILD' does not have a valid first argument.",
//                         );
//                     }
//                 }
//                 else {
//                     return err_message(
//                         file_name,
//                         index,
//                         "'#BUILD' should always have 2 arguments.",
//                     );
//                 }
//             }
//             else if spilt.get(0).unwrap().eq("#INFO") {
//                 if spilt.len() >= 3 {
//                     if spilt.get(1).unwrap().eq("name") {
//                         write_data.metadata.name = spilt[2..spilt.len()].join(" ");
//                     }
//                     if spilt.get(1).unwrap().eq("authors") {
//                         write_data.metadata.authors =
//                             spilt[2..spilt.len()].iter().map(|s| s.clone()).collect();
//                     }
//                     if spilt.get(1).unwrap().eq("license") {
//                         write_data.metadata.license = spilt[2..spilt.len()].join(" ");
//                     }
//                     else if spilt.len() == 3 {
//                         if spilt.get(1).unwrap().eq("version") {
//                             write_data.metadata.version = spilt.get(2).unwrap().clone();
//                         }
//                         else if spilt.get(1).unwrap().eq("website") {
//                             write_data.metadata.website = "https://".to_string();
//                             write_data
//                                 .metadata
//                                 .website
//                                 .push_str(spilt.get(2).unwrap().clone().as_str());
//                         }
//                         else if spilt.get(1).unwrap().eq("git") {
//                             write_data.metadata.git = "https://".to_string();
//                             write_data
//                                 .metadata
//                                 .git
//                                 .push_str(spilt.get(2).unwrap().clone().as_str());
//                         }
//                     }
//                     else {
//                         return err_message(
//                             file_name,
//                             index,
//                             "'#INFO ARG' should only have 2 arguments. Unless it is for name, authors, or license.",
//                         );
//                     }
//                 }
//                 else {
//                     return err_message(file_name, index, "'#INFO' needs at least 2 arguments.");
//                 }
//             }
//             else if spilt.get(0).unwrap().eq("#REQUIRE") {
//                 if spilt.len() == 3 {
//                     if spilt.get(1).unwrap().eq("hta_version") {
//                         let v = match version::parse_version_str(spilt.get(2).unwrap()) {
//                             None => return err_message(file_name, index, "'#REQUIRE hta_version' has invalid second arg. Should be in format 'x.x.x.'"),
//                             Some(v) => v,
//                         };
//
//                         if !version::is_version_ge(write_data.compiler_version, v) {
//                             return err_message(file_name, index, "'#REQUIRE hta_version' is less then the compiler version, please either increase the version or use an older compiler.");
//                         }
//                     }
//                     else if spilt.get(1).unwrap().eq("native_lib") {
//                         return err_message(
//                             file_name,
//                             index,
//                             "#REQUIRE native_lib is not implemented.",
//                         );
//                     }
//                     else {
//                         return err_message(
//                             file_name,
//                             index,
//                             "'#REQUIRE' does not have a valid first argument.",
//                         );
//                     }
//                 }
//                 else {
//                     return err_message(
//                         file_name,
//                         index,
//                         "'#REQUIRE' should only have 2 arguments.",
//                     );
//                 }
//             }
//             else if spilt.get(0).unwrap().eq("#IMPORT") {
//                 if spilt.len() == 2 {
//                     let i_name = spilt.get(1).unwrap().clone();
//
//                     if imports.contains(&i_name) {
//                         return err_message(
//                             file_name,
//                             index,
//                             format!(
//                                 "'#IMPORT' tries to import already imported file. ({})",
//                                 i_name
//                             )
//                             .as_str(),
//                         );
//                     }
//
//                     imports.push(i_name.clone());
//                 }
//                 else {
//                     return err_message(
//                         file_name,
//                         index,
//                         "'#IMPORT' should only have 1 arguments.",
//                     );
//                 }
//             }
//
//             line.drain(0..line.len());
//         }
//     }
//
//     Ok(imports)
// }
//
// pub fn pre_process(path: &str, lines: &mut Vec<String>) -> Result<String, String> {
//     let file = Path::new(path);
//     let mut file_name = file.file_name().unwrap().to_str().unwrap().to_string();
//     file_name.drain((file_name.len() - file.extension().unwrap().len() - 1)..file_name.len());
//     let mut namespace = (file_name, false);
//
//     let mut define_map = HashMap::new();
//     let mut ac = AhoCorasick::new(&[] as &[String]);
//
//     for (index, line) in lines.iter_mut().enumerate() {
//         if line.starts_with("#") {
//             if line.starts_with("#NAMESPACE") || line.starts_with("#DEFINE") {
//                 let spilt: Vec<&str> = line.split_whitespace().collect();
//                 let spilt: Vec<String> = spilt.iter().map(|s| s.to_string()).collect();
//
//                 //TODO Regex namespace to make sure it falls within the criteria.
//                 if line.starts_with("#NAMESPACE") {
//                     if spilt.len() == 2 {
//                         if namespace.1 {
//                             return err_message(path, index, "'#NAMESPACE' was already used.");
//                         }
//
//                         namespace.0 = spilt.get(1).unwrap().clone();
//                         namespace.1 = true;
//                     }
//                     else {
//                         return err_message(
//                             path,
//                             index,
//                             "'#NAMESPACE' should only have one argument.",
//                         );
//                     }
//                 }
//                 else if line.starts_with("#DEFINE") {
//                     if spilt.len() >= 3 {
//                         line.drain(0.."#DEFINE".len());
//
//                         //TODO Should this trim end?
//                         let mut var = line.trim_start().to_string();
//                         var.drain(0..spilt.get(1).unwrap().len());
//                         let var = var.trim_start().to_string();
//
//                         define_map.insert(spilt.get(1).unwrap().clone(), var);
//
//                         let patterns = define_map.keys();
//                         ac = AhoCorasick::new(patterns);
//                     }
//                     else {
//                         return err_message(
//                             path,
//                             index,
//                             "'#DEFINE' should have at least 2 arguments.",
//                         );
//                     }
//                 }
//                 else {
//                     return err_message(path, index, "Pre-Processor statement is not recognised.");
//                 }
//
//                 line.drain(0..line.len());
//             }
//         }
//         else {
//             // Find
//             let mut matches = Vec::new();
//             for mat in ac.find_iter(line) {
//                 matches.push((mat.pattern(), mat.start(), mat.end()));
//             }
//
//             // Replace
//             let values: Vec<&String> = define_map.values().collect();
//             for mat in matches.iter() {
//                 line.replace_range(mat.1..mat.2, values.get(mat.0).unwrap().as_str());
//             }
//         }
//     }
//
//     Ok(namespace.0)
// }

//TODO MOVE
//TODO Compile.
// pub fn compile() -> Result<(Vec<Instructions>, HashMap<Tag, TagMap>), String> {
//     //let mut variables = HashMap::new(); // Hashmap to keep track of variable types.
//
//     Ok((Vec::new(), HashMap::new()))
// }

//TODO Linker Check.
// pub fn linker() {}

//TODO Safety Checks.
// pub fn safety_checks() {}

//TODO Optimize.
// pub fn optimize() {}
//</editor-fold>
