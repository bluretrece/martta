#[derive(Clone, Debug)]
pub enum Prog {
    Body { stmts: Vec<Stmt> },
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Expr(Expr),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Int(i32),
    Bool(bool),
    Str(String),
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Str(String),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(x) => write!(f, "{}", *x),
            Self::Bool(b) => write!(f, "{}", *b),
            Self::Nil => write!(f, "Nil"),
            Self::Str(s) => write!(f, "{}", *s),
        }
    }
}
