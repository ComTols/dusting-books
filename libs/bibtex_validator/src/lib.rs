use crate::LogLevel;
use std::{env, fs};
use std::path::{Path, PathBuf};
use bibtex_parser_lib::models::document::Document;
use logger::*;
use crate::errors::{DustyError, DustyResult};
use crate::models::Validator;

mod models;
pub(crate) mod errors;

pub fn load_validators(path: Option<String>) -> DustyResult<Vec<Validator>> {
    let path = match path {
        None => { 
            let exe_path = env::current_exe().unwrap();
            let program_dir = exe_path.parent().unwrap().to_path_buf();
            program_dir.join("validators")
        },
        Some(p) => Path::new(p.as_str()).to_path_buf()
    };

    let validator_paths = find_yaml_files(path);
    
    let mut validators = vec![];
    for validator_path in validator_paths {
        validators.push(match Validator::from_file(validator_path.clone()) {
            Ok(validator) => validator,
            Err(_) => { 
                warn!("Error while loading validator: {}", validator_path.display());
                continue
            },
        });
    }
    
    println!("Loaded validators: {:?}", validators);
    
    Ok(validators)
}

pub fn validate_document(document: Document) {
    
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
