use pest::iterators::{Pairs, Pair};

use crate::bytecode::{Ins, ByteCode};
use crate::parser::Rule;
use crate::vm::{Vm, Value};

fn compile_factor(vm: &mut Vm, ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    let inner = inner.next().unwrap();

    let rule = inner.as_rule();

    if let Rule::expr = rule {
        compile_expr(vm, inner.into_inner().next().unwrap());

        return
    }

    let i = if let Rule::identifier = rule {
        let i = Ins{
            code: ByteCode::LoadGlobal,
            arg: vm.get_identifier_id(inner.as_str())
        };

        return
    } else {
        let const_id = match rule {
            Rule::number => {
                let num = inner.as_str().parse().unwrap();
                vm.store_const(crate::vm::Value::Number(num))
            },
            Rule::string_literal => {
                let str = inner.as_str();
                vm.store_const(Value::String(str.to_string()))
            },
            Rule::truth => {
                let b = inner.as_str().parse().unwrap();
                vm.store_const(Value::Boolean(b))
            },
            _ => unreachable!()
        };

        Ins {
            code: ByteCode::LoadConst,
            arg: const_id
        }
    };

    vm.add_instruction(i);
}

fn compile_term(vm: &mut Vm, ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    compile_factor(vm, inner.next().unwrap());

    let code = match inner.next() {
        Some(n) => match n.as_rule() {
            Rule::multi => ByteCode::BinMul,
            Rule::divide => ByteCode::BinDivide,
            _ => panic!("Not supported rule at this point")
        },
        _ => return
    };

    compile_factor(vm, inner.next().unwrap());
}

fn compile_expr(vm: &mut Vm, ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    compile_term(vm, inner.next().unwrap());

    let next = match inner.next() {
        Some(n) => n,
        _ => return
    };
    
    let op = match next.as_rule() {
        Rule::plus => ByteCode::BinAdd,
        Rule::minus => ByteCode::BinMinus,
        _ => panic!("Not supported rule at this point")
    };

    compile_term(vm, inner.next().unwrap());

    let i = Ins {
        code: op,
        arg: 0
    };

    vm.add_instruction(i);
}

fn compile_identifier(vm: &mut Vm, ast: Pair<Rule>) {

}

fn compile_set_stmt(vm: &mut Vm, ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    compile_identifier(vm, inner.next().unwrap());
    compile_expr(vm, inner.next().unwrap());
}

fn compile_var_stmt(vm: &mut Vm, ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    let s = inner.next().unwrap();

    println!("{}", s.as_str());

    compile_expr(vm, inner.next().unwrap());

    let arg = vm.get_identifier_id(s.as_str());

    let i = Ins{
        code: ByteCode::StoreGlobal,
        arg: arg
    };

    vm.add_instruction(i);
}

fn compile_scope(vm: &mut Vm, ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    compile_stmts(vm, inner.next().unwrap());
}

fn compile_while_stmt(vm: &mut Vm, ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    compile_expr(vm, inner.next().unwrap());
    compile_scope(vm, inner.next().unwrap());
}

fn compile_if_stmt(vm: &mut Vm, ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    compile_expr(vm, inner.next().unwrap());
    compile_stmts(vm, inner.next().unwrap());
}

fn compile_arg(vm: &mut Vm, ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    compile_expr(vm, inner.next().unwrap());
}

fn compile_call_stmt(vm: &mut Vm, ast: Pair<Rule>) {
    let mut inner = ast.into_inner();

    compile_expr(vm, inner.next().unwrap());
    
    for t in inner {
        compile_arg(vm, t);
    }
}

fn compile_stmt(vm: &mut Vm, ast: Pair<Rule>) {
    for t in ast.into_inner() {
        match t.as_rule() {
            Rule::if_stmt => compile_if_stmt(vm, t),
            Rule::while_stmt => compile_while_stmt(vm, t),
            Rule::set_stmt => compile_set_stmt(vm, t),
            Rule::var_stmt => compile_var_stmt(vm, t),
            Rule::call_stmt => compile_call_stmt(vm, t),
            _ => {}
        }
    }
}

fn compile_stmts(vm: &mut Vm, ast: Pair<Rule>) {
    for t in ast.into_inner() {
        match t.as_rule() {
            Rule::stmt => compile_stmt(vm, t),
            _ => {}
        }
    }
}

fn compile_file(vm: &mut Vm, ast: Pair<Rule>) {
    let inner = ast.into_inner();

    for t in inner {
        match t.as_rule() {
            Rule::EOI => {},
            Rule::stmts => compile_stmts(vm, t),
            _ => unreachable!()
        }
    }
}

pub fn compile(ast: Pairs<Rule>) -> Vm {
    let mut vm = Vm::new();

    for t in ast {
        match t.as_rule() {
            Rule::file => {
                compile_file(&mut vm, t);
            },
            _ => {
                panic!("Not supported")
            }
        }
    }

    vm
}