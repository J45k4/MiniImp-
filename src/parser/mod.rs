use pest::{Parser, iterators::Pairs};

mod parser_gen;

pub struct MiniImp;

pub fn parse_text(input: &str) -> Pairs<parser_gen::Rule> {
    let r = MiniImp::parse(parser_gen::Rule::file, input).unwrap();

    r
}