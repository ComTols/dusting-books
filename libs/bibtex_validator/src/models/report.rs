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
struct Finding {
    /// Weather the finding was automatically corrected, needs the users attention or breaks the validation
    level: FindingLevel,
    /// The unique key of the element in witch the finding occurred
    element_key: String,
    /// The unique key of the field in the element in witch the finding occurred
    field_key: String,
    /// The message code related to the failed condition. Can be shown as a translated message to the user
    message_code: MessageCode,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum FindingLevel {
    /// The finding was automatically corrected
    Corrected,
    /// The finding needs the users attention
    Warning,
    /// The finding broken the validation
    Error,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum MessageCode {

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
}
