use pest::iterators::{Pairs, Pair};

use crate::bytecode::{Ins, ByteCode, LOGICAL_AND, EQUAL_TO_OP, LOGICAL_OR, SMALLER_THAN_OP, GREATER_THAN_OP, GREATER_THAN_EQUAL_TO_OP, SMALLER_THAN_EQUAL_TO_OP, NOT_EQUAL_TO_OP};
use crate::parser::Rule;
use crate::vm::{Vm, Value};

fn compile_factor(vm: &mut Vm, ast: Pair<Rule>) {
    log::debug!("compile_factor");

    let mut inner = ast.into_inner();

    let inner = inner.next().unwrap();

    let rule = inner.as_rule();

    if let Rule::expr = rule {
        compile_expr(vm, inner.into_inner().next().unwrap());

        return
    }

    let i = if let Rule::identifier = rule {
        let id = vm.store_identifier(inner.as_str());

        Ins{
            code: ByteCode::Load,
            arg: id 
        }
    } else {
        let const_id = match rule {
            Rule::number => {
                let num = inner.as_str().parse().unwrap();
                vm.store_new(crate::vm::Value::Number(num))
            },
            Rule::string_literal => {
                let inner = inner.into_inner().next().unwrap();

                let str = inner.as_str();
                vm.store_new(Value::String(str.to_string()))
            },
            Rule::truth => {
                let b = inner.as_str().parse().unwrap();
                vm.store_new(Value::Boolean(b))
            },
            _ => unreachable!()
        };

        Ins {
            code: ByteCode::Load,
            arg: const_id
        }
    };

    vm.add_instruction(i);
}

fn compile_term(vm: &mut Vm, ast: Pair<Rule>) {
    log::debug!("compile_term");

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

    vm.add_instruction(Ins{
        code,
        arg: 0
    });
}

fn compile_expr(vm: &mut Vm, ast: Pair<Rule>) {
    log::debug!("compile_expr");

    let mut inner = ast.into_inner();

    loop {
        let next = match inner.next() {
            Some(n) => n,
            None => break
        };

        let next_rule = next.as_rule();

        match next_rule {
            Rule::plus | Rule::minus => {
                let op = match next_rule {
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
            Rule::logical_and | 
            Rule::logical_or |
            Rule::logical_eq |
            Rule::logical_smaller |
            Rule::logical_bigger |
            Rule::logical_smaller_eq |
            Rule::logical_bigger_eq |
            Rule::logical_not_eq |
            Rule::is |
            Rule::not => {
                compile_expr(vm, inner.next().unwrap());

                let a = match next_rule {
                    Rule::logical_and => LOGICAL_AND,
                    Rule::logical_or => LOGICAL_OR,
                    Rule::logical_eq => EQUAL_TO_OP,
                    Rule::logical_smaller => SMALLER_THAN_OP,
                    Rule::logical_bigger => GREATER_THAN_OP,
                    Rule::logical_smaller_eq => SMALLER_THAN_EQUAL_TO_OP,
                    Rule::logical_bigger_eq => GREATER_THAN_EQUAL_TO_OP,
                    Rule::logical_not_eq => NOT_EQUAL_TO_OP,
                    Rule::is => EQUAL_TO_OP,
                    Rule::not => NOT_EQUAL_TO_OP,
                    _ => unimplemented!()
                };

                let i = Ins {
                    code: ByteCode::CmpEq,
                    arg: a
                };

                vm.add_instruction(i);
            },
            Rule::expr => {
                compile_expr(vm, next);
            },
            Rule::term => {
                compile_term(vm, next);
            },
            _ => {}
        }
    }
}

fn compile_set_stmt(vm: &mut Vm, ast: Pair<Rule>) {
    log::debug!("compile_set_stmt");

    let mut inner = ast.into_inner();

    let s = inner.next().unwrap();

    let arg = vm.store_identifier(s.as_str());

    compile_expr(vm, inner.next().unwrap());

    let i = Ins{
        code: ByteCode::Store,
        arg: arg
    };

    vm.add_instruction(i);
}

fn compile_var_stmt(vm: &mut Vm, ast: Pair<Rule>) {
    log::debug!("compile_var_stmt");

    let mut inner = ast.into_inner();

    let s = inner.next().unwrap();

    let arg = vm.store_identifier(s.as_str());

    compile_expr(vm, inner.next().unwrap());

    let i = Ins{
        code: ByteCode::Store,
        arg: arg
    };

    vm.add_instruction(i);
}

fn compile_scope(vm: &mut Vm, ast: Pair<Rule>) {
    log::debug!("compile_scope");

    let mut inner = ast.into_inner();

    compile_stmts(vm, inner.next().unwrap());
}

fn compile_while_stmt(vm: &mut Vm, ast: Pair<Rule>) {
    log::debug!("compile_while_stmt");

    let mut inner = ast.into_inner();

    let start_index = vm.get_instructions().len();

    compile_expr(vm, inner.next().unwrap());

    let i = Ins {
        code: ByteCode::JumpIfFalse,
        arg: 0
    };

    vm.add_instruction(i);

    let jump_index = vm.get_instructions().len() - 1;

    compile_scope(vm, inner.next().unwrap());

    let i = Ins {
        code: ByteCode::Jump,
        arg: start_index as u32
    };

    vm.add_instruction(i);

    vm.mod_arg(jump_index, vm.get_instructions().len() as u32);
}

fn compile_if_stmt(vm: &mut Vm, ast: Pair<Rule>) {
    log::debug!("compile_if_stmt");

    let mut inner = ast.into_inner();

    compile_expr(vm, inner.next().unwrap());

    let i = Ins {
        code: ByteCode::JumpIfFalse,
        arg: 0
    };

    vm.add_instruction(i);

    let jump_index = vm.get_instructions().len() - 1;

    compile_stmts(vm, inner.next().unwrap());

    vm.mod_arg(jump_index, vm.get_instructions().len() as u32);
}

fn compile_arg(vm: &mut Vm, ast: Pair<Rule>) {
    log::debug!("compile_arg");

    let mut inner = ast.into_inner();

    compile_expr(vm, inner.next().unwrap());
}

fn compile_call_stmt(vm: &mut Vm, ast: Pair<Rule>) {
    log::debug!("compile_call_stmt");

    let mut inner = ast.into_inner();

    compile_expr(vm, inner.next().unwrap());
    
    let mut number_of_args = 0;

    for t in inner {
        compile_arg(vm, t);
        number_of_args += 1;
    }

    let i = Ins{
        code: ByteCode::Call,
        arg: number_of_args
    };

    vm.add_instruction(i);
}

fn compile_stmt(vm: &mut Vm, ast: Pair<Rule>) { 
    log::debug!("compile_stmt");

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
    log::debug!("compile stmts");

    for t in ast.into_inner() {
        match t.as_rule() {
            Rule::stmt => compile_stmt(vm, t),
            _ => {}
        }
    }
}

fn compile_file(vm: &mut Vm, ast: Pair<Rule>) {
    log::debug!("compile file");

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
    log::debug!("compile");

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