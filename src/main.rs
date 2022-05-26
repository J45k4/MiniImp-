mod parser;



fn main() {
    let file_content = std::fs::read_to_string("./example.mi").unwrap();

    let ast = parser::parse_text(&file_content);

    println!("{:#?}", ast);
}
