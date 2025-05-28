use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::models::value::Value;

#[derive(Default, Debug)]
pub struct KeyValueList (pub(crate) HashMap<String, Value>);

impl Display for KeyValueList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.0 {
            write!(f, "\t{} = {}\n", key, value.clone())?;
        }
        Ok(())
    }
}

impl KeyValueList {
    pub fn new() -> KeyValueList {
        KeyValueList(HashMap::new())
    }
    pub fn insert(&mut self, key: String, value: Value) {
        self.0.insert(key, value);
    }
}
