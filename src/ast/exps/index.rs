use crate::ast::Exp;

#[derive(Debug)]
pub struct Index {
    lhs: Box<Exp>,
    exp: Box<Exp>
}

impl Index {
    pub fn new(lhs: Exp, exp: Exp) -> Self {
        Self {
            lhs: Box::new(lhs),
            exp: Box::new(exp)
        }
    }
}

impl Into<Exp> for Index {
    fn into(self) -> Exp {
        Exp::Index(self)
    }
}
