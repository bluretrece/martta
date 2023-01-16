use clap::Parser;
use std::cell::RefCell;
use std::rc::Rc;
pub mod ast;
pub mod builtin;
pub mod environment;
pub mod error;
pub mod interpreter;
pub mod repl;
pub mod tests;
pub mod type_checker;
use ast::*;
use environment::*;
use interpreter::*;
use repl::*;
use type_checker::*;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::pedantic)]
    #[allow(dead_code)]
    parser
);

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Command {
    #[arg(short = 'r', value_name = "run REPL")]
    repl: bool,

    #[arg(short = 'f', value_name = "open from file")]
    file: Option<String>,
}

fn main() {
    let args = Command::parse();

    match args.repl {
        true => Repl::run(),
        _ => println!("file handler"),
    }
    // let env = Environment::default();
    // let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
    // let mut tc = Typechecker::default();
    // let line = "let a: int = | n | => {
    //     return n + 1
    // };

    // a(4);";
    // let source: Prog = parser::ProgParser::new().parse(&line).unwrap();
    // let tc_value: Vec<HirExpr> = tc.typecheck(&source).unwrap();
    // let res = interpreter.run(&tc_value).unwrap();
    // // let res = interpreter.expr_eval(&tc_value).unwrap();
    // println!("{}", res);
    // loop {
    //     print!("> ");
    //     io::stdout().flush().unwrap();
    //     let mut line = String::new();
    //     io::stdin()
    //         .read_line(&mut line)
    //         .expect("Unable to read line from the REPL");
    //     if line.is_empty() {
    //         break;
    //     }
    //     let source: Prog = parser::ProgParser::new().parse(&line).unwrap();
    //     let tc_value: HirExpr = tc.typecheck(&source).unwrap();
    //     let res = interpreter.expr_eval(&tc_value).unwrap();
    //     println!("{}", res);
    // }
}
