use std::process::Command;

use quote::quote;
use pest_generator::derive_parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let t = quote! {
        #[derive(Parser)]
        #[grammar = "../grammar.pest"]
        pub struct MiniImp;
    };

    let t = derive_parser(t, false);

    let mut t = t.to_string();

    t = "use super::MiniImp;".to_string() + &t;

    std::fs::write("./src/parser/parser_gen.rs", t.to_string().as_bytes()).unwrap();

    Command::new("rustfmt")
        .arg("./src/parser/parser_gen.rs")
        .output().unwrap();

    Ok(())
}