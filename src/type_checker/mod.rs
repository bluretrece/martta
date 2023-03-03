use crate::ast::*;
use crate::error::*;
use std::collections::HashMap;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::pedantic)]
    #[allow(dead_code)]
    parser
);

#[derive(Debug, Default, Clone)]
pub struct Context {
    pub values: HashMap<String, Type>,
}

impl Context {
    pub fn define(&mut self, name: String, type_: Type) -> Result<(), String> {
        self.values.insert(name, type_);
        Ok(())
    }

    pub fn lookup(&mut self, name: String) -> Option<Type> {
        if let Some(value) = self.values.get(&name).cloned() {
            return Some(value);
        }
        None
    }
}

#[derive(Debug, Default)]
pub struct Typechecker {
    ctx: Context,
}

impl Typechecker {
    pub fn new() -> Self {
        Self {
            ctx: Context::default(),
        }
    }

    pub fn typecheck(&mut self, program: &Prog) -> Result<Vec<HirExpr>, Error> {
        let mut value = Vec::new();
        match program {
            Prog::Body(stmts) => {
                for stmt in stmts {
                    value.push(self.stmt_eval(stmt)?);
                }
            }
        }
        Ok(value)
    }

    pub fn eval_block(&mut self, stmts: Vec<Stmt>) -> Result<HirExpr, Error> {
        let mut value: HirExpr = HirExpr::Nothing;
        let steps = || -> Result<HirExpr, Error> {
            for statement in stmts {
                value = self.stmt_eval(&statement)?
            }
            Ok(value)
        };
        let result = steps();

        result
    }

    pub fn ascription_type(&self, ascription: Ascription) -> Type {
        match ascription {
            Ascription::Int => Type::Primitive(Primitive::Int),
            Ascription::Bool => Type::Primitive(Primitive::Bool),
            Ascription::Str => Type::Primitive(Primitive::Str),
            Ascription::List(a) => match *a {
                Ascription::Int => Type::Primitive(Primitive::List(Box::new(Primitive::Int))),
                Ascription::Bool => Type::Primitive(Primitive::List(Box::new(Primitive::Bool))),
                Ascription::Str => Type::Primitive(Primitive::List(Box::new(Primitive::Str))),
                // Recursive primitive not allowed for now
                _ => unreachable!(),
            },
        }
    }

    pub fn stmt_eval(&mut self, expr: &Stmt) -> Result<HirExpr, Error> {
        match expr {
            Stmt::Expr(x) => self.typecheck_expr(x),
            Stmt::Func(name, args, stmts, ascription) => {
                let return_type: Type = self.ascription_type(ascription.clone());
                let body_type: Type = self.eval_block(stmts.to_vec())?.into();
                let block_value = self.eval_block(stmts.to_vec())?;

                self.ctx
                    .define(name.to_string(), return_type.clone())
                    .unwrap_or_else(|_| ());

                assert_eq!(
                    return_type, body_type,
                    "Types mismatch. Expected {:?} as return type, but got {:?} instead.",
                    return_type, body_type
                );

                Ok(HirExpr::Function(
                    name.to_owned(),
                    args.to_vec(),
                    vec![block_value],
                    return_type,
                ))
            }
            Stmt::Assign(name, rhs, annotation) => {
                let type_: Type = self.typecheck_expr(rhs)?.into();
                let expr_ = self.typecheck_expr(rhs)?;
                let expected = self.ascription_type(annotation.clone());

                self.ctx.define(name.to_string(), type_.clone())?;

                assert_eq!(
                    type_, expected,
                    "Types mismatch. Expected: {:?} but got {:?}",
                    expected, type_
                );

                Ok(HirExpr::Assign(String::from(name), Box::new(expr_), type_))
            }
            Stmt::Return(e) => {
                let expr = self.typecheck_expr(e)?;
                let type_: Type = self.typecheck_expr(e)?.into();

                Ok(HirExpr::Return(Box::new(expr), type_))
            }
            Stmt::IfStatement(cond, stmts) => {
                let type_: Type = self.typecheck_expr(cond)?.into();
                let cond = self.typecheck_expr(cond)?;
                let stmts = self.eval_block(stmts.clone())?;

                assert_eq!(type_, Type::Primitive(Primitive::Bool));

                Ok(HirExpr::IfStatement(
                    Box::new(cond.clone()),
                    vec![stmts],
                    type_,
                ))
            }
            Stmt::IfElse(t1, t2, t3) => {
                let h1 = self.typecheck_expr(t1)?;
                let ty1: Type = self.typecheck_expr(t1)?.into();

                assert_eq!(ty1, Type::Primitive(Primitive::Bool));

                let ty2: Type = self.eval_block(t2.to_vec())?.into();
                let ty3: Type = self.eval_block(t3.to_vec())?.into();
                let h2 = self.eval_block(t2.clone())?;
                let h3 = self.eval_block(t3.clone())?;

                assert_eq!(ty2, ty3, "Types must match: {:?} /= {:?}", ty2, ty3);

                Ok(HirExpr::IfElse(Box::new(h1), vec![h2], vec![h3], ty2))
            }
            _ => Err(Error::TypeError(
                "The type system does not support other expressions yet".into(),
            )),
        }
    }

    pub fn typecheck_expr(&mut self, expr: &Expr) -> Result<HirExpr, Error> {
        match expr {
            Expr::Int(literal) => Ok(HirExpr::Literal(
                Literal::Int(*literal),
                Type::Primitive(Primitive::Int),
            )),
            Expr::Bool(literal) => Ok(HirExpr::Literal(
                Literal::Bool(*literal),
                Type::Primitive(Primitive::Bool),
            )),
            Expr::Var(v) => {
                let type_ = match self.ctx.lookup(v.to_string()) {
                    Some(t) => t,
                    None => Type::Primitive(Primitive::Int),
                };

                Ok(HirExpr::Var(v.to_string(), type_))
            }
            Expr::Str(s) => Ok(HirExpr::Literal(
                Literal::String(s.to_string()),
                Type::Primitive(Primitive::Str),
            )),
            Expr::Function(args, stmts) => {
                let body = self.eval_block(stmts.clone())?;
                let type_: Type = self.eval_block(stmts.to_vec())?.into();

                Ok(HirExpr::Lambda(args.to_vec(), vec![body], type_))
            }
            Expr::List(elements) => {
                let mut typechecked_expressions: Vec<Type> = Vec::new();
                let mut parsed_exprs = Vec::new();

                for el in elements {
                    typechecked_expressions.push(self.typecheck_expr(el)?.into());
                    parsed_exprs.push(self.typecheck_expr(el)?);
                }

                let type_ = || -> Type {
                    let head = typechecked_expressions.first().unwrap();
                    typechecked_expressions
                        .iter()
                        .all(|t| t == head)
                        .then(|| head)
                        .clone()
                        .unwrap()
                        .clone()
                };

                Ok(HirExpr::List(parsed_exprs, type_()))
            }
            Expr::Binary(lhs, op, rhs) => {
                let lhs_ = self.typecheck_expr(lhs)?;
                let rhs_ = self.typecheck_expr(rhs)?;

                let type_ = match op {
                    Operator::Add | Operator::Sub | Operator::Div => {
                        let ty = self.unify(&lhs_.clone().into(), &rhs_.clone().into())?;

                        ty
                    }
                    Operator::EqTo
                    | Operator::Or
                    | Operator::And
                    | Operator::LessThan
                    | Operator::GreaterThan => Type::Primitive(Primitive::Bool),
                    _ => unimplemented!("Unimplemented type operator"),
                };

                Ok(HirExpr::Binary(
                    Box::new(lhs_.clone()),
                    op.clone(),
                    Box::new(rhs_.clone()),
                    type_,
                ))
            }
            Expr::Call(Call::Function(Function {
                func: function,
                args,
            })) => {
                let mut vals = Vec::new();
                let type_: Type = match self.ctx.lookup(function.to_string()) {
                    Some(t) => t,
                    None => Type::Primitive(Primitive::Int),
                };

                for arg in args {
                    match self.typecheck_expr(arg) {
                        Ok(v) => vals.push(v),
                        Err(e) => return Err(e),
                    }
                }
                // TODO: Check if arguments number matches
                Ok(HirExpr::Call(
                    HirFunction(function.to_string(), vals),
                    type_,
                ))
            }
            _ => Err(Error::TypeError(
                "The type system does not support this type yet".into(),
            )),
        }
    }

    pub fn unify(&self, ty1: &Type, ty2: &Type) -> Result<Type, Error> {
        match (ty1, ty2) {
            (Type::Primitive(p1), Type::Primitive(p2)) if p1 == p2 => Ok(ty2.clone()),
            (Type::Primitive(p1), Type::Primitive(p2)) if p1 != p2 => {
                Err(Error::TypeError("Types do not unify".into()))
            }
            (_, _) => unimplemented!(),
        }
    }
}
