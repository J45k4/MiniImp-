use pest::{Parser, iterators::Pairs};

mod parser_gen;

pub use parser_gen::*;

pub struct MiniImp;

pub fn parse_text(input: &str) -> Pairs<parser_gen::Rule> {
    let r = MiniImp::parse(parser_gen::Rule::file, input).unwrap();

    r
}

// pub fn parse_file(path: &str) -> Pairs<parser_gen::Rule> {
//     let text = std::fs::read_to_string(path).unwrap();

//     let r = MiniImp::parse(parser_gen::Rule::file, &text).unwrap().clone();
// }