use pest::iterators::{Pairs, Pair};

use crate::parser::Rule;

fn compile_boolean(ast: Pair<Rule>) {

}



fn compile_expr(ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    for t in inner {
        match t.as_rule() {
            Rule::boolean => todo!(),
            Rule::is_bool => todo!(),
            Rule::not_bool => todo!(),
            Rule::term => todo!(),
            Rule::number => todo!(),
            Rule::string_literal => todo!(),
            Rule::true_bool => todo!(),
            Rule::false_bool => todo!(),
            _ => {}
        }
    }
}

fn compile_identifier(ast: Pair<Rule>) {

}

fn compile_set_stmt(ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    compile_identifier(inner.next().unwrap());
    compile_expr(inner.next().unwrap());
}

fn compile_var_stmt(ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    compile_identifier(inner.next().unwrap());
    compile_expr(inner.next().unwrap());
}

fn compile_scope(ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    compile_stmts(inner.next().unwrap());
}

fn compile_while_stmt(ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    compile_expr(inner.next().unwrap());
    compile_scope(inner.next().unwrap());
}

fn compile_if_stmt(ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    compile_expr(inner.next().unwrap());
    compile_stmts(inner.next().unwrap());
}

fn compile_arg(ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    compile_expr(inner.next().unwrap());
}

fn compile_call_stmt(ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    compile_expr(inner.next().unwrap());
    
    for t in inner {
        compile_arg(t);
    }
}

fn compile_stmt(ast: Pair<Rule>) {
    for t in ast.into_inner() {
        match t.as_rule() {
            Rule::if_stmt => compile_if_stmt(t),
            Rule::while_stmt => compile_while_stmt(t),
            Rule::set_stmt => compile_set_stmt(t),
            Rule::var_stmt => compile_var_stmt(t),
            Rule::call_stmt => compile_call_stmt(t),
            _ => {}
        }
    }
}

fn compile_stmts(ast: Pair<Rule>) {
    for t in ast.into_inner() {
        match t.as_rule() {
            Rule::stmt => compile_stmt(t),
            _ => {}
        }
    }
}

fn compile_file(ast: Pair<Rule>) {
    let inner = ast.into_inner();

    for t in inner {
        match t.as_rule() {
            Rule::EOI => {},
            Rule::identifier => todo!(),
            Rule::plus => todo!(),
            Rule::minus => todo!(),
            Rule::multi => todo!(),
            Rule::divide => todo!(),
            Rule::operator => todo!(),
            Rule::arg => todo!(),
            Rule::expr => todo!(),
            Rule::program => todo!(),
            Rule::stmt => todo!(),
            Rule::stmts => compile_stmts(t),
            Rule::scope => todo!(),
            _ => {}
        }
    }
}

pub fn compile(ast: Pairs<Rule>) {
    for t in ast {
        match t.as_rule() {
            Rule::file => {
                compile_file(t);
            },
            _ => {
                panic!("Not supported")
            }
        }
    }
}