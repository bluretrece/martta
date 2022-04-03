use crate::*;
pub fn eval(program: &Prog, env: &mut Environment) -> Result<Value, String> {
    let mut value = Value::Nil;
    match program {
        Prog::Body(stmts) => {
            for stmt in stmts {
                value = stmt_eval(stmt, env)?;
            }
        }
    }
    Ok(value)
}

pub fn eval_block(stmts: Vec<Stmt>, env: &mut Environment) -> Result<Value, String> {
    let mut value = Value::Nil;
    for stmt in stmts {
        value = stmt_eval(&stmt, env)?;
    }
    Ok(value)
}

pub fn stmt_eval(expr: &Stmt, env: &mut Environment) -> Result<Value, String> {
    match expr {
        Stmt::Expr(x) => expr_eval(x, env),
        Stmt::Func(name, args, stmts) => {
            let v = Value::Function(args.to_vec(), stmts.to_vec());

            match env.define(name.clone(), v) {
                Ok(_) => Ok(Value::Nil),
                Err(e) => return Err(e),
            }
            //             if let Err(e) = env.define(name.clone(), v) {
            //                 return Err(e);
            //             } else {
            //                 Ok(Value::Nil)
            //             }
        }
        Stmt::Assign(name, rhs) => match expr_eval(rhs, env) {
            Ok(v) => {
                env.define(name.to_string(), v);
                Ok(Value::Nil)
            }
            Err(e) => Err(e),
        },
        Stmt::ReAssign(_lhs, _op, _rhs) => {
            unimplemented!();
        }
        Stmt::While(cond, stmts) => loop {
            let conditional = match expr_eval(cond, env) {
                Ok(b) => match b {
                    Value::Bool(true) => true,
                    Value::Bool(false) => false,
                    _ => unimplemented!(),
                },
                Err(_) => unimplemented!(),
            };

            if !conditional {
                break Ok(Value::Nil);
            }
            if let Err(e) = eval_block(stmts.to_vec(), env) {
                return Err(e);
            }
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
pub fn expr_evals(exprs: &Vec<Expr>, env: &mut Environment) -> Result<Vec<Value>, String> {
    let mut vals: Vec<Value> = Vec::new();

    for expr in exprs {
        match expr_eval(expr, env) {
            Ok(v) => vals.push(v),
            Err(e) => return Err(e),
        }
    }

    Ok(vals)
}

pub fn expr_eval(expr: &Expr, env: &mut Environment) -> Result<Value, String> {
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
                _ => unreachable!(),
            }
        }
        Expr::Int(v) => Ok(Value::Int(*v)),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Str(s) => Ok(Value::Str(s.to_string())),
        Expr::Var(name) => match env.get_var(name.to_string()) {
            Some(v) => Ok(v),
            None => Err(format!("'{}' is not defined", name)),
        },
        Expr::List(list) => {
            let values = match expr_evals(list, env) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            Ok(Value::List(values))
        }
        Expr::Call(Call {
            func: function,
            args,
        }) => {
            let mut vals = Vec::new();

            // Evaluate the arguments
            for arg in args {
                match expr_eval(arg, env) {
                    Ok(v) => vals.push(v),
                    Err(e) => return Err(e),
                }
            }

            // Is it the function defined?
            let function_defined = match env.get_var(function.to_string()) {
                Some(v) => v,
                None => return Err(format!("Function '{}' is not defined", &function)),
            };

            // Is it builtin function or user defined function?
            if let Value::BuiltinFunction(f) = function_defined {
                f(vals)
            } else if let Value::Function(args, stmts) = function_defined {
                eval_block(stmts, env)
            } else {
                Err(format!("'{}' isn't a function", function))
            }
        }
    }
}
