pub mod stats;

use stats::*;

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
    FunctionCall(FunctionCall),
    Index(Index),
    Function(Function),
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

#[derive(Debug)]
pub struct Table {
    fields: Vec<Field>
}

#[derive(Debug)]
pub struct Field {
    key: Box<Exp>,
    value: Box<Exp>
}

#[derive(Debug)]
pub struct Function {
    params: Vec<String>,
    body: Block
}

#[derive(Debug)]
pub struct FunctionCall {
    lhs: Box<Exp>,
    args: Vec<Exp>
}

#[derive(Debug)]
pub struct MethodCall {
    lhs: Box<Exp>,
    name: String,
    args: Vec<Exp>
}

#[derive(Debug)]
pub struct Binary {
    lhs: Box<Exp>,
    op: BinOp,
    rhs: Box<Exp>
}

#[derive(Debug)]
pub struct Unary {
    op: UnOp,
    exp: Box<Exp>
}

#[derive(Debug)]
pub struct Index {
    lhs: Box<Exp>,
    exp: Box<Exp>
}

#[derive(Debug)]
pub struct Member {
    lhs: Box<Exp>,
    name: String
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    And,
    Concat,
    Div,
    Eq,
    Exp,
    Gt,
    GtEq,
    Lt,
    LtEq,
    Mod,
    Mul,
    Ne,
    Or,
    Sub,
}

#[derive(Debug)]
pub enum UnOp {
    Neg,
    Not,
    Len
}

impl Table {
    pub fn new(fields: Vec<Field>) -> Self {
        Self {
            fields
        }
    }
}

impl Into<Exp> for Table {
    fn into(self) -> Exp {
        Exp::Table(self)
    }
}

impl Field {
    pub fn new(key: Exp, value: Exp) -> Self {
        Self {
            key: Box::new(key),
            value: Box::new(value)
        }
    }
}

impl Function {
    pub fn new(params: Vec<String>, body: Block) -> Self {
        Self {
            params,
            body
        }
    }
}

impl Into<Exp> for Function {
    fn into(self) -> Exp {
        Exp::Function(self)
    }
}

impl FunctionCall {
    pub fn new(lhs: Exp, args: Vec<Exp>) -> Self {
        Self {
            lhs: Box::new(lhs),
            args
        }
    }
}

impl Into<Exp> for FunctionCall {
    fn into(self) -> Exp {
        Exp::FunctionCall(self)
    }
}

impl MethodCall {
    pub fn new(lhs: Exp, name: String, args: Vec<Exp>) -> Self {
        Self {
            lhs: Box::new(lhs),
            name,
            args
        }
    }
}

impl Into<Exp> for MethodCall {
    fn into(self) -> Exp {
        Exp::MethodCall(self)
    }
}

impl Binary {
    pub fn new(lhs: Exp, op: BinOp, rhs: Exp) -> Self {
        Self {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs)
        }
    }
}

impl Into<Exp> for Binary {
    fn into(self) -> Exp {
        Exp::Binary(self)
    }
}

impl Unary {
    pub fn new(op: UnOp, exp: Exp) -> Self {
        Self {
            op,
            exp: Box::new(exp)
        }
    }
}

impl Into<Exp> for Unary {
    fn into(self) -> Exp {
        Exp::Unary(self)
    }
}

impl Index {
    pub fn new(lhs: Exp, exp: Exp) -> Self {
        Self {
            lhs: Box::new(lhs),
            exp: Box::new(exp)
        }
    }
}

impl Into<Exp> for Index {
    fn into(self) -> Exp {
        Exp::Index(self)
    }
}

impl Member {
    pub fn new(lhs: Exp, name: String) -> Self {
        Self {
            lhs: Box::new(lhs),
            name
        }
    }
}

impl Into<Exp> for Member {
    fn into(self) -> Exp {
        Exp::Member(self)
    }
}

impl Exp {
    pub fn eval(&self) -> f64 {
        match self {
            Exp::Number(num) => *num,

            Exp::Binary(binary) => binary.eval(),

            Exp::Unary(unary) => unary.eval(),

            _ => unimplemented!()
        }
    }
}

impl Binary {
    pub fn eval(&self) -> f64 {
        let lhs = self.lhs.eval();
        let rhs = self.rhs.eval();

        match self.op {
            BinOp::Add => lhs + rhs,
            BinOp::Div => lhs / rhs,
            BinOp::Exp => lhs.powf(rhs),
            BinOp::Gt => if lhs > rhs { 1f64 } else { 0f64 },
            BinOp::Mul => lhs * rhs,
            BinOp::Sub => lhs - rhs,

            _ => unimplemented!()
        }
    }
}

impl Unary {
    pub fn eval(&self) -> f64 {
        let val = self.exp.eval();

        match self.op {
            UnOp::Neg => -val,

            _ => unimplemented!()
        }
    }
}