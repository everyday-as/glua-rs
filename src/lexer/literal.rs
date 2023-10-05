use byteyarn::YarnRef;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Literal<'a> {
    Bool(bool),
    Nil,
    Number(f64),
    String(YarnRef<'a, [u8]>),
}
