use crate::ast::exps::{
    Binary, Function, FunctionCall, Index, Member, MethodCall, TableConstructor, Unary,
};

#[derive(Clone, Copy, Debug)]
pub enum Exp<'a> {
    Binary(Binary<'a>),
    Bool(bool),
    Function(Function<'a>),
    FunctionCall(FunctionCall<'a>),
    Index(Index<'a>),
    Member(Member<'a>),
    MethodCall(MethodCall<'a>),
    Nil,
    Number(f64),
    Ref(&'a str),
    String(&'a [u8]),
    Table(TableConstructor<'a>),
    Unary(Unary<'a>),
    VarArgs,
}

impl<'a> From<Binary<'a>> for Exp<'a> {
    fn from(value: Binary<'a>) -> Self {
        Self::Binary(value)
    }
}

impl From<bool> for Exp<'_> {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl<'a> From<Function<'a>> for Exp<'a> {
    fn from(value: Function<'a>) -> Self {
        Self::Function(value)
    }
}

impl<'a> From<FunctionCall<'a>> for Exp<'a> {
    fn from(value: FunctionCall<'a>) -> Self {
        Self::FunctionCall(value)
    }
}

impl<'a> From<Index<'a>> for Exp<'a> {
    fn from(value: Index<'a>) -> Self {
        Self::Index(value)
    }
}

impl<'a> From<Member<'a>> for Exp<'a> {
    fn from(value: Member<'a>) -> Self {
        Self::Member(value)
    }
}

impl<'a> From<MethodCall<'a>> for Exp<'a> {
    fn from(value: MethodCall<'a>) -> Self {
        Self::MethodCall(value)
    }
}

impl From<f64> for Exp<'_> {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl<'a> From<TableConstructor<'a>> for Exp<'a> {
    fn from(value: TableConstructor<'a>) -> Self {
        Self::Table(value)
    }
}

impl<'a> From<Unary<'a>> for Exp<'a> {
    fn from(value: Unary<'a>) -> Self {
        Self::Unary(value)
    }
}
