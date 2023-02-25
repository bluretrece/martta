use crate::builtin::*;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
    pub vals: HashMap<String, Value>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Default for Environment {
    fn default() -> Self {
        let mut env = Self {
            vals: HashMap::new(),
            enclosing: None,
        };

        env.define(
            "reduce".to_string(),
            crate::value::Value::BuiltinFunction(reduce),
        );

        env
    }
}

impl Environment {
    pub fn define(&mut self, name: String, value: Value) -> Result<(), String> {
        self.vals.insert(name, value);
        Ok(())
    }

    pub fn with_ref(environment: Rc<RefCell<Environment>>) -> Self {
        Self {
            vals: HashMap::new(),
            enclosing: Some(environment),
        }
    }

    pub fn get_var(&mut self, name: String) -> Option<Value> {
        if let Some(value) = self.vals.get(&name).cloned() {
            return Some(value);
        } else if let Some(enclosing) = &self.enclosing {
            return (*enclosing.borrow_mut()).get_var(name.clone());
        }
        None
    }
}
