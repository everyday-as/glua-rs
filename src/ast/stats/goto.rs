#[derive(Clone, Debug)]
pub struct Goto {
    pub label: String,
}

impl Goto {
    pub fn new(label: String) -> Self {
        Self { label }
    }
}
