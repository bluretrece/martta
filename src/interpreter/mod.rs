use crate::error::*;
use crate::*;
use std::cell::RefCell;
use std::rc::Rc;
pub mod value;
pub use self::value::Value;

pub struct Interpreter {
    pub env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new(env: Rc<RefCell<Environment>>) -> Self {
        Self { env }
    }

    pub fn eval(&mut self, program: &Prog) -> Result<Value, Error> {
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

    pub fn eval_block(
        &mut self,
        stmts: Vec<HirExpr>,
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, Error> {
        let mut value: Value = Value::Nil;
        let previous = self.env.clone();
        let steps = || -> Result<Value, Error> {
            self.env = env;
            for statement in stmts {
                // self.stmt_eval?
                value = self.expr_eval(&statement)?
            }
            Ok(value)
        };
        let result = steps();
        self.env = previous;

        result
    }

    pub fn execute_block(
        &mut self,
        stmts: Vec<Stmt>,
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, Error> {
        let mut v = Value::Nil;
        let previous = Rc::clone(&self.env);
        self.env = env;
        for stmt in stmts {
            v = self.stmt_eval(&stmt)?
        }

        self.env = previous;
        Ok(v)
    }

    pub fn stmt_eval(&mut self, expr: &Stmt) -> Result<Value, Error> {
        match expr {
            // Stmt::Expr(e) => self.expr_eval(e),
            _ => todo!(),
            // Stmt::Return(e) => {
            //     let value = match self.expr_eval(e) {
            //         Ok(v) => v,
            //         Err(e) => return Err(e),
            //     };
            //     Ok(value)
            // }

            // Stmt::Func(name, args, stmts) => {
            //     let v = Value::Function(args.to_vec(), stmts.to_vec());

            //     match self.env.borrow_mut().define(name.clone(), v) {
            //         Ok(_) => Ok(Value::Nil),
            //         Err(e) => Err(Error::InvalidOperation(e)),
            //     }
            // }

            // Stmt::Class(identifier, _body) => {
            //     self.env
            //         .borrow_mut()
            //         .define(identifier.clone(), Value::Str(String::from(identifier)))?;
            //     Ok(Value::Nil)
            // }

            // Stmt::Assign(name, rhs) => match self.expr_eval(rhs) {
            //     Ok(v) => {
            //         self.env.borrow_mut().define(name.to_string(), v)?;
            //         Ok(Value::Nil)
            //     }
            //     Err(e) => Err(e),
            // },

            // Stmt::ReAssign(ref var, rhs) => match self.expr_eval(rhs) {
            //     Ok(value) => {
            //         if let Some(x) = self.env.borrow_mut().vals.get_mut(var) {
            //             *x = value;
            //         }
            //         Ok(Value::Nil)
            //     }
            //     Err(e) => Err(e),
            // },

            // Stmt::While(cond, stmts) => loop {
            //     let conditional = match self.expr_eval(cond) {
            //         Ok(b) => match b {
            //             Value::Bool(true) => true,
            //             Value::Bool(false) => false,
            //             _ => unimplemented!(),
            //         },
            //         Err(_) => unimplemented!(),
            //     };

            //     if !conditional {
            //         break Ok(Value::Nil);
            //     }
            //     if let Err(e) = self.eval_block(stmts.to_vec(), self.env.clone()) {
            //         return Err(e);
            //     }
            // },

            // Stmt::IfStatement(cond, stmts) => match self.expr_eval(cond) {
            //     Ok(b) => match b {
            //         Value::Bool(true) => self.eval_block(stmts.to_vec(), self.env.clone()),
            //         Value::Bool(false) => Ok(Value::Nil),
            //         _ => unreachable!(),
            //     },
            //     Err(_) => Err(Error::InvalidOperation(
            //         "Expression must be boolean".to_string(),
            //     )),
            // },
            // Stmt::IfElse(cond, stmts, estmt) => match self.expr_eval(cond) {
            //     Ok(b) => match b {
            //         Value::Bool(true) => self.eval_block(stmts.to_vec(), self.env.clone()),
            //         Value::Bool(false) => self.eval_block(estmt.to_vec(), self.env.clone()),
            //         _ => unreachable!(),
            //     },
            //     Err(_) => Err(Error::InvalidOperation(
            //         "Expression must be boolean".to_string(),
            //     )),
            // },
        }
    }
    pub fn expr_evals(&mut self, exprs: &Vec<HirExpr>) -> Result<Vec<Value>, Error> {
        let mut vals: Vec<Value> = Vec::new();

        for expr in exprs {
            match self.expr_eval(expr) {
                Ok(v) => vals.push(v),
                Err(e) => return Err(Error::InvalidOperation(e.to_string())),
            }
        }

        Ok(vals)
    }

    pub fn expr_eval(&mut self, expr: &HirExpr) -> Result<Value, Error> {
        match expr {
            HirExpr::Binary(lhs, op, rhs, _) => {
                let lhs = self.expr_eval(lhs)?;
                let rhs = self.expr_eval(rhs)?;

                match op {
                    Operator::Add => Ok(lhs + rhs),
                    Operator::Sub => Ok(lhs - rhs),
                    Operator::Div => Ok(lhs / rhs),
                    Operator::GreaterThan => Ok(Value::Bool(lhs > rhs)),
                    Operator::LessThan => Ok(Value::Bool(lhs < rhs)),
                    Operator::LessOrEqual => Ok(Value::Bool(lhs <= rhs)),
                    Operator::EqTo => Ok(Value::Bool(lhs == rhs)),
                    Operator::Or => match (rhs, lhs) {
                        (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),
                        (_, _) => Err(Error::InvalidOperation(
                            "Only boolean types allowed in Or operations".to_string(),
                        )),
                    },
                    Operator::And => match (rhs, lhs) {
                        (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
                        (_, _) => Err(Error::InvalidOperation(
                            "Only boolean types allowed in Or operations".to_string(),
                        )),
                    },
                    _ => unreachable!(),
                }
            }
            HirExpr::Literal(Literal::Int(l), _) => Ok(Value::Int(*l)),
            HirExpr::Literal(Literal::Bool(b), _) => Ok(Value::Bool(*b)),
            HirExpr::IfElse(cond, stmts, estmt, _) => match self.expr_eval(cond) {
                Ok(b) => match b {
                    Value::Bool(true) => self.eval_block(stmts.to_vec(), self.env.clone()),
                    Value::Bool(false) => self.eval_block(estmt.to_vec(), self.env.clone()),
                    _ => unreachable!(),
                },
                Err(_) => Err(Error::InvalidOperation(
                    "Expression must be boolean".to_string(),
                )),
            },
            HirExpr::Assign(name, rhs, _) => match self.expr_eval(rhs) {
                Ok(v) => {
                    self.env.borrow_mut().define(name.to_string(), v)?;
                    Ok(Value::Nil)
                }
                Err(e) => Err(e),
            },
            HirExpr::Nothing => Ok(Value::Nil),
            HirExpr::Var(name) => match self.env.borrow_mut().get_var(name.to_string()) {
                Some(v) => Ok(v),
                None => Err(Error::InvalidOperation(format!(
                    "'{}' is not defined",
                    name
                ))),
            },
            _ => unimplemented!(),
            // Expr::Str(s) => Ok(Value::Str(s.to_string())),
            // Expr::List(list) => {
            //     let values = match self.expr_evals(list) {
            //         Ok(v) => v,
            //         Err(e) => return Err(e),
            //     };

            //     Ok(Value::List(values))
            // }
            // Expr::Function(args, stmts) => {
            //     let f = Value::Function(args.to_vec(), stmts.to_vec());
            //     Ok(f)
            // }
            // Expr::Call(Call::Class(Class { identifier: name })) => {
            //     match self.env.borrow_mut().get_var(name.to_string()) {
            //         Some(v) => Ok(v),
            //         None => Err(Error::InvalidOperation(format!(
            //             "'{}' is not defined",
            //             name
            //         ))),
            //     }
            // }
            // Expr::Call(Call::Function(Function {
            //     func: function,
            //     args,
            // })) => {
            //     let mut vals = Vec::new();

            //     for arg in args {
            //         match self.expr_eval(arg) {
            //             Ok(v) => vals.push(v),
            //             Err(e) => return Err(e),
            //         }
            //     }

            //     let function_defined = match self.env.borrow_mut().get_var(function.to_string()) {
            //         Some(v) => v,
            //         None => {
            //             return Err(Error::InvalidOperation(format!(
            //                 "Function '{}' is not defined",
            //                 &function
            //             )))
            //         }
            //     };

            //     match function_defined {
            //         Value::BuiltinFunction(f) => f(vals),
            //         Value::Function(params, stmts) => {
            //             let environment =
            //                 Rc::new(RefCell::new(Environment::with_ref(self.env.clone())));
            //             for (param, argument) in params.iter().zip(vals.iter()) {
            //                 environment
            //                     .borrow_mut()
            //                     .define(param.clone(), argument.clone())?;
            //             }
            //             self.eval_block(stmts, environment)
            //         }
            //         _ => Err(Error::InvalidOperation(format!(
            //             "'{}' isn't a function",
            //             function
            //         ))),
            //     }
            // }
        }
    }
}
