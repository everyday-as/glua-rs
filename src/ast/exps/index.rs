use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct Index {
    pub lhs: Box<Exp>,
    pub exp: Box<Exp>
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
