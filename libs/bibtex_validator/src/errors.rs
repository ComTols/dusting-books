use std::io;
use serde::Deserialize;

pub type DustyResult<T> = Result<T, DustyError>;

#[derive(Debug)]
pub enum DustyError {
    LoadingError(io::Error),
    ParsingError(serde_yaml::Error),
}