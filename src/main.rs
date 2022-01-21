pub mod ast;
use ast::*;
use std::collections::HashMap;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::pedantic)]
    #[allow(dead_code)]
    parser
);

#[derive(Debug, Default)]
pub struct Environtment {
    pub vals: HashMap<String, Value>,
    // pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environtment {
    pub fn define(&mut self, name: String, value: Value) -> Result<Value, String> {
        self.vals.insert(name, value.clone());
        Ok(value)
    }

    pub fn get_var(&mut self, name: String) -> Option<Value> {
        self.vals.get(&name).map(|value| value.clone())
    }
}

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

fn stmt_eval(expr: &Stmt, env: &mut Environtment) -> Result<Value, String> {
    match expr {
        Stmt::Expr(x) => expr_eval(x, env),
        Stmt::Assign(name, rhs) => match expr_eval(rhs, env) {
            Ok(v) => env.define(name.to_string(), v),
            Err(e) => return Err(e),
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
                Operator::Or => {
                    if let Value::Bool(a) = lhs {
                        if let Value::Bool(b) = rhs {
                            Ok(Value::Bool(a || b))
                        } else {
                            Err(format!("Second operand must be boolean"))
                        }
                    } else {
                        Err(format!("Only boolean types allowed in Or operations"))
                    }
                }
                Operator::And => {
                    if let Value::Bool(a) = lhs {
                        if let Value::Bool(b) = rhs {
                            Ok(Value::Bool(a && b))
                        } else {
                            Err(format!("Second operand must be boolean"))
                        }
                    } else {
                        Err(format!("Only boolean types allowed in Or operations"))
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
    env.define("println".to_string(), Value::Function(std_print));
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
