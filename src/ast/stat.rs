use crate::ast::{
    exps::{FunctionCall, MethodCall},
    stats::{
        Assignment, Do, For, ForIn, FunctionDef, Goto, IfElse, RepeatUntil, Return, VarDef, While,
    },
};

#[derive(Clone, Debug)]
pub enum Stat<'a> {
    Assignment(Assignment<'a>),
    Break,
    /// GMod specific continue statement
    Continue,
    Do(Do<'a>),
    For(For<'a>),
    ForIn(ForIn<'a>),
    FunctionCall(FunctionCall<'a>),
    FunctionDef(FunctionDef<'a>),
    // GMod specific goto statement
    Goto(Goto<'a>),
    IfElse(IfElse<'a>),
    // GMod specific label statement
    Label(&'a str),
    MethodCall(MethodCall<'a>),
    None,
    RepeatUntil(RepeatUntil<'a>),
    Return(Return<'a>),
    VarDef(VarDef<'a>),
    While(While<'a>),
}

impl<'a> From<Assignment<'a>> for Stat<'a> {
    fn from(value: Assignment<'a>) -> Self {
        Self::Assignment(value)
    }
}

impl<'a> From<Do<'a>> for Stat<'a> {
    fn from(value: Do<'a>) -> Self {
        Self::Do(value)
    }
}

impl<'a> From<For<'a>> for Stat<'a> {
    fn from(value: For<'a>) -> Self {
        Self::For(value)
    }
}

impl<'a> From<ForIn<'a>> for Stat<'a> {
    fn from(value: ForIn<'a>) -> Self {
        Self::ForIn(value)
    }
}

impl<'a> From<FunctionCall<'a>> for Stat<'a> {
    fn from(value: FunctionCall<'a>) -> Self {
        Self::FunctionCall(value)
    }
}

impl<'a> From<FunctionDef<'a>> for Stat<'a> {
    fn from(value: FunctionDef<'a>) -> Self {
        Self::FunctionDef(value)
    }
}

impl<'a> From<Goto<'a>> for Stat<'a> {
    fn from(value: Goto<'a>) -> Self {
        Self::Goto(value)
    }
}

impl<'a> From<IfElse<'a>> for Stat<'a> {
    fn from(value: IfElse<'a>) -> Self {
        Self::IfElse(value)
    }
}

impl<'a> From<MethodCall<'a>> for Stat<'a> {
    fn from(value: MethodCall<'a>) -> Self {
        Self::MethodCall(value)
    }
}

impl<'a> From<RepeatUntil<'a>> for Stat<'a> {
    fn from(value: RepeatUntil<'a>) -> Self {
        Self::RepeatUntil(value)
    }
}

impl<'a> From<Return<'a>> for Stat<'a> {
    fn from(value: Return<'a>) -> Self {
        Self::Return(value)
    }
}

impl<'a> From<VarDef<'a>> for Stat<'a> {
    fn from(value: VarDef<'a>) -> Self {
        Self::VarDef(value)
    }
}

impl<'a> From<While<'a>> for Stat<'a> {
    fn from(value: While<'a>) -> Self {
        Self::While(value)
    }
}
