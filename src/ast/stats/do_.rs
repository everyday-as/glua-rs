use crate::ast::{Exp, Stat};

#[derive(Debug)]
pub struct Do {
    body: Vec<Stat>
}

impl Do {
    pub fn new(body: Vec<Stat>) -> Self {
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