use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct Member {
    pub lhs: Box<Exp>,
    pub name: String
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
