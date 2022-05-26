mod parser;

fn main() {
    let ast = parser::parse_text("+");

    println!("{:#?}", ast);
}
