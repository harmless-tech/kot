use crate::{
    ast::Ident,
    parser::{ParseData, ParseResult},
    Pos,
};

pub(super) fn p_assign(id: Ident, pos: Pos, data: &mut ParseData) -> ParseResult {
    // TODO: p_block opt!!!
    // abc = 123
    // or
    // abc { return 123 }

    // let block = p_block(data)?;

    todo!()
}
