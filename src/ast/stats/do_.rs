use crate::ast::{Stat, Block};

#[derive(Debug)]
pub struct Do {
    body: Block
}

impl Do {
    pub fn new(body: Block) -> Self {
        Self {
            body
        }
    }
}

impl Into<Stat> for Do {
    fn into(self) -> Stat {
        Stat::Do(self)
    }
}
