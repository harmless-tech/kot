mod lexer;
mod tokens;
mod writer;

//TODO Maybe don't use files, so this can be used without a filesystem.
//TODO Debug is determined by the built binary.

pub fn compile(/* List of names and contents */) -> () /* List of names and compiled binary */
{
}

//TODO Maybe this should only be in the binary.
pub fn build(/* Build file */) {}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_example_file() {
        //TODO

        let mut f_str = String::new();
        let mut file = File::open("../../../../specs/0/example.kot").unwrap();
        file.read_to_string(&mut f_str);

        //println!("Import: {}", f_str);
    }
}
