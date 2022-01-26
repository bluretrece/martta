use crate::Value;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Environment {
    pub vals: HashMap<String, Value>,
    // pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: Value) -> Result<Value, String> {
        self.vals.insert(name, value.clone());
        Ok(value)
    }

    pub fn get_var(&mut self, name: String) -> Option<Value> {
        self.vals.get(&name).cloned()
    }
}
