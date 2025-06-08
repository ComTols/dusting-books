use std::fmt::{Display, Formatter};
use crate::models::{Element};
use crate::models::value::Value;
use dusty_errors::{DustyError, DustyResult, LogicalValidationError};

#[derive(Debug)]
pub struct Document (pub Vec<Element>);

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for d in &self.0 {
            write!(f, "{}\n", d)?;
        }
        Ok(())
    }
}


impl Document {
    pub fn valid(&self) -> DustyResult<()> {
        let mut errors: Vec<LogicalValidationError> = vec![];
        for element in &self.0 {
            match element {
                Element::StringDef(def) => {
                    for (_, value) in def.0.0.iter() {
                        match self.valid_value(value) {
                            Ok(_) => {},
                            Err(e) => match e { 
                                DustyError::LogicalValidationError(mut errs) => errors.append(&mut errs),
                                _ => return Err(e),
                            }
                        }
                    }
                }
                Element::Preamble(preamble) => {
                    match self.valid_value(&preamble.content) {
                        Ok(_) => {},
                        Err(e) => match e {
                            DustyError::LogicalValidationError(mut errs) => errors.append(&mut errs),
                            _ => return Err(e),
                        }
                    }
                }
                Element::Comment(comment) => {
                    match self.valid_value(&comment.0) {
                        Ok(_) => {},
                        Err(e) => match e {
                            DustyError::LogicalValidationError(mut errs) => errors.append(&mut errs),
                            _ => return Err(e),
                        }
                    }
                }
                Element::Entry(entry) => {
                    for (_, value) in entry.entries.0.iter() {
                        match self.valid_value(value) {
                            Ok(_) => {},
                            Err(e) => match e {
                                DustyError::LogicalValidationError(mut errs) => errors.append(&mut errs),
                                _ => return Err(e),
                            }
                        }
                    }
                }
            }
        }
        
        if !errors.is_empty() {
            return Err(DustyError::LogicalValidationError(errors));
        }
        
        Ok(())
    }
    
    fn valid_value(&self, value: &Value) -> DustyResult<()> {
        match value { 
            Value::String(_) => Ok(()),
            Value::Mixed(values) => {
                let mut errors: Vec<LogicalValidationError> = vec![];
                for val in values {
                    match self.valid_value(val) {
                        Ok(_) => {},
                        Err(e) => {
                            match e { 
                                DustyError::LogicalValidationError(mut errs) => errors.append(&mut errs),
                                _ => return Err(e)
                            }
                        },
                    };
                    
                }
                if !errors.is_empty() {
                    return Err(DustyError::LogicalValidationError(errors));
                }
                Ok(())
            }
            Value::Key(key) => {
                if !self.has_string_def(key) {
                    return Err(DustyError::LogicalValidationError(vec![LogicalValidationError(format!("key '{}' not defined", key))]));
                }
                Ok(())
            },
        }
    }
    
    fn has_string_def(&self, key: &String) -> bool {
        for element in &self.0 {
            match element { 
                Element::StringDef(def) => {
                    if def.0.0.get(key).is_some() {
                        return true;
                    }
                }
                _ => {},
            }
        }
        false
    }
}

