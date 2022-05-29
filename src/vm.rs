use std::{collections::{HashMap}};

use crate::bytecode::Ins;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
    Empty
}

#[derive(Debug)]
pub struct Vm {
    pc: usize,
    stack: Vec<Value>,
    instuctions: Vec<Ins>,
    constants: Vec<Value>,
    values: Vec<Value>,
    identifier_map: HashMap<String, u32>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            pc: 0,
            stack: vec![],
            instuctions: vec![],
            constants: vec![],
            values: vec![],
            identifier_map: HashMap::new(),
        }
    }

    pub fn get_instructions(&self) -> &Vec<Ins> {
        &self.instuctions
    }

    pub fn store_const(&mut self, v: Value) -> u32 {
        let next_id = u32::try_from(self.constants.len()).unwrap();

        self.constants.push(v);

        next_id
    }

    pub fn store(&mut self, key: u32, v: Value) {
        self.values[usize::try_from(key).unwrap()] = v;
    }

    pub fn store_new(&mut self, v: Value) -> u32 {
        let new_id = self.get_new_id();

        self.values[new_id] = v;

        u32::try_from(new_id).unwrap()
    }


    fn get_new_id(&mut self) -> usize {
        for i in 0..self.values.len() {
            if let Value::None = self.values[i] {
                return i;
            }
        }

        self.values.push(Value::Empty);

        self.values.len() - 1
    }

    pub fn store_identifier(&mut self, name: &str) -> u32 {
        match self.identifier_map.get(name) {
            Some(v) => {
                v.to_owned()
            },
            None => {
                let next_id = u32::try_from(self.get_new_id()).unwrap();

                self.identifier_map.insert(name.to_string(), next_id);

                next_id
            }
        }
    }

    pub fn load_global(&self, id: u32) -> Value {
        let index = usize::try_from(id).unwrap();

        self.values[index].clone()
    }

    pub fn load_const(&self, id: u32) -> Value {
        let index = usize::try_from(id).unwrap();

        self.constants[index].clone()
    }

    pub fn add_instruction(&mut self, ins: Ins) {
        self.instuctions.push(ins);
    }

    pub fn run(&mut self) {
        let len = self.instuctions.len();

        while self.pc <  len {
            let pc = self.pc;
            self.pc += 1;

            let ins = &self.instuctions[pc];

            match ins.code {
                crate::bytecode::ByteCode::Nope => todo!(),
                crate::bytecode::ByteCode::LoadConst => {
                    let v = self.load_const(ins.arg);

                    self.stack.push(v);
                },
                crate::bytecode::ByteCode::Store => {
                    let v = self.stack.pop().unwrap();

                    println!("storing value {:?}", v);

                    self.store(ins.arg, v);
                },
                crate::bytecode::ByteCode::Load => {
                    let v = self.load_global(ins.arg);

                    self.stack.push(v);
                },
                crate::bytecode::ByteCode::BinMul => todo!(),
                crate::bytecode::ByteCode::BinAdd => {
                    let tos = self.stack.pop().unwrap();
                    let tos1 = self.stack.pop().unwrap();

                    let tos_v = match tos {
                        Value::Number(n) => n,
                        _ => unimplemented!()
                    };

                    let tos1_v = match tos1 {
                        Value::Number(n) => n,
                        _ => unimplemented!()
                    };

                    let result = tos_v + tos1_v;

                    self.stack.push(Value::Number(result));
                },
                crate::bytecode::ByteCode::BinMinus => todo!(),
                crate::bytecode::ByteCode::BinDivide => todo!(),
                crate::bytecode::ByteCode::JumpIfTrue => todo!(),
                crate::bytecode::ByteCode::JumpIfFalse => todo!(),
                crate::bytecode::ByteCode::ReturnValue => todo!(),
                crate::bytecode::ByteCode::Call => {
                    println!("{:?}", self);
                },
            }
        }
    }
}