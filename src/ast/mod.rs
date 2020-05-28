pub mod stats;
pub mod exps;

use stats::*;
use exps::*;

pub type Block = Vec<Stat>;

#[derive(Debug)]
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
    IfElse(IfElse),
    MethodCall(MethodCall),
    None,
    RepeatUntil(RepeatUntil),
    Return(Return),
    VarDef(VarDef),
    While(While),
}

#[derive(Debug)]
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
    Table(Table),
    Unary(Unary),
    VarArgs
}
