pub mod exps;
pub mod node;
pub mod stats;
pub mod visitors;

use crate::ast::node::Node;
use exps::*;
use stats::*;

pub type Block = Vec<Node<Stat>>;

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
