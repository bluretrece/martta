use crate::error::Error;
use std::convert::From;

#[derive(Clone, Debug)]
pub enum Prog {
    Body(Block),
}

pub type Block = Vec<Stmt>;
pub type HirBlock = Vec<HirNode>;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum HirExpr {
    Literal(Literal, Type),
    Binary(Box<HirNode>, Operator, Box<HirNode>, Type),
    Var(String, Type),
    Call(HirFunction),
    Nothing,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct HirFunction(pub String, pub Vec<HirNode>);

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Type {
    Primitive(Primitive),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum HirNode {
    HirExpr(HirExpr),
    HirStmt(HirStmt),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum HirStmt {
    IfElse(Box<HirNode>, Vec<HirNode>, Vec<HirNode>, Type),
    IfStatement(Box<HirNode>, Vec<HirNode>, Type),
    Assign(String, Box<HirNode>, Type),
    Function(String, Vec<String>, HirBlock, Type),
    Return(Box<HirNode>, Type),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Primitive {
    Int,
    Bool,
    Str,
    Unit,
}

impl From<HirNode> for Type {
    fn from(hir: HirNode) -> Self {
        match hir {
            HirNode::HirExpr(HirExpr::Literal(_, Type::Primitive(Primitive::Int))) => {
                Type::Primitive(Primitive::Int)
            }
            HirNode::HirExpr(HirExpr::Literal(_, Type::Primitive(Primitive::Str))) => {
                Type::Primitive(Primitive::Str)
            }
            HirNode::HirExpr(HirExpr::Literal(_, Type::Primitive(Primitive::Bool))) => {
                Type::Primitive(Primitive::Bool)
            }
            HirNode::HirExpr(HirExpr::Binary(_, _, _, Type::Primitive(Primitive::Int))) => {
                Type::Primitive(Primitive::Int)
            }
            HirNode::HirExpr(HirExpr::Binary(_, _, _, Type::Primitive(Primitive::Bool))) => {
                Type::Primitive(Primitive::Bool)
            }
            HirNode::HirStmt(HirStmt::Function(_, _, _, Type::Primitive(Primitive::Int))) => {
                Type::Primitive(Primitive::Int)
            }
            HirNode::HirStmt(HirStmt::Function(_, _, _, Type::Primitive(Primitive::Int))) => {
                Type::Primitive(Primitive::Int)
            }
            // Not possible. Consider let bar: int = foo(); where foo returns an int.
            HirNode::HirExpr(HirExpr::Call(HirFunction(builtin, _expr))) => {
                Type::Primitive(Primitive::Unit)
            }
            HirNode::HirExpr(HirExpr::Var(v, Type::Primitive(Primitive::Int))) => {
                Type::Primitive(Primitive::Int)
            }
            HirNode::HirExpr(HirExpr::Var(v, Type::Primitive(Primitive::Bool))) => {
                Type::Primitive(Primitive::Bool)
            }
            HirNode::HirStmt(HirStmt::Return(_, Type::Primitive(Primitive::Int))) => {
                Type::Primitive(Primitive::Int)
            }
            HirNode::HirStmt(HirStmt::Return(_, Type::Primitive(Primitive::Bool))) => {
                Type::Primitive(Primitive::Bool)
            }
            HirNode::HirStmt(HirStmt::Return(_, Type::Primitive(Primitive::Str))) => {
                Type::Primitive(Primitive::Str)
            }
            _ => unimplemented!("{:?}", hir),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Literal {
    Bool(bool),
    Int(i32),
    String(String),
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Return(Expr),
    Assign(String, Expr, TypeAnnotation),
    ReAssign(String, Expr),
    IfStatement(Expr, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    IfElse(Expr, Vec<Stmt>, Vec<Stmt>),
    Func(String, Vec<String>, Block, TypeAnnotation),
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

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum TypeAnnotation {
    Int,
    Bool,
    Str,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn expression_to_type() {
//         let ty1: Type =
//             HirExpr::Literal(Literal::Bool(true), Type::Primitive(Primitive::Bool)).into();
//         let expected = Type::Primitive(Primitive::Bool);
//         assert_eq!(ty1, expected);
//     }
//     #[test]
//     fn type_is_int() {
//         let ty1: Type = HirExpr::Literal(Literal::Int(80), Type::Primitive(Primitive::Int)).into();
//         let expected = Type::Primitive(Primitive::Int);
//         assert_eq!(ty1, expected);
//     }
// }
