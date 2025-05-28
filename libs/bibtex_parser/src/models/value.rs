use std::fmt::Display;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    String(String),
    Key(String),
    Mixed(Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Key(s) => write!(f, "{}", s),
            Value::Mixed(values) => {
                for (i, value) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, " + ")?;
                    }
                    write!(f, "{}", value)?;
                }
                Ok(())
            }
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::String(String::new())
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::String(s) => {
                0u8.hash(state);
                s.hash(state);
            }
            Value::Key(k) => {
                1u8.hash(state);
                k.hash(state);
            }
            Value::Mixed(values) => {
                2u8.hash(state);
                values.hash(state);
            }
        }
    }
}
