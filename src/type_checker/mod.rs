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

    pub fn stmt_eval(&mut self, expr: &Stmt) -> Result<HirExpr, Error> {
        match expr {
            Stmt::Expr(x) => self.typecheck_expr(x),
            _ => unimplemented!("The type system does not support other expressions yet"),
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
                    _ => unimplemented!("The type system does not support substraction yet"),
                };

                Ok(HirExpr::Binary(
                    Box::new(lhs_.clone()),
                    op.clone(),
                    Box::new(rhs_.clone()),
                    type_,
                ))
            }
            _ => unimplemented!("The type system does not support this type yet"),
        }
    }

    pub fn unify(&self, ty1: &Type, ty2: &Type) -> Result<Type, Error> {
        match (ty1, ty2) {
            (Type::Primitive(p1), Type::Primitive(p2)) if p1 == p2 => Ok(ty2.clone()),
            (Type::Primitive(p1), Type::Primitive(p2)) if p1 != p2 => {
                panic!("Types mismatch")
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
