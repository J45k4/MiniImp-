use std::{collections::{HashMap}};

use crate::bytecode::{Ins, ByteCode, EQUAL_TO_OP, SMALLER_THAN_OP, GREATER_THAN_OP, LOGICAL_AND};

pub enum Action {
    Sleep(u32),
    Line(u32, u32, u32, u32),
    Rectangle(u32, u32, u32, u32),
    Circle(u32, u32, u32),
}

pub enum VmRunResult {
    Continue,
    Actions(Vec<Action>),
    Stop
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Print,
    Sleep,
    None,
    Empty,
    Line,
    Rectangle,
    Circle,
}

#[derive(Debug)]
pub struct Vm {
    pc: usize,
    stack: Vec<Value>,
    instuctions: Vec<Ins>,
    values: Vec<Value>,
    identifier_map: HashMap<String, u32>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            pc: 0,
            stack: vec![],
            instuctions: vec![],
            values: vec![],
            identifier_map: HashMap::new(),
        }
    }

    pub fn get_instructions(&self) -> &Vec<Ins> {
        &self.instuctions
    }

    pub fn mod_arg(&mut self, index: usize, arg: u32) {
        self.instuctions[index].arg = arg;
    }

    pub fn store(&mut self, key: u32, v: Value) {
        self.values[usize::try_from(key).unwrap()] = v;
    }

    pub fn store_new(&mut self, v: Value) -> u32 {
        self.values.push(v);

        u32::try_from(self.values.len() - 1).unwrap()
    }

    pub fn store_identifier(&mut self, name: &str) -> u32 {
        match self.identifier_map.get(name) {
            Some(v) => {
                v.to_owned()
            },
            None => {
                let val = match name {
                    "print" => Value::Print,
                    "sleep" => Value::Sleep,
                    "line" => Value::Line,
                    "rectangle" => Value::Rectangle,
                    "circle" => Value::Circle,
                    _ => Value::Empty
                };

                self.values.push(val);

                let next_id = u32::try_from(self.values.len() - 1).unwrap();

                self.identifier_map.insert(name.to_string(), next_id);

                next_id
            }
        }
    }

    pub fn load(&self, id: u32) -> Value {
        let index = usize::try_from(id).unwrap();

        self.values[index].clone()
    }

    pub fn add_instruction(&mut self, ins: Ins) {
        self.instuctions.push(ins);
    }

    pub fn work(&mut self) -> VmRunResult {
        let mut actions = vec![];

        let len = self.instuctions.len();

        while self.pc < len {
            let pc = self.pc;
            self.pc += 1;
    
            let ins = &self.instuctions[pc];
    
            match ins.code {
                ByteCode::Nope => todo!(),
                ByteCode::LoadConst => {
                    let v = self.load(ins.arg);
    
                    self.stack.push(v);
                },
                ByteCode::Store => {
                    let v = self.stack.pop().unwrap();
    
                    println!("storing value {:?}", v);
    
                    self.store(ins.arg, v);
                },
                ByteCode::Load => {
                    let v = self.load(ins.arg);
    
                    self.stack.push(v);
                },
                ByteCode::BinMul => todo!(),
                ByteCode::BinAdd => {
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
                ByteCode::BinMinus => todo!(),
                ByteCode::BinDivide => todo!(),
                ByteCode::JumpIfTrue => todo!(),
                ByteCode::JumpIfFalse => {
                    let tos = self.stack.pop().unwrap();
    
                    match tos {
                        Value::Boolean(true) => {},
                        Value::Number(1.0..) => {}
                        _ => {
                            self.pc = ins.arg as usize;
    
                            println!("jumping to {}", ins.arg);
                        }
                    };
                },
                ByteCode::ReturnValue => todo!(),
                ByteCode::Call => {    
                    let mut args = vec![];
    
                    for i in 0..ins.arg {
                        let v = self.stack.pop().unwrap();
                        args.push(v);
                    }

                    let v = self.stack.pop().unwrap();
    
                    match v {
                        Value::Print => {
                            println!("{:?}", args);
                        },
                        Value::Line => {    
                            let mut args = args.into_iter().rev();
    
                            let x = args.next().unwrap();
                            let y = args.next().unwrap();
                            let w = args.next().unwrap();
                            let h = args.next().unwrap();
    
                            match (x, y, w, h) {
                                (Value::Number(x), Value::Number(y), Value::Number(w), Value::Number(h)) => {    
                                    actions.push(Action::Line(x as u32, y as u32, w as u32, h as u32));
                                },
                                _ => {
                                    println!("invalid line args");
                                }
                            }
                        },
                        Value::Sleep => {
                            if args.len() != 1 {
                                panic!("sleep needs 1 argument");
                            }
    
                            match args[0] {
                                Value::Number(n) => {
                                    println!("sleeping for {}", n);
    
                                    actions.push(Action::Sleep(n as u32));
    
                                    return VmRunResult::Actions(actions);
                                },
                                _ => panic!("sleep needs a number")
                            }
                        },
                        Value::Rectangle => {
                            if args.len() > 4 {
                                panic!("rectangle needs 4 arguments");
                            }

                            let mut args = args.into_iter().rev();
    
                            let x = args.next().unwrap();
                            let y = args.next().unwrap();
                            let w = args.next().unwrap();
                            let h = args.next().unwrap();
    
                            match (x, y, w, h) {
                                (Value::Number(x), Value::Number(y), Value::Number(w), Value::Number(h)) => {    
                                    actions.push(Action::Rectangle(x as u32, y as u32, w as u32, h as u32));
                                },
                                _ => {
                                    println!("invalid line args");
                                }
                            }
                        },
                        Value::Circle => {
                            if args.len() > 3 {
                                panic!("circle needs 3 arguments");
                            }

                            let mut args = args.into_iter().rev();

                            let x = args.next().unwrap();
                            let y = args.next().unwrap();
                            let r = args.next().unwrap();

                            match (x, y, r) {
                                (Value::Number(x), Value::Number(y), Value::Number(r)) => {    
                                    actions.push(Action::Circle(x as u32, y as u32, r as u32));
                                },
                                _ => {
                                    println!("invalid line args");
                                }
                            }
                        }
                        _ => unimplemented!()
                    }
                },
                ByteCode::CmpEq => {
                    let tos = self.stack.pop().unwrap();
                    let tos1 = self.stack.pop().unwrap();
    
                    match (tos, tos1, ins.arg) {
                        (Value::Number(n2), Value::Number(n1), op) => {
                            let result = match op {
                                EQUAL_TO_OP => n1 == n2,
                                SMALLER_THAN_OP => n1 < n2,
                                GREATER_THAN_OP => n1 > n2,
                                NOT_EQUAL_TO_OP => n1 != n2,
                                GREATER_THAN_EQUAL_TO_OP => n1 >= n2,
                                SMALLER_THAN_EQUAL_TO_OP => n1 <= n2,
                                // LOGICAL_AND => n2 && n1,
                                // LOGICAL_OR => n2 || n1,
                                _ => unimplemented!()
                            };
    
                            self.stack.push(Value::Boolean(result));
                        },
                        _ => unimplemented!()
                    }
                },
                ByteCode::Jump => {
                    self.pc = ins.arg as usize;
    
                    println!("jumping to {}", ins.arg);
                },
            };
        }

        VmRunResult::Stop 
    }
}