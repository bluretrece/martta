use std::convert::From;

#[derive(Clone, Debug)]
pub enum Prog {
    Body(Block),
}

pub type Block = Vec<Stmt>;
pub type HirBlock = Vec<HirExpr>;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum HirExpr {
    Literal(Literal, Type),
    Binary(Box<HirExpr>, Operator, Box<HirExpr>, Type),
    Assign(String, Box<HirExpr>, Type),
    Var(String, Type),
    IfElse(Box<HirExpr>, Vec<HirExpr>, Vec<HirExpr>, Type),
    IfStatement(Box<HirExpr>, Vec<HirExpr>, Type),
    Function(String, Vec<String>, HirBlock, Type),
    Lambda(Vec<String>, Vec<HirExpr>, Type),
    Return(Box<HirExpr>, Type),
    Call(HirFunction),
    Nothing,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct HirFunction(pub String, pub Vec<HirExpr>);

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Type {
    Primitive(Primitive),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Primitive {
    Int,
    Bool,
    Str,
    Unit,
}

impl From<HirExpr> for Type {
    fn from(hir: HirExpr) -> Self {
        match hir {
            HirExpr::Literal(_, Type::Primitive(Primitive::Int)) => Type::Primitive(Primitive::Int),
            HirExpr::Literal(_, Type::Primitive(Primitive::Str)) => Type::Primitive(Primitive::Str),
            HirExpr::Literal(_, Type::Primitive(Primitive::Bool)) => {
                Type::Primitive(Primitive::Bool)
            }
            HirExpr::Binary(_, _, _, Type::Primitive(Primitive::Int)) => {
                Type::Primitive(Primitive::Int)
            }
            HirExpr::Binary(_, _, _, Type::Primitive(Primitive::Bool)) => {
                Type::Primitive(Primitive::Bool)
            }
            HirExpr::Function(_, _, _, Type::Primitive(Primitive::Int)) => {
                Type::Primitive(Primitive::Int)
            }
            // CHECK: Not possible. Consider let bar: int = foo(); where foo returns an int.
            HirExpr::Call(HirFunction(_, _expr)) => Type::Primitive(Primitive::Unit),
            HirExpr::Var(_, Type::Primitive(Primitive::Int)) => Type::Primitive(Primitive::Int),
            HirExpr::Var(_, Type::Primitive(Primitive::Bool)) => Type::Primitive(Primitive::Bool),
            HirExpr::Return(_, Type::Primitive(Primitive::Int)) => Type::Primitive(Primitive::Int),
            HirExpr::Return(_, Type::Primitive(Primitive::Bool)) => {
                Type::Primitive(Primitive::Bool)
            }
            HirExpr::Return(_, Type::Primitive(Primitive::Str)) => Type::Primitive(Primitive::Str),
            HirExpr::Lambda(_, _, Type::Primitive(Primitive::Int)) => Type::Primitive(Primitive::Int), // Support for other types..
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
    Assign(String, Expr, Ascription),
    ReAssign(String, Expr),
    IfStatement(Expr, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    IfElse(Expr, Vec<Stmt>, Vec<Stmt>),
    Func(String, Vec<String>, Block, Ascription),
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
pub enum Ascription {
    Int,
    Bool,
    Str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expression_to_type() {
        let ty1: Type =
            HirExpr::Literal(Literal::Bool(true), Type::Primitive(Primitive::Bool)).into();
        let expected = Type::Primitive(Primitive::Bool);
        assert_eq!(ty1, expected);
    }
    #[test]
    fn type_is_int() {
        let ty1: Type = HirExpr::Literal(Literal::Int(80), Type::Primitive(Primitive::Int)).into();
        let expected = Type::Primitive(Primitive::Int);
        assert_eq!(ty1, expected);
    }
}
