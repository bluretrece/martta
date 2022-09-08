use crate::error::Error;
use std::convert::From;

#[derive(Clone, Debug)]
pub enum Prog {
    Body(Block),
}

pub type Block = Vec<Stmt>;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum HirExpr {
    Literal(Literal, Type),
    Binary(Box<HirExpr>, Operator, Box<HirExpr>, Type),
    Nothing,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Type {
    Primitive(Primitive),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Primitive {
    Int,
    Bool,
}

impl From<HirExpr> for Type {
    fn from(hir: HirExpr) -> Self {
        match hir {
            HirExpr::Literal(_, Type::Primitive(Primitive::Int)) => Type::Primitive(Primitive::Int),
            HirExpr::Literal(_, Type::Primitive(Primitive::Bool)) => {
                Type::Primitive(Primitive::Bool)
            }
            HirExpr::Binary(_, _, _, Type::Primitive(Primitive::Int)) => {
                Type::Primitive(Primitive::Int)
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Literal {
    Bool(bool),
    Int(i32),
}

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
