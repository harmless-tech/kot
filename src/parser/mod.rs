mod parse_item;
mod parse_tree;

use crate::{
    data::{Ast, PosAst, PosToken, Token},
    Pos,
};

// TODO: Get rid of static?
static EOF_TOKEN: PosToken = PosToken::eof(Pos::new(usize::MAX, usize::MAX));

// TODO: Add environment to track idents.
#[derive(Debug)]
struct Parser {
    tokens: Vec<PosToken>,
    index: usize,
}
impl Parser {
    fn new(tokens: Vec<PosToken>) -> Self {
        Self { tokens, index: 0 }
    }

    fn within(&self) -> bool {
        self.index < self.tokens.len()
    }

    fn get(&mut self) -> Option<&PosToken> {
        let tmp = self.tokens.get(self.index);
        self.index += 1;
        tmp
    }

    fn peek(&self) -> Option<&PosToken> {
        self.peek_i(0)
    }

    fn skip(&mut self) {
        self.skip_i(1);
    }

    fn peek_i(&self, offset: usize) -> Option<&PosToken> {
        self.tokens.get(self.index + offset)
    }

    fn skip_i(&mut self, i: usize) {
        self.index += i;
    }
}

pub fn parse(tokens: Vec<PosToken>) -> anyhow::Result<PosAst> {
    let mut parser = Parser::new(tokens);

    let root = PosAst::new(
        Ast::FakeGlobalBlock(parse_tree::p_expression(&mut parser)?.into()),
        Pos::new(0, 0),
    );

    match parser.peek() {
        Some(PosToken {
            token: Token::Eof,
            pos,
        }) => println!("DBG: Found Eof at {pos:?}"),
        _ => panic!("Required EOF token but not found!!!"),
    }

    dbg!(&root);

    Ok(root)
}

fn p_block() {
    todo!()
}

fn map_opt_token(opt_token: Option<&PosToken>) -> &PosToken {
    opt_token.map_or(&EOF_TOKEN, |t| t)
}
