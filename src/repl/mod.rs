use crate::ast::*;
use crate::environment::*;
use crate::interpreter::Interpreter;
use crate::type_checker::*;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::pedantic)]
    #[allow(dead_code)]
    parser
);

pub struct Repl {}

impl Repl {
    pub fn run() {
        let env = Environment::default();
        let mut tc = Typechecker::default();
        let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
        loop {
            print!(":> ");
            std::io::stdout().flush().unwrap();
            let mut line = String::new();
            std::io::stdin()
                .read_line(&mut line)
                .expect("Unable to read line from the REPL");
            if line.is_empty() || line.contains(":q") {
                break;
            }
            let source: Prog = parser::ProgParser::new().parse(&line).unwrap();
            let tc_value: Vec<HirExpr> = tc.typecheck(&source).unwrap();
            let res = interpreter.run(&tc_value).unwrap();
            println!("{}", res);
        }
    }
}
