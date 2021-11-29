pub mod ast;
use ast::*;

use parser::ProgParser;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::pedantic)]
    #[allow(dead_code)]
    parser
);

fn eval(program: &Prog) -> Result<(), String> {
    match program {
        Prog::Body { stmts } => {
            for stmt in stmts {
                if let Ok(v) = stmt_eval(stmt) {
                    println!("{:?}", v);
                }
            }
        }
    }

    Ok(())
}

fn stmt_eval(expr: &Stmt) -> Result<Value, String> {
    match expr {
        Stmt::Expr(x) => expr_eval(x),
        _ => unimplemented!(),
    }
}

fn expr_eval(expr: &Expr) -> Result<Value, String> {
    match expr {
        Expr::Int(v) => Ok(Value::Int(*v)),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Str(s) => Ok(Value::Str(*s)),
        _ => unimplemented!(),
    }
}

fn main() {
    let source = parser::ProgParser::new().parse("true;").unwrap();
    println!("{:?}", eval(&source));
}
