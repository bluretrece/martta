#[derive(Clone, Debug)]
pub enum Prog {
    Body(Block),
}

pub type Block = Vec<Stmt>;

#[derive(Clone, Debug)]
pub enum Stmt {
    Expr(Expr),
    Assign(String, Expr),
    ReAssign(String, Operator, Expr),
    IfStatement(Expr, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    IfElse(Expr, Vec<Stmt>, Vec<Stmt>),
    Func(String, Vec<Expr>, Vec<Stmt>),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Int(i32),
    Bool(bool),
    Str(String),
    Var(String),
    Binary(Box<Expr>, Operator, Box<Expr>),
    Call(Call),
    List(Vec<Expr>),
}

#[derive(Clone, Debug)]
pub struct Call {
    pub func: String,
    pub args: Vec<Box<Expr>>,
}

#[derive(Clone, Debug)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Or,
    And,
    GreaterThan,
    LessThan,
    EqTo,
    SumTo,
}

#[derive(Clone, Debug, std::cmp::PartialEq, PartialOrd)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Str(String),
    List(Vec<Value>),
    Function(fn(Vec<Value>) -> Result<Value, String>),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(x) => write!(f, "{}", *x),
            Self::Bool(b) => write!(f, "{}", *b),
            Self::List(list) => {
                let mut values = Vec::new();
                for v in list {
                    values.push(v);
                }

                for el in values.iter() {
                    write!(f, "{} ", el);
                }
                Ok(())
            }
            Self::Nil => write!(f, "Nil"),
            Self::Str(s) => write!(f, "{}", *s),
            _ => unimplemented!(),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Self;
    fn sub(self, other: Value) -> Self {
        match self {
            Self::Int(x) => match other {
                Value::Int(y) => Value::Int(x - y),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Self;
    fn div(self, other: Value) -> Self {
        match self {
            Self::Int(x) => match other {
                Value::Int(y) => Value::Int(x / y),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl std::ops::Add for Value {
    type Output = Self;
    fn add(self, other: Value) -> Self {
        match self {
            Self::Int(x) => match other {
                Value::Int(y) => Value::Int(x + y),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
