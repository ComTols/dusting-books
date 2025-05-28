use std::fmt::{Display, Formatter};
use crate::models::Element;

#[derive(Debug)]
pub struct Document (pub(crate) Vec<Element>);

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for d in &self.0 {
            write!(f, "{}\n", d)?;
        }
        Ok(())
    }
}


