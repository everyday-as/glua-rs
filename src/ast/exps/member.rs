use crate::ast::Exp;

#[derive(Debug)]
pub struct Member {
    lhs: Box<Exp>,
    name: String
}

impl Member {
    pub fn new(lhs: Exp, name: String) -> Self {
        Self {
            lhs: Box::new(lhs),
            name
        }
    }
}

impl Into<Exp> for Member {
    fn into(self) -> Exp {
        Exp::Member(self)
    }
}
