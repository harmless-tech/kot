use log::error;
use std::process::exit;

fn err_message<T>(file_name: &str, line_num: usize, message: &str) -> Result<T, String> {
    Err(format!(
        "File: {}, Line: {}: {}",
        file_name,
        line_num + 1,
        message
    ))
}

//TODO Should this use borrowing instead for contents?
pub fn remove_comments(file_name: &str, contents: String) -> Result<String, String> {
    let mut lines: Vec<String> = contents
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    // Remove /* */ comments.
    let mut in_quotes = false;
    let mut comment_first_slash;
    let mut comment_first_star;
    let mut in_block_comment = false;

    let mut index = 0;
    while index < lines.len() {
        let line = match lines.get_mut(index) {
            Some(l) => l,
            None => {
                error!(
                    "[FATAL] Lexer somehow left the lines vector range! (lexer::remove_comments())"
                );
                exit(-1);
            }
        };

        let mut loc1 = -1_i32;
        let mut loc2 = -1_i32;

        comment_first_slash = false;
        comment_first_star = false;

        for (i, chr) in line.chars().enumerate() {
            if chr == '"' {
                in_quotes = !in_quotes;
                comment_first_slash = false;
                comment_first_star = false;
            }
            else if chr == '/' {
                if !in_quotes && comment_first_star && in_block_comment {
                    loc2 = (i as i32) + 1;
                    in_block_comment = false;
                }
                else if !in_quotes && comment_first_star {
                    return err_message(
                        file_name,
                        index,
                        "There is a '*/' without a matching '/*'.",
                    );
                }
                else if !in_quotes {
                    comment_first_slash = true;
                }

                comment_first_star = false;
            }
            else if chr == '*' {
                if !in_quotes && comment_first_slash && !in_block_comment {
                    loc1 = (i as i32) - 1;
                    in_block_comment = true;
                }
                else if !in_quotes {
                    comment_first_star = true;
                }

                comment_first_slash = false;
            }
            else {
                comment_first_slash = false;
                comment_first_star = false;
            }
        }

        if loc1 >= 0 && loc2 >= 0 {
            line.drain((loc1 as usize)..(loc2 as usize));
            index -= 1;
        }
        else if loc1 >= 0 {
            line.drain((loc1 as usize)..line.len());
        }
        else if loc2 >= 0 {
            line.drain(0..(loc2 as usize));
        }
        else if in_block_comment {
            line.drain(0..line.len());
        }

        index += 1;
    }

    // Remove // comments.
    let mut in_quotes = false;
    let mut comment_first_slash;

    for line in lines.iter_mut() {
        comment_first_slash = false;

        let mut loc = -1_i32;

        for (i, chr) in line.chars().enumerate() {
            if chr == '"' {
                in_quotes = !in_quotes;
                comment_first_slash = false;
            }
            else if chr == '/' {
                if !in_quotes && comment_first_slash && !line.starts_with("#") {
                    loc = (i as i32) - 1;
                    break;
                }
                else if !in_quotes {
                    comment_first_slash = true;
                }
            }
            else {
                comment_first_slash = false;
            }
        }

        if loc >= 0 {
            line.drain((loc as usize)..line.len());
        }
    }

    Ok(lines.join("\n"))
}

pub fn pre_process(file_name: &str, contents: String) -> Result<(String, Vec<String>, String), String> {
    let mut lines: Vec<String> = contents
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let mut spec_vec = Vec::new();
    if let Some(first_line) = lines.get_mut(0) {
        spec_vec = first_line.split_whitespace().map(|s| s.to_string()).collect();
        first_line.drain(0..first_line.len());
    }
    else {
        return err_message(file_name, 0, "The first line does not exist.");
    }

    if !spec_vec.contains(&kot::KOT_VERSION.to_string()) {
        return err_message(file_name, 0, "KOT_SPEC_# is missing or does not match with the spec for this compiler.");
    }

    let mut metadata = Vec::new();
    for line in lines.iter_mut() {
        if line.starts_with("#") {
            metadata.push(line.drain(1..line.len()).collect::<String>());
            line.drain(0..1);
        }
    }

    Ok((lines.join("\n"), spec_vec, metadata.join("\n")))
}

//TODO Should this use borrowing instead for contents?
pub fn tokenize(contents: String) -> Result<(), String> {
    Ok(())
}

//TODO Better tests.
#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use crate::lexer::{remove_comments, pre_process};

    #[test]
    fn test_example_file() {
        let mut f_str = String::new();
        let mut file = File::open("../../../../specs/0/example.kot").unwrap();
        file.read_to_string(&mut f_str);

        let mut contents = remove_comments("example.kot", f_str).unwrap();
        println!("\nNo Comments:\n{}", contents);

        let mut contents = pre_process("example.kot", contents).unwrap();
        println!("\nPre-Process:\n{}", contents.0);
        println!("\nPre-Process Specs:\n{:?}", contents.1);
        println!("\nPre-Process Metadata:\n{}", contents.2);
        println!();
    }

    #[test]
    fn test_example_build_file() {
        let mut f_str = String::new();
        let mut file = File::open("../../../../specs/0/kot.build").unwrap();
        file.read_to_string(&mut f_str);

        let mut contents = remove_comments("kot.build", f_str).unwrap();
        println!("\nNo Comments:\n{}", contents);

        let mut contents = pre_process("kot.build", contents).unwrap();
        println!("\nPre-Process:\n{}", contents.0);
        println!("\nPre-Process Specs:\n{:?}", contents.1);
        println!("\nPre-Process Metadata:\n{}", contents.2);
        println!();
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
//         if line.starts_with("#BUILD")
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
