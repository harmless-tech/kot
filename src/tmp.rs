// TODO: Remove!
// This is a possible error handling method.

#[derive(Debug)]
enum Val {
    Val1,
    Val2,
    Val3(String),
}
impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::write;

        match self {
            Val::Val1 => write(f, format_args!("Val1 Error")),
            Val::Val2 => write(f, format_args!("Val2 Error")),
            Val::Val3(s) => write(f, format_args!("Val3 Error - {s}")),
        }
    }
}
impl std::error::Error for Val {}

fn tester() -> anyhow::Result<()> {
    Err(Val::Val3("To String".to_string()).into())
}
