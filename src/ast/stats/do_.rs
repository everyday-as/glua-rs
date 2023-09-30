use crate::ast::Block;

#[derive(Clone, Copy, Debug)]
pub struct Do<'a> {
    pub body: Block<'a>,
}

impl<'a> Do<'a> {
    pub fn new(body: Block<'a>) -> Self {
        Self { body }
    }
}
