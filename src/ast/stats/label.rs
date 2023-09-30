#[derive(Clone, Debug, PartialEq)]
pub struct Label {
    name: String,
}

impl Label {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
