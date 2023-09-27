use crate::ast::Block;

#[derive(Clone, Debug)]
pub struct Do {
    pub body: Block,
}

impl Do {
    pub fn new(body: Block) -> Self {
        Self { body }
    }
}
