#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Label<'a> {
    name: &'a str,
}

impl<'a> Label<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}
