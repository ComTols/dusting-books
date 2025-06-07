use bibtex_validator_lib;
use bibtex_parser_lib;
use bibtex_validator_lib::load_validators;

fn main() {
    let doc = bibtex_parser_lib::deserialize_file("../../test/literatur.bib").unwrap();
    println!("{}", doc);
    let validators = load_validators(Some("validators".to_string())).unwrap();
    println!("{:?}", validators);
}
