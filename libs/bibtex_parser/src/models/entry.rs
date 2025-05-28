use std::fmt::{Display, Formatter};
use crate::models::key_value::KeyValueList;

#[derive(Debug)]
pub struct Entry {
    pub model: String,
    pub key: String,
    pub entries: KeyValueList,
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{} {{\n\t{}\n{}}}", self.model, self.key, self.entries)
    }
}

impl Entry {
    pub fn new(model: String, key: String, entries: KeyValueList) -> Entry {
        Self {model, key, entries}
    }
}
