#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn anonymous_fn() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::BuiltinFunction(std_print))
            .unwrap();
        let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
        let input = "let a = fn (n) => {
            return n + 1
        };

        a(4);";

        let source = parser::ProgParser::new().parse(&input).unwrap();
        let res = interpreter.eval(&source);

        assert_eq!(res, Ok(Value::Int(5)));
    }

    #[test]
    fn fibonacci() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::BuiltinFunction(std_print))
            .unwrap();
        let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
        let input = "fn sum :: n {
            if n < 2 {
                return n;
            } else {
                return sum(n-1) + sum(n-2);
            }
        }

        let n=8;
        sum(n)";

        let source = parser::ProgParser::new().parse(&input).unwrap();
        let res = interpreter.eval(&source);

        assert_eq!(res, Ok(Value::Int(21)));
    }

    #[test]
    #[should_panic]
    fn bool_operation() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::BuiltinFunction(std_print))
            .unwrap();
        let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
        let input = "true && 2";

        let source = parser::ProgParser::new().parse(&input).unwrap();
        let res = interpreter.eval(&source);

        assert_eq!(res, Ok(Value::Int(1)));
    }

    #[test]
    fn re_assignment() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::BuiltinFunction(std_print))
            .unwrap();
        let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
        let input = "
            let a = 1;

            a = 4;
            println(a);";

        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = interpreter.eval(&source);

        assert_eq!(res, Ok(Value::Int(4)));
    }

    #[test]
    fn int_parsing() {
        let env = Environment::default();
        let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
        let input = "197";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = interpreter.eval(&source);

        assert_eq!(res, Ok(Value::Int(197)));
    }

    #[test]
    fn bool_parsing() {
        let env = Environment::default();
        let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
        let input = "true";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = interpreter.eval(&source);

        assert_eq!(res, Ok(Value::Bool(true)));
    }

    #[test]
    fn int_printing() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::BuiltinFunction(std_print))
            .unwrap();
        let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
        let input = "println(40 + 6);";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = interpreter.eval(&source);

        assert_eq!(res, Ok(Value::Int(46)));
    }

    #[test]
    fn assignment_and_println() {
        let env = Environment::default();
        let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
        let input = "let x = 5;\n let a = x;\n a;";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = interpreter.eval(&source);

        assert_eq!(res, Ok(Value::Int(5)));
    }

    #[test]
    fn false_bool_parsing() {
        let env = Environment::default();
        let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
        let input = "false";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = interpreter.eval(&source);

        assert_eq!(res, Ok(Value::Bool(false)));
    }

    #[test]
    fn binary_bool_assignment() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::BuiltinFunction(std_print))
            .unwrap();
        let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
        let input = "let b = false && false; b;";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = interpreter.eval(&source);

        assert_eq!(res, Ok(Value::Bool(false)));
    }

    #[test]
    fn if_should_not_evaluate() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::BuiltinFunction(std_print))
            .unwrap();
        let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
        let input = "if false || false {
			99;
		    }";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = interpreter.eval(&source);

        assert_eq!(res, Ok(Value::Nil));
    }

    #[test]
    fn if_should_evaluate() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::BuiltinFunction(std_print))
            .unwrap();
        let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
        let input = "if true {
			println(99);
		     } else {
			println(86);
		     }";
        let source = parser::ProgParser::new().parse(input).unwrap();
        let res = interpreter.eval(&source);

        assert_eq!(res, Ok(Value::Int(99)));
    }

    #[test]
    fn else_evaluation() {
        let mut env = Environment::default();
        env.define("println".to_string(), Value::BuiltinFunction(std_print))
            .unwrap();
        let mut interpreter = Interpreter::new(Rc::new(RefCell::new(env)));
        let input = "if 1 == 4 {
			println(99);
		     } else {
			println(86);
		     }";
        let source = parser::ProgParser::new().parse(&input).unwrap();
        let res = interpreter.eval(&source);

        assert_eq!(res, Ok(Value::Int(86)));
    }
}
