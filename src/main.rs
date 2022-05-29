use compiler::compile;

mod parser;
mod compiler;
mod bytecode;
mod vm;

fn main() {
    let file_content = std::fs::read_to_string("./example.mi").unwrap();

    let ast = parser::parse_text(&file_content).unwrap();

    println!("{:?}", ast);

    let mut vm = compile(ast);

    println!("{:?}", vm);

    vm.run();

    println!("{:?}", vm);
}
