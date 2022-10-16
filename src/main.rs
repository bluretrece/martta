use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;
pub mod ast;
pub mod builtin;
pub mod environment;
pub mod error;
pub mod interpreter;
pub mod tests;
pub mod type_checker;
use ast::*;
use environment::*;
use interpreter::*;
use type_checker::*;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::pedantic)]
    #[allow(dead_code)]
    parser
);

fn main() {
    let env = Environment::default();
    let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
    let mut tc = Typechecker::default();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Unable to read line from the REPL");
        if line.is_empty() {
            break;
        }
        let source: Prog = parser::ProgParser::new().parse(&line).unwrap();
        let tc_value: HirExpr = tc.typecheck(&source).unwrap();
        let res = interpreter.expr_eval(&tc_value).unwrap();
        println!("{}", res);
    }
}
