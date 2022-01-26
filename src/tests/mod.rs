#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn int_parsing() {
        let mut env = Environment::default();
        let input = "197";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = eval(&source, &mut env);

        assert_eq!(res, Ok(Value::Int(197)));
    }

    #[test]
    fn bool_parsing() {
        let mut env = Environment::default();
        let input = "true";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = eval(&source, &mut env);

        assert_eq!(res, Ok(Value::Bool(true)));
    }

    #[test]
    fn int_printing() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::Function(std_print))
            .unwrap();
        let input = "println(40 + 6);";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = eval(&source, &mut env);

        assert_eq!(res, Ok(Value::Int(46)));
    }

    #[test]
    fn assignment_and_println() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::Function(std_print))
            .unwrap();
        let input = "let x = 5;\n let a = x;\n a;";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = eval(&source, &mut env);

        assert_eq!(res, Ok(Value::Int(5)));
    }

    #[test]
    fn false_bool_parsing() {
        let mut env = Environment::default();
        let input = "false";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = eval(&source, &mut env);

        assert_eq!(res, Ok(Value::Bool(false)));
    }

    #[test]
    fn binary_bool_assignment() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::Function(std_print))
            .unwrap();
        let input = "let b = false && false; b;";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = eval(&source, &mut env);

        assert_eq!(res, Ok(Value::Bool(false)));
    }

    #[test]
    fn if_should_not_evaluate() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::Function(std_print))
            .unwrap();
        let input = "if false || false {
			99;
		    }";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = eval(&source, &mut env);

        assert_eq!(res, Ok(Value::Nil));
    }

    #[test]
    fn if_should_evaluate() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::Function(std_print))
            .unwrap();
        let input = "if true {
			println(99);
		     } else {
			println(86);
		     }";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = eval(&source, &mut env);

        assert_eq!(res, Ok(Value::Int(99)));
    }

    #[test]
    #[ignore]
    fn source_file_evaluation() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::Function(std_print))
            .unwrap();
        let input = std::fs::read_to_string("hello.mrt").expect("Cannot read source file");
        let source = parser::ProgParser::new().parse(&input).unwrap();
        let res = eval(&source, &mut env);

        assert_eq!(res, Ok(Value::Int(91)));
    }

    #[test]
    fn else_evaluation() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::Function(std_print))
            .unwrap();
        let input = "if 1 == 4 {
			println(99);
		     } else {
			println(86);
		     }";
        let source = parser::ProgParser::new().parse(&input).unwrap();
        let res = eval(&source, &mut env);

        assert_eq!(res, Ok(Value::Int(86)));
    }
}
