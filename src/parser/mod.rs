use crate::token::{PosToken, Tokens};
use std::ops::Range;

struct Parser {
    tokens: Tokens,
    index: usize,
    eof: PosToken,
}
impl Parser {
    fn new(tokens: Tokens) -> Self {
        Self {
            tokens,
            index: 0,
            eof: PosToken::eof(),
        }
    }

    fn within(&self) -> bool {
        self.index < self.tokens.len()
    }

    fn range(&self, range: Range<usize>) -> Vec<&PosToken> {
        self.tokens[range].iter().collect()
    }

    fn peek(&self) -> &PosToken {
        self.peek_i(0)
    }

    fn peek_i(&self, offset: usize) -> &PosToken {
        self.tokens.get(self.index + offset).unwrap_or(&self.eof)
    }

    fn i(&mut self, i: usize) {
        self.index += i;
    }
}

pub fn parse(tokens: Tokens) -> anyhow::Result<()> {
    todo!()
}
