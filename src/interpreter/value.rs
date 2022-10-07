use crate::ast::*;
use crate::error::Error;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Str(String),
    List(Vec<Value>),
    BuiltinFunction(fn(Vec<Value>) -> Result<Value, Error>),
    Function(Vec<String>, Vec<HirNode>),
    // Function(Vec<String>, Vec<Stmt>),
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
                    write!(f, "{} ", el)?;
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
