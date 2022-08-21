use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;
pub mod ast;
pub mod environment;
pub mod error;
pub mod interpreter;
pub mod tests;
use ast::*;
use environment::*;
use interpreter::*;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::pedantic)]
    #[allow(dead_code)]
    parser
);

pub fn std_print(vals: Vec<Value>) -> Result<Value, error::Error> {
    println!("{:?}", &vals);

    Ok(vals[0].clone())
}

fn main() {
    let mut env = Environment::default();
    let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
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
        let res = interpreter.eval(&source).unwrap();
        println!("{}", res);
    }
}
