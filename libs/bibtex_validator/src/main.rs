use bibtex_validator_lib;
use bibtex_parser_lib;
use bibtex_validator_lib::{validate_document, VALIDATORS};

fn main() {
    let doc = bibtex_parser_lib::deserialize_file("../../test/literatur.bib").unwrap();
    println!("{}", doc);
    let validators = VALIDATORS.lock().unwrap();
    println!("{:?}", validators);
    let report = validate_document(&validators[0], doc);
    println!("{:?}", report);
}
