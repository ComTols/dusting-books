use std::fmt::{Display, Formatter};
use crate::models::entry::Entry;
use crate::models::key_value::KeyValueList;
use crate::models::preamble::Preamble;
use crate::models::value::Value;

pub(crate) mod preamble;
pub(crate) mod entry;
pub(crate) mod value;
pub(crate) mod key_value;
pub mod document;

#[derive(Debug)]
pub struct StringDef(pub(crate) KeyValueList);

impl Display for StringDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "@string {{\n{}}}", self.0)
    }
}

impl StringDef {
    pub fn new(list: KeyValueList) -> StringDef {
        StringDef(list)
    }
}

#[derive(Debug)]
pub struct Comment (Value);

impl Comment {
    pub fn new(value: Value) -> Comment {
        Comment(value)
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "@comment {}", self.0)
    }
}

#[derive(Debug)]
pub enum Element {
    StringDef(StringDef),
    Comment(Comment),
    Preamble(Preamble),
    Entry(Entry)
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::StringDef(value) => {write!(f, "{}", value)}
            Element::Comment(value) => {write!(f, "{}", value)}
            Element::Preamble(value) => {write!(f, "{}", value)}
            Element::Entry(value) => {write!(f, "{}", value)}
        }
    }
}
