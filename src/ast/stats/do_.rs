use crate::ast::{Stat, Block};

#[derive(Clone, Debug)]
pub struct Do {
    pub body: Block
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
