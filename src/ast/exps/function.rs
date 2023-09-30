use crate::ast::Block;

#[derive(Clone, Copy, Debug)]
pub struct Function<'a> {
    pub params: &'a [&'a str],
    pub body: Block<'a>,
}

impl<'a> Function<'a> {
    pub fn new(params: &'a [&'a str], body: Block<'a>) -> Self {
        Self { params, body }
    }
}
