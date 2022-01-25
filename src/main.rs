pub mod environment;
use environment::*;
pub mod ast;
use ast::*;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::pedantic)]
    #[allow(dead_code)]
    parser
);

fn eval(program: &Prog, env: &mut Environtment) -> Result<(), String> {
    match program {
        Prog::Body { stmts } => {
            for stmt in stmts {
                if let Err(err) = stmt_eval(stmt, env) {
                    return Err(err);
                }
            }
        }
    }
    Ok(())
}

fn eval_block(stmts: Vec<Stmt>, env: &mut Environtment) -> Result<Value, String> {
    let mut value = Value::Nil;
    for stmt in stmts {
        value = stmt_eval(&stmt, env)?;
    }
    Ok(value)
}

fn stmt_eval(expr: &Stmt, env: &mut Environtment) -> Result<Value, String> {
    match expr {
        Stmt::Expr(x) => expr_eval(x, env),
        Stmt::Assign(name, rhs) => match expr_eval(rhs, env) {
            Ok(v) => env.define(name.to_string(), v),
            Err(e) => Err(e),
        },
        Stmt::IfStatement(cond, stmts) => match expr_eval(cond, env) {
            Ok(b) => match b {
                Value::Bool(true) => eval_block(stmts.to_vec(), env),
                Value::Bool(false) => Ok(Value::Nil),
                _ => unreachable!(),
            },
            Err(_) => Err("Expression must be boolean".to_string()),
        },
        Stmt::IfElse(cond, stmts, estmt) => match expr_eval(cond, env) {
            Ok(b) => match b {
                Value::Bool(true) => eval_block(stmts.to_vec(), env),
                Value::Bool(false) => eval_block(estmt.to_vec(), env),
                _ => unreachable!(),
            },
            Err(_) => Err("Expression must be boolean".to_string()),
        },
    }
}

fn expr_eval(expr: &Expr, env: &mut Environtment) -> Result<Value, String> {
    match expr {
        Expr::Binary(lhs, op, rhs) => {
            let lhs = expr_eval(lhs, env)?;
            let rhs = expr_eval(rhs, env)?;

            match op {
                Operator::Add => Ok(lhs + rhs),
                Operator::Sub => Ok(lhs - rhs),
                Operator::Div => Ok(lhs / rhs),
                Operator::GreaterThan => Ok(Value::Bool(lhs > rhs)),
                Operator::LessThan => Ok(Value::Bool(lhs < rhs)),
                Operator::EqTo => Ok(Value::Bool(lhs == rhs)),
                Operator::Or => {
                    if let Value::Bool(a) = lhs {
                        if let Value::Bool(b) = rhs {
                            Ok(Value::Bool(a || b))
                        } else {
                            Err("Second operand must be boolean".to_string())
                        }
                    } else {
                        Err("Only boolean types allowed in Or operations".to_string())
                    }
                }
                Operator::And => {
                    if let Value::Bool(a) = lhs {
                        if let Value::Bool(b) = rhs {
                            Ok(Value::Bool(a && b))
                        } else {
                            Err("Second operand must be boolean".to_string())
                        }
                    } else {
                        Err("Only boolean types allowed in Or operations".to_string())
                    }
                }
            }
        }
        Expr::Int(v) => Ok(Value::Int(*v)),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Str(s) => Ok(Value::Str(s.to_string())),
        Expr::Var(name) => match env.get_var(name.to_string()) {
            Some(v) => Ok(v),
            None => Err(format!("'{}' is not defined", name)),
        },
        Expr::Call(Call {
            func: function,
            args,
        }) => {
            let mut vals = Vec::new();

            for arg in args {
                match expr_eval(arg, env) {
                    Ok(v) => vals.push(v),
                    Err(e) => return Err(e),
                }
            }

            let function_defined = match env.get_var(function.to_string()) {
                Some(v) => v,
                None => return Err(format!("Function '{}' is not defined", &function)),
            };

            if let Value::Function(f) = function_defined {
                f(vals)
            } else {
                Err(format!("'{}' isn't a function", function))
            }
        }
    }
}

pub fn std_print(vals: Vec<Value>) -> Result<Value, String> {
    println!("{:?}", vals);

    Ok(Value::Nil)
}

fn main() {
    let input = std::fs::read_to_string("hello.mrt").expect("Cannot read source file");
    let mut env = Environtment::default();
    env.define("println".to_string(), Value::Function(std_print))
        .unwrap();
    let source = parser::ProgParser::new().parse(&input).unwrap();
    println!("{:?}", eval(&source, &mut env));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_parsing() {
        let mut env = HashMap::new();
        let int_literal = "197;";
        let source = parser::ProgParser::new().parse(int_literal).unwrap();
        let res = eval(&source, &env);

        assert_eq!(res, Ok(()));
    }
}
