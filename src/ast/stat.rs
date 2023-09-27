use crate::ast::exps::{FunctionCall, MethodCall};
use crate::ast::stats::{
    Assignment, Do, For, ForIn, FunctionDef, Goto, IfElse, Label, RepeatUntil, Return, VarDef,
    While,
};

#[derive(Clone, Debug)]
pub enum Stat {
    Assignment(Assignment),
    Break,
    /// GMod specific continue statement
    Continue,
    Do(Do),
    For(For),
    ForIn(ForIn),
    FunctionCall(FunctionCall),
    FunctionDef(FunctionDef),
    // GMod specific goto statement
    Goto(Goto),
    IfElse(IfElse),
    // GMod specific label statement
    Label(Label),
    MethodCall(MethodCall),
    None,
    RepeatUntil(RepeatUntil),
    Return(Return),
    VarDef(VarDef),
    While(While),
}

impl From<Assignment> for Stat {
    fn from(value: Assignment) -> Self {
        Self::Assignment(value)
    }
}

impl From<Do> for Stat {
    fn from(value: Do) -> Self {
        Self::Do(value)
    }
}

impl From<For> for Stat {
    fn from(value: For) -> Self {
        Self::For(value)
    }
}

impl From<ForIn> for Stat {
    fn from(value: ForIn) -> Self {
        Self::ForIn(value)
    }
}

impl From<FunctionCall> for Stat {
    fn from(value: FunctionCall) -> Self {
        Self::FunctionCall(value)
    }
}

impl From<FunctionDef> for Stat {
    fn from(value: FunctionDef) -> Self {
        Self::FunctionDef(value)
    }
}

impl From<Goto> for Stat {
    fn from(value: Goto) -> Self {
        Self::Goto(value)
    }
}

impl From<IfElse> for Stat {
    fn from(value: IfElse) -> Self {
        Self::IfElse(value)
    }
}

impl From<Label> for Stat {
    fn from(value: Label) -> Self {
        Self::Label(value)
    }
}

impl From<MethodCall> for Stat {
    fn from(value: MethodCall) -> Self {
        Self::MethodCall(value)
    }
}

impl From<RepeatUntil> for Stat {
    fn from(value: RepeatUntil) -> Self {
        Self::RepeatUntil(value)
    }
}

impl From<Return> for Stat {
    fn from(value: Return) -> Self {
        Self::Return(value)
    }
}

impl From<VarDef> for Stat {
    fn from(value: VarDef) -> Self {
        Self::VarDef(value)
    }
}

impl From<While> for Stat {
    fn from(value: While) -> Self {
        Self::While(value)
    }
}
