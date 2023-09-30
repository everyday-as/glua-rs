#[derive(Clone, Copy, Debug)]
pub struct Goto<'a> {
    pub label: &'a str,
}

impl<'a> Goto<'a> {
    pub fn new(label: &'a str) -> Self {
        Self { label }
    }
}
