use crate::ast::exps::{
    Binary, Function, FunctionCall, Index, Member, MethodCall, TableConstructor, Unary,
};

#[derive(Clone, Debug)]
pub enum Exp {
    Binary(Binary),
    Bool(bool),
    Function(Function),
    FunctionCall(FunctionCall),
    Index(Index),
    Member(Member),
    MethodCall(MethodCall),
    Nil,
    Number(f64),
    Ref(String),
    String(String),
    Table(TableConstructor),
    Unary(Unary),
    VarArgs,
}

impl From<Binary> for Exp {
    fn from(value: Binary) -> Self {
        Self::Binary(value)
    }
}

impl From<bool> for Exp {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Function> for Exp {
    fn from(value: Function) -> Self {
        Self::Function(value)
    }
}

impl From<FunctionCall> for Exp {
    fn from(value: FunctionCall) -> Self {
        Self::FunctionCall(value)
    }
}

impl From<Index> for Exp {
    fn from(value: Index) -> Self {
        Self::Index(value)
    }
}

impl From<Member> for Exp {
    fn from(value: Member) -> Self {
        Self::Member(value)
    }
}

impl From<MethodCall> for Exp {
    fn from(value: MethodCall) -> Self {
        Self::MethodCall(value)
    }
}

impl From<f64> for Exp {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<TableConstructor> for Exp {
    fn from(value: TableConstructor) -> Self {
        Self::Table(value)
    }
}

impl From<Unary> for Exp {
    fn from(value: Unary) -> Self {
        Self::Unary(value)
    }
}
