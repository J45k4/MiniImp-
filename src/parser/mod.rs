use anyhow::Result;
use pest::{Parser, iterators::Pairs, Token};

mod parser_gen;

pub use parser_gen::*;

pub struct MiniImp;

pub fn parse_text(input: &str) -> Result<Pairs<parser_gen::Rule>> {
    let r = MiniImp::parse(parser_gen::Rule::file, input)?;

    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::parse_text;

    #[test]
    fn test_parse_while() {
        let code = r#"while true begin
end."#;

        parse_text(code).unwrap();

        let code = "while true begin end.";

        parse_text(code).unwrap();
    }

    #[test]
    fn test_empty_file() {
        parse_text("").unwrap();
        parse_text(" ").unwrap();
    }

    #[test]
    fn test_if_stmt() {
        parse_text(r#"
if true then
    print("hello world");
end.
        "#).unwrap();
    }

    #[test]
    fn test_var() {
        parse_text(r#"var x = 25;"#).unwrap();
    }

    #[test]
    fn test_set() {
        parse_text(r#"set x = 25;"#).unwrap();
    }
}
