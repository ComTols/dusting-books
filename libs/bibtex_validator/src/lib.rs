use std::{env, fs};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};
use bibtex_parser_lib::models::document::Document;
use logger::*;
use crate::errors::{DustyResult};
use crate::models::{report::Report, validator::Validator};

mod models;
pub(crate) mod errors;
mod globals;

pub static VALIDATORS : LazyLock<Mutex<Vec<Validator>>> = LazyLock::new(|| {Mutex::new(load_validators(None))});

fn load_validators(path: Option<String>) -> Vec<Validator> {
    let path = match path {
        None => { 
            let working_dir_validators = Path::new("validators");
            if working_dir_validators.exists() && working_dir_validators.is_dir() {
                working_dir_validators.to_path_buf()
            } else {
                let exe_path = env::current_exe().unwrap();
                let program_dir = exe_path.parent().unwrap().to_path_buf();
                program_dir.join("validators")
            }
        },
        Some(p) => Path::new(p.as_str()).to_path_buf()
    };

    let validator_paths = find_yaml_files(path);
    
    let mut validators = vec![];
    for validator_path in validator_paths {
        validators.push(match Validator::from_file(validator_path.clone()) {
            Ok(validator) => validator,
            Err(e) => { 
                warn!("Error while loading validator {}: {:?}", validator_path.display(), e);
                continue
            },
        });
    }
    
    remove_duplicates(&mut validators);

    validators
}

pub fn reload_validators(path: Option<String>) -> Vec<Validator> {
    let mut static_validators = VALIDATORS.lock().unwrap();
    *static_validators = load_validators(path);
    static_validators.clone()
}

pub fn validate_document(validator: &Validator, document: Document) -> DustyResult<Report> {
    validator.validate(&document)
}

fn find_yaml_files(dir: PathBuf) -> Vec<PathBuf> {
    let mut yaml_files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yaml" {
                        yaml_files.push(path);
                    }
                }
            }
        }
    }

    yaml_files
}

fn remove_duplicates(validators: &mut Vec<Validator>) {
    let mut seen = HashSet::new();
    
    validators.retain(|validator| {
        if !seen.insert(
            (
                validator.name.clone(),
                validator.version.clone()
            )
        ) {
            warn!("duplicate validator removed: {:?}", validator);
            return false;
        }
        true
    })
}
