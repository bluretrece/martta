use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    pub env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new(env: Rc<RefCell<Environment>>) -> Self {
        Self { env }
    }

    pub fn eval(&mut self, program: &Prog) -> Result<Value, String> {
        let mut value = Value::Nil;
        match program {
            Prog::Body(stmts) => {
                for stmt in stmts {
                    value = self.stmt_eval(stmt)?;
                }
            }
        }
        Ok(value)
    }

    pub fn eval_block(&mut self, stmts: Vec<Stmt>) -> Result<Value, String> {
        let mut value = Value::Nil;
        for stmt in stmts {
            value = self.stmt_eval(&stmt)?;
        }
        Ok(value)
    }

    pub fn stmt_eval(&mut self, expr: &Stmt) -> Result<Value, String> {
        match expr {
            Stmt::Expr(x) => self.expr_eval(x),
            Stmt::Return(e) => {
                let value = match self.expr_eval(e) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
                Ok(value)
            }

            Stmt::Func(name, args, stmts) => {
                let v = Value::Function(args.to_vec(), stmts.to_vec());

                match self.env.borrow_mut().define(name.clone(), v) {
                    Ok(_) => Ok(Value::Nil),
                    Err(e) => return Err(e),
                }
            }

            Stmt::Assign(name, rhs) => match self.expr_eval(rhs) {
                Ok(v) => {
                    self.env.borrow_mut().define(name.to_string(), v);
                    Ok(Value::Nil)
                }
                Err(e) => Err(e),
            },

            Stmt::ReAssign(_lhs, _op, _rhs) => {
                unimplemented!();
            }

            Stmt::While(cond, stmts) => loop {
                let conditional = match self.expr_eval(cond) {
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
                if let Err(e) = self.eval_block(stmts.to_vec()) {
                    return Err(e);
                }
            },

            Stmt::IfStatement(cond, stmts) => match self.expr_eval(cond) {
                Ok(b) => match b {
                    Value::Bool(true) => self.eval_block(stmts.to_vec()),
                    Value::Bool(false) => Ok(Value::Nil),
                    _ => unreachable!(),
                },
                Err(_) => Err("Expression must be boolean".to_string()),
            },
            Stmt::IfElse(cond, stmts, estmt) => match self.expr_eval(cond) {
                Ok(b) => match b {
                    Value::Bool(true) => self.eval_block(stmts.to_vec()),
                    Value::Bool(false) => self.eval_block(estmt.to_vec()),
                    _ => unreachable!(),
                },
                Err(_) => Err("Expression must be boolean".to_string()),
            },
        }
    }
    pub fn expr_evals(&mut self, exprs: &Vec<Expr>) -> Result<Vec<Value>, String> {
        let mut vals: Vec<Value> = Vec::new();

        for expr in exprs {
            match self.expr_eval(expr) {
                Ok(v) => vals.push(v),
                Err(e) => return Err(e),
            }
        }

        Ok(vals)
    }

    pub fn expr_eval(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Binary(lhs, op, rhs) => {
                let lhs = self.expr_eval(lhs)?;
                let rhs = self.expr_eval(rhs)?;

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
            Expr::Var(name) => match self.env.borrow_mut().get_var(name.to_string()) {
                Some(v) => Ok(v),
                None => Err(format!("'{}' is not defined", name)),
            },
            Expr::List(list) => {
                let values = match self.expr_evals(list) {
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
                    match self.expr_eval(arg) {
                        Ok(v) => vals.push(v),
                        Err(e) => return Err(e),
                    }
                }

                // Is it the function defined?
                let function_defined = match self.env.borrow_mut().get_var(function.to_string()) {
                    Some(v) => v,
                    None => return Err(format!("Function '{}' is not defined", &function)),
                };

                // Is it builtin function or user defined function?
                if let Value::BuiltinFunction(f) = function_defined {
                    f(vals)
                } else if let Value::Function(args, stmts) = function_defined {
                    self.eval_block(stmts)
                } else {
                    Err(format!("'{}' isn't a function", function))
                }
            }
        }
    }
}
