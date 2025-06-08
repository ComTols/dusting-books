use std::fmt::Display;
use serde::Deserialize;
use crate::models::validator::Validator;

#[derive(Debug, Deserialize, Clone)]
pub struct Report {
    /// The name of the validator used to create this report
    validator_name: String,
    /// The version of the validator used to create this report
    validator_version: String,

    /// List of findings
    findings: Vec<Finding>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Finding {
    /// Weather the finding was automatically corrected, needs the users attention or breaks the validation
    pub(crate) level: FindingLevel,
    /// The unique key of the element in witch the finding occurred
    pub(crate) element_key: String,
    /// The unique key of the field in the element in witch the finding occurred
    pub(crate) field_key: String,
    /// The message code related to the failed condition. Can be shown as a translated message to the user
    pub(crate) message_code: MessageCode,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub(crate) enum FindingLevel {
    /// The finding was automatically corrected
    Corrected,
    /// The finding needs the users attention
    Warning,
    /// The finding broken the validation
    Error,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub(crate) enum MessageCode {
    LogicalValidationError,
}

impl Display for MessageCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "{:<3}: unknown", "0"),
        }
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.findings)
    }
}

impl Report {
    pub(crate) fn from_validation(validator: &Validator) -> Self {
        Self {
            findings: vec![],
            validator_name: validator.name.clone(),
            validator_version: validator.version.clone(),
        }
    }
    
    pub(crate) fn push_finding(&mut self, finding: Finding) {
        self.findings.push(finding);
    }
}
