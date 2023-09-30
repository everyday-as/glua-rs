use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal<'a> {
    Bool(bool),
    Nil,
    Number(f64),
    String(Cow<'a, str>),
}
