use std::collections::{HashMap};

use crate::bytecode::Ins;

#[derive(Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool)
}

pub struct Scope {
    variables: HashMap<u32, Value>
}

#[derive(Debug)]
pub struct Vm {
    pc: u32,
    stack: Vec<u32>,
    instuctions: Vec<Ins>,
    constants: Vec<Value>,
    globals: Vec<Value>,
    identifier_map: HashMap<String, u32>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            pc: 0,
            stack: vec![],
            instuctions: vec![],
            constants: vec![],
            globals: vec![],
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

    pub fn store_global(&mut self, key: u32, v: Value) {
        self.globals.push(v);

        u32::try_from(self.globals.len() - 1).unwrap();
    }

    pub fn get_identifier_id(&mut self, name: &str) -> u32 {
        match self.identifier_map.get(name) {
            Some(v) => {
                v.to_owned()
            },
            None => {
                let next_id = u32::try_from(self.globals.len()).unwrap();

                self.identifier_map.insert(name.to_string(), next_id);

                next_id
            }
        }
    }

    pub fn add_instruction(&mut self, ins: Ins) {
        self.instuctions.push(ins);
    }

    pub fn run(&mut self) {

    }
}