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
    // pub enclosing: Option<Rc<RefCell<Environment>>>,
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

    pub fn typecheck(&mut self, program: &Prog) -> Result<HirNode, Error> {
        let mut value = HirNode::HirExpr(HirExpr::Nothing);
        match program {
            Prog::Body(stmts) => {
                for stmt in stmts {
                    value = self.stmt_eval(stmt)?;
                }
            }
        }
        Ok(value)
    }

    pub fn eval_block(&mut self, stmts: Vec<Stmt>) -> Result<HirNode, Error> {
        let mut value: HirNode = HirNode::HirExpr(HirExpr::Nothing);
        let steps = || -> Result<HirNode, Error> {
            for statement in stmts {
                value = self.stmt_eval(&statement)?
            }
            Ok(value)
        };
        let result = steps();

        result
    }

    pub fn annotation_type(&self, annotation: TypeAnnotation) -> Type {
        match annotation {
            TypeAnnotation::Int => Type::Primitive(Primitive::Int),
            TypeAnnotation::Bool => Type::Primitive(Primitive::Bool),
            TypeAnnotation::Str => Type::Primitive(Primitive::Str),
            _ => unimplemented!("String type annotations are not yet supported"),
        }
    }

    pub fn stmt_eval(&mut self, expr: &Stmt) -> Result<HirNode, Error> {
        match expr {
            Stmt::Expr(x) => self.typecheck_expr(x),
            Stmt::Func(name, args, stmts, annotation) => {
                let return_type = self.annotation_type(annotation.clone());
                let body_type: Type = self.eval_block(stmts.to_vec())?.into();
                let block_value = self.eval_block(stmts.to_vec())?;

                assert_eq!(
                    return_type, body_type,
                    "Types mismatch. Expected {:?} as return type, but got {:?} instead.",
                    return_type, body_type
                );

                println!("Context: {:?} ", self.ctx.values);

                Ok(HirNode::HirStmt(HirStmt::Function(
                    name.to_owned(),
                    args.to_vec(),
                    vec![block_value],
                    return_type,
                )))
            }
            Stmt::Assign(name, rhs, annotation) => {
                let type_: Type = self.typecheck_expr(rhs)?.into();
                let expr_ = self.typecheck_expr(rhs)?;
                let expected = self.annotation_type(annotation.clone());

                self.ctx.define(name.to_string(), type_.clone());

                println!("Context state: {:?}", self.ctx.values);

                assert_eq!(
                    type_, expected,
                    "Types mismatch. Expected: {:?} but got {:?}",
                    expected, type_
                );

                Ok(HirNode::HirStmt(HirStmt::Assign(
                    String::from(name),
                    Box::new(expr_),
                    type_,
                )))
            }
            Stmt::Return(e) => {
                let expr = self.typecheck_expr(e)?;
                let type_: Type = self.typecheck_expr(e)?.into();

                Ok(HirNode::HirStmt(HirStmt::Return(Box::new(expr), type_)))
            }
            Stmt::IfStatement(cond, stmts) => {
                let type_: Type = self.typecheck_expr(cond)?.into();
                let cond = self.typecheck_expr(cond)?;
                let stmts = self.eval_block(stmts.clone())?;

                assert_eq!(type_, Type::Primitive(Primitive::Bool));

                Ok(HirNode::HirStmt(HirStmt::IfStatement(
                    Box::new(cond.clone()),
                    vec![stmts],
                    type_,
                )))
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

                Ok(HirNode::HirStmt(HirStmt::IfElse(
                    Box::new(h1),
                    vec![h2],
                    vec![h3],
                    ty2,
                )))
            }
            _ => Err(Error::TypeError(
                "The type system does not support other expressions yet".into(),
            )),
        }
    }

    pub fn typecheck_expr(&mut self, expr: &Expr) -> Result<HirNode, Error> {
        match expr {
            Expr::Int(literal) => Ok(HirNode::HirExpr(HirExpr::Literal(
                Literal::Int(*literal),
                Type::Primitive(Primitive::Int),
            ))),
            Expr::Bool(literal) => Ok(HirNode::HirExpr(HirExpr::Literal(
                Literal::Bool(*literal),
                Type::Primitive(Primitive::Bool),
            ))),
            Expr::Var(v) => {
                let type_ = match self.ctx.lookup(v.to_string()) {
                    Some(t) => t,
                    None => Type::Primitive(Primitive::Int),
                };

                Ok(HirNode::HirExpr(HirExpr::Var(v.to_string(), type_)))
            }
            Expr::Str(s) => Ok(HirNode::HirExpr(HirExpr::Literal(
                Literal::String(s.to_string()),
                Type::Primitive(Primitive::Str),
            ))),
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

                Ok(HirNode::HirExpr(HirExpr::Binary(
                    Box::new(lhs_.clone()),
                    op.clone(),
                    Box::new(rhs_.clone()),
                    type_,
                )))
            }
            Expr::Call(Call::Function(Function {
                func: function,
                args,
            })) => {
                let mut vals = Vec::new();

                for arg in args {
                    match self.typecheck_expr(arg) {
                        Ok(v) => vals.push(v),
                        Err(e) => return Err(e),
                    }
                }
                // TODO: Check if arguments number matches
                Ok(HirNode::HirExpr(HirExpr::Call(HirFunction(
                    function.to_string(),
                    vals,
                ))))
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn hir_gets_parsed() {
//         let source: Prog = parser::ProgParser::new().parse("34").unwrap();
//         let mut type_checker = Typechecker::default();
//         let tc = type_checker.typecheck(&source).unwrap();
//         assert_eq!(
//             tc,
//             HirExpr::Literal(Literal::Int(34), Type::Primitive(Primitive::Int))
//         )
//     }

//     #[test]
//     fn type_is_int() {
//         let source: Prog = parser::ProgParser::new().parse("34").unwrap();
//         let mut type_checker = Typechecker::default();
//         let hir: Type = type_checker.typecheck(&source).unwrap().into();
//         assert_eq!(hir, Type::Primitive(Primitive::Int));
//     }

//     #[test]
//     fn binary_operation_is_int() {
//         let source: Prog = parser::ProgParser::new().parse("3 + 3").unwrap();
//         let mut type_checker = Typechecker::default();
//         let hir: Type = type_checker.typecheck(&source).unwrap().into();
//         assert_eq!(hir, Type::Primitive(Primitive::Int));
//     }

//     #[test]
//     #[should_panic]
//     fn types_mismatch() {
//         let source: Prog = parser::ProgParser::new().parse("3 + true").unwrap();
//         let mut type_checker = Typechecker::default();
//         let hir: Type = type_checker.typecheck(&source).unwrap().into();
//     }

//     #[test]
//     fn sub_is_int() {
//         let source: Prog = parser::ProgParser::new().parse("184 - 42").unwrap();
//         let mut type_checker = Typechecker::default();
//         let hir: Type = type_checker.typecheck(&source).unwrap().into();
//         assert_eq!(hir, Type::Primitive(Primitive::Int));
//     }
// }
