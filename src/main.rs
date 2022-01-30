pub mod ast;
pub mod environment;
pub mod eval;
pub mod tests;
use ast::*;
use environment::*;
use eval::*;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::pedantic)]
    #[allow(dead_code)]
    parser
);

pub fn std_print(vals: Vec<Value>) -> Result<Value, String> {
    println!("{:?}", &vals);

    Ok(vals[0].clone())
}

fn main() {
    let mut env = Environment::default();
    env.define("println".to_string(), Value::Function(std_print))
        .unwrap();
    let input = std::fs::read_to_string("list.mrt").expect("Cannot read source file");
    let source = parser::ProgParser::new().parse(&input).unwrap();
    let res = eval(&source, &mut env).unwrap();
    println!("{}", res);
}
