use crate::ast::Exp;

#[derive(Debug)]
pub struct Table {
    fields: Vec<Field>
}

#[derive(Debug)]
pub struct Field {
    key: Box<Exp>,
    value: Box<Exp>
}

impl Table {
    pub fn new(fields: Vec<Field>) -> Self {
        Self {
            fields
        }
    }
}

impl Into<Exp> for Table {
    fn into(self) -> Exp {
        Exp::Table(self)
    }
}

impl Field {
    pub fn new(key: Exp, value: Exp) -> Self {
        Self {
            key: Box::new(key),
            value: Box::new(value)
        }
    }
}
