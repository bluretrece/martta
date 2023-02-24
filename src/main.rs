use clap::Parser;
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

    match args.file {
        None => Repl::run(),
        Some(_) => println!("file handler"),
    }
}
