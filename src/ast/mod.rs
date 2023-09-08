pub mod stats;
pub mod exps;
pub mod node;
pub mod visitors;

use stats::*;
use exps::*;
use crate::ast::node::Node;

pub type Block = Vec<Node<Stat>>;

#[derive(Clone, Debug)]
pub enum Stat {
    Assignment(Node<Assignment>),
    Break,
    /// GMod specific continue statement
    Continue,
    Do(Node<Do>),
    For(Node<For>),
    ForIn(Node<ForIn>),
    FunctionCall(Node<FunctionCall>),
    FunctionDef(Node<FunctionDef>),
    IfElse(Node<IfElse>),
    MethodCall(Node<MethodCall>),
    None,
    RepeatUntil(Node<RepeatUntil>),
    Return(Node<Return>),
    VarDef(Node<VarDef>),
    While(Node<While>),
}

#[derive(Clone, Debug)]
pub enum Exp {
    Binary(Node<Binary>),
    Bool(Node<bool>),
    Function(Node<Function>),
    FunctionCall(Node<FunctionCall>),
    Index(Node<Index>),
    Member(Node<Member>),
    MethodCall(Node<MethodCall>),
    Nil,
    Number(Node<f64>),
    Ref(Node<String>),
    String(Node<String>),
    Table(Node<TableConstructor>),
    Unary(Node<Unary>),
    VarArgs
}
