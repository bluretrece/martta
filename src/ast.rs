use crate::error::Error;

#[derive(Clone, Debug)]
pub enum Prog {
    Body(Block),
}

pub type Block = Vec<Stmt>;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Return(Expr),
    Assign(String, Expr),
    ReAssign(String, Expr),
    IfStatement(Expr, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    IfElse(Expr, Vec<Stmt>, Vec<Stmt>),
    Func(String, Vec<String>, Block),
    Class(String, Block),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Expr {
    Int(i32),
    Bool(bool),
    Str(String),
    Var(String),
    Binary(Box<Expr>, Operator, Box<Expr>),
    Call(Call),
    Function(Vec<String>, Vec<Stmt>),
    List(Vec<Expr>),
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Call {
    Function(Function),
    Class(Class),
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Function {
    pub func: String,
    pub args: Vec<Expr>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Class {
    pub identifier: String,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Or,
    And,
    GreaterThan,
    LessThan,
    LessOrEqual,
    EqTo,
    SumTo,
}
