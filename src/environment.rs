use crate::Value;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Environtment {
    pub vals: HashMap<String, Value>,
    // pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environtment {
    pub fn define(&mut self, name: String, value: Value) -> Result<Value, String> {
        self.vals.insert(name, value.clone());
        Ok(value)
    }

    pub fn get_var(&mut self, name: String) -> Option<Value> {
        self.vals.get(&name).map(|value| value.clone())
    }
}
