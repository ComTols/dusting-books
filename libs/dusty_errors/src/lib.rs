use std::io;

pub type DustyResult<T> = Result<T, DustyError>;

#[derive(Debug)]
pub enum DustyError {
    LoadingError(io::Error),
    ValidatorParsingError(serde_yaml::Error),
    LogicalValidationError(Vec<LogicalValidationError>),
    BibtexParsingError(String),
    BuildError(String),
    ReadingError(std::io::Error),
}

#[derive(Debug)]
pub struct LogicalValidationError (pub String);
