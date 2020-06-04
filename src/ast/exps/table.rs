use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct TableConstructor {
    pub fields: Vec<Field>
}

#[derive(Clone, Debug)]
pub struct Field {
    pub key: Option<Box<Exp>>,
    pub value: Box<Exp>,
}

impl TableConstructor {
    pub fn new(fields: Vec<Field>) -> Self {
        Self {
            fields
        }
    }
}

impl Into<Exp> for TableConstructor {
    fn into(self) -> Exp {
        Exp::Table(self)
    }
}

impl Field {
    pub fn new(key: Option<Exp>, value: Exp) -> Self {
        Self {
            key: key.map(|key| Box::new(key)),
            value: Box::new(value),
        }
    }
}
