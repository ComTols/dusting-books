use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use serde::Deserialize;
use bibtex_parser_lib::models::document::Document;
use crate::errors::{DustyError, DustyResult};
use crate::models::report::Report;

#[derive(Debug, Deserialize, Clone)]
pub struct Validator {

    /// The name of the validator.
    /// Is written in the issuing file to recognize the validator.
    /// The name must be unique. To offer several versions, see field version.
    pub(crate) name: String,
    /// The version of the validator.
    /// Can be used to validate old files without obtaining errors.
    /// The version should always correspond to the version of the latex pack used.
    pub(crate) version: String,

    /// List of the possible entry definitions.
    elements: HashMap<String, Element>,

    /// List of fields that must be present in all elements.
    /// They are automatically added to each element and can be deactivated for individual elements if necessary.
    global_required_fields: Option<HashMap<String, Field>>,
    /// List of fields that can be present in all elements.
    /// They are automatically added to each element and can be deactivated for individual elements if necessary.
    global_optional_fields: Option<HashMap<String, Field>>,
}

#[derive(Debug, Deserialize, Clone)]
struct Element {
    /// List of necessary elements.
    /// If one of the elements is not available, the validation fails.
    required: Vec<Condition>,
    /// List of possible elements.
    optional: Vec<Condition>,
    /// List of prohibited elements.
    /// If one of the elements is present, the validation fails.
    unsupported: Option<Vec<String>>,

    /// List of global fields that do not occur in this element.
    /// They are not added as prohibited, but only deactivated. If one of the fields still occurs, it is handled as unknown.
    disabled_global_fields: Option<Vec<String>>,

    /// List of dependent elements.
    /// If this element is present, then at least one element of each defined type must also be present.
    dependencies: Option<Vec<String>>,

}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "condition", content = "fields")]
enum Condition {
    OneOf(HashMap<String, Field>),
    AllOf(HashMap<String, Field>),
}

#[derive(Debug, Deserialize, Clone)]
struct Field {
    /// The type of the field. Can be string, number, date or dateTime.
    data_type: Option<SupportedPrimitiveDataType>,
    /// Specifies whether an additional tag is allowed after the key, challenged or forbidden.
    ///
    /// Example: `title[de]`
    tag: Option<TagDefinition>,

    /// A list of dependencies.
    /// If this field occurs, all dependencies must also occur.
    dependencies: Option<Vec<Dependency>>,

    /// Can define a regex that is compared. Only effective with type String.
    pattern: Option<String>,
    /// Can define a maximum value. Only effective with type Number.
    min: Option<i64>,
    /// Can define a minimum value. Only effective with type Number.
    max: Option<i64>,
    /// Can define a default value.
    /// If this field is not available in the element, it will be added.
    default: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
enum TagDefinition {
    /// No tag allowed
    No,
    /// Tag allowed
    Optional,
    /// Tag required
    Required
}

#[derive(Debug, Deserialize, Clone)]
struct Dependency {
    /// The path to the required field.
    /// It has to start with an entry definition:
    ///  - The keyword `this` use for this entry
    ///  - For a specific entry definition, use the keyword of the entry definition.
    ///  - For any entry types, leave it empty.
    ///
    /// The parts are separated by points.
    ///
    /// Example: `this.url`
    path: String,

    /// The name of a field that is to be compared.
    /// If this option is specified, the field defined here is also compared from the list of the matching paths. The values must match.
    ///
    /// Example: `path: 'proceedings', join_on: 'key'` search for an occurrence of a proceeding element and compares the key.
    join_on: Option<String>,

    /// Specified whether the content of the matched element is integrated into this element.
    ///
    /// Example: `path: 'xdata', join_on: 'key', append: 'add'` add the elements of a xdata element with a matching key to this element.
    append: Option<AppendOptions>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "value")]
enum SupportedPrimitiveDataType {
    /// Can be used if the data type is not specifically predictable. Matches all data types.
    All,
    String,
    Number,
    Date,
    DateTime,
    List(List),
    Enum(Vec<String>),
}

#[derive(Debug, Deserialize, Clone)]
struct List {
    /// A character set to separate entries from each other.
    ///
    /// Example: `and` defines `Mustermann, Max and Proband, Peter` as two elements: `Mustermann, Max` and `Proband, Peter`
    separator: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
enum AppendOptions {
    /// Do not extend this element
    No,
    /// Extends this element.
    /// If the field has already been defined in this element, it is not added and skipped.
    Add,
    /// Extends this element.
    /// If the field is already defined in this element, it will be overwritten.
    Override
}

impl Validator {
    pub(crate) fn from_file(path: PathBuf) -> DustyResult<Validator> {
        let contents = match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(e) => return Err(DustyError::LoadingError(e)),
        };

        let validator: Self = match serde_yaml::from_str(contents.as_str()) {
            Ok(validator) => validator,
            Err(e) => return Err(DustyError::ParsingError(e)),
        };


        Ok(validator)
    }
    
    pub(crate) fn validate(&self, _document: &Document) -> DustyResult<Report> {
        Ok(Report::from_validation(self))
    }
}

impl Display for Validator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.version)
    }
}