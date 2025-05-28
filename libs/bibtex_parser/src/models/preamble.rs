use std::fmt::{Display, Formatter};
use crate::models::value::Value;

#[derive(Debug)]
pub struct Preamble {
    pub content: Value,
}

impl Preamble {
    pub fn new(content: Value) -> Preamble {
        Preamble { content }
    }
}

impl Display for Preamble {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@preamble {}", self.content)
    }
}
