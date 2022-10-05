use crate::ast::*;
use crate::error::*;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::pedantic)]
    #[allow(dead_code)]
    parser
);

#[derive(Debug, Default)]
pub struct Typechecker;
impl Typechecker {
    pub fn typecheck(&mut self, program: &Prog) -> Result<HirExpr, Error> {
        let mut value = HirExpr::Nothing;
        match program {
            Prog::Body(stmts) => {
                for stmt in stmts {
                    value = self.stmt_eval(stmt)?;
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

    pub fn stmt_eval(&mut self, expr: &Stmt) -> Result<HirExpr, Error> {
        match expr {
            Stmt::Expr(x) => self.typecheck_expr(x),
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
            Expr::Binary(lhs, op, rhs) => {
                let lhs_ = self.typecheck_expr(lhs)?;
                let rhs_ = self.typecheck_expr(rhs)?;

                let type_ = match op {
                    Operator::Add | Operator::Sub | Operator::Div => {
                        let ty = self.unify(&lhs_.clone().into(), &rhs_.clone().into())?;

                        ty
                    }
                    _ => unimplemented!("Unimplemented type operator"),
                };

                Ok(HirExpr::Binary(
                    Box::new(lhs_.clone()),
                    op.clone(),
                    Box::new(rhs_.clone()),
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
                Err(Error::TypeError("Types mismatch".into()))
            }
            (_, _) => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hir_gets_parsed() {
        let source: Prog = parser::ProgParser::new().parse("34").unwrap();
        let mut type_checker = Typechecker::default();
        let tc = type_checker.typecheck(&source).unwrap();
        assert_eq!(
            tc,
            HirExpr::Literal(Literal::Int(34), Type::Primitive(Primitive::Int))
        )
    }

    #[test]
    fn type_is_int() {
        let source: Prog = parser::ProgParser::new().parse("34").unwrap();
        let mut type_checker = Typechecker::default();
        let hir: Type = type_checker.typecheck(&source).unwrap().into();
        assert_eq!(hir, Type::Primitive(Primitive::Int));
    }

    #[test]
    fn binary_operation_is_int() {
        let source: Prog = parser::ProgParser::new().parse("3 + 3").unwrap();
        let mut type_checker = Typechecker::default();
        let hir: Type = type_checker.typecheck(&source).unwrap().into();
        assert_eq!(hir, Type::Primitive(Primitive::Int));
    }

    #[test]
    #[should_panic]
    fn types_mismatch() {
        let source: Prog = parser::ProgParser::new().parse("3 + true").unwrap();
        let mut type_checker = Typechecker::default();
        let hir: Type = type_checker.typecheck(&source).unwrap().into();
    }

    #[test]
    fn sub_is_int() {
        let source: Prog = parser::ProgParser::new().parse("184 - 42").unwrap();
        let mut type_checker = Typechecker::default();
        let hir: Type = type_checker.typecheck(&source).unwrap().into();
        assert_eq!(hir, Type::Primitive(Primitive::Int));
    }
}
