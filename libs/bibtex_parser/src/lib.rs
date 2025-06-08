use std::fs::read_to_string;
use std::io::{Read, Write};
use dusty_errors::{DustyError, DustyResult};
use crate::models::document::Document;

mod parser;
mod errors;
pub mod models;

    pub fn deserialize_file(path: &str) -> DustyResult<Document> {
        let payload = match read_to_string(path) {
            Ok(contents) => contents,
            Err(err) => return Err(DustyError::LoadingError(err))
        };
        deserialize(payload.as_str())
    }
    pub fn deserialize_read<R: Read>(mut reader: R) -> DustyResult<Document> {
        let mut buffer = String::new();
        match reader.read_to_string(&mut buffer) {
            Ok(_) => (),
            Err(err) => return Err(DustyError::ReadingError(err))
        };
        deserialize(buffer.as_str())
    }
    pub fn deserialize(payload: &str) -> DustyResult<Document> {
        parser::parse(payload)
    }

    pub fn serialize_file(path: &str) {}
    pub fn serialize_write<R: Write>(mut writer: R) {}
    pub fn serialize( payload: &str) {}
