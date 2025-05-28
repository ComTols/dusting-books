use crate::parser::Rule;
use pest::error::Error;
use pest::iterators::Pair;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum DustyError {
    IOError(std::io::Error),
    ReadingError(std::io::Error),
    ParsingError(Error<Rule>),
    BuildError(ErrorStack),
}

impl Display for DustyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DustyError::IOError(e) => write!(f, "unable to read file: {}", e),
            DustyError::ReadingError(e) => write!(f, "unable to read from reader: {}", e),
            DustyError::ParsingError(e) => write!(f, "unable to parse input: {}", e),
            DustyError::BuildError(e) => write!(f, "unable to build input:\n{}", e),
        }
    }
}

#[derive(Debug)]
pub struct ErrorStack(Vec<String>);

impl Display for ErrorStack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for e in self.0.iter().rev() {
            write!(f, "\t{}\n", e)?;
        }
        Ok(())
    }
}

impl ErrorStack {
    pub(crate) fn new(rule: &Pair<Rule>) -> ErrorStack {
        let span = rule.as_span();
        let (line, col) = span.start_pos().line_col();
        ErrorStack(vec![format!(
            "line {} column {}: {}",
            line,
            col,
            span.as_str()
        )])
    }

    pub(crate) fn push_from_error(&mut self, errs: &ErrorStack) {
        for err in &errs.0 {
            self.0.push(err.clone());
        }
    }
}
