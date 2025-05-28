use bibtex_parser_lib;

fn main() {
    let doc = bibtex_parser_lib::deserialize_file("../../test/literatur.bib").unwrap();
    println!("{}", doc);
}
