use crate::data::{Ast, WrappedValue};

// TODO: Support floats and uints.
// TODO: Error with position.
pub fn parse_number(num: &str, radix: u32) -> anyhow::Result<WrappedValue> {
    assert!(!num.contains('.'), "No float support yet!");
    Ok(WrappedValue::Int64(i64::from_str_radix(num, radix)?))
}
