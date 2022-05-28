use compiler::compile;

mod parser;
mod compiler;
mod bytecode;

fn main() {
    // let file_content = std::fs::read_to_string("./example.mi").unwrap();

    let ast = parser::parse_text("var x = 5;").unwrap();

    println!("{:#?}", ast);

    compile(ast);
}
