use std::{collections::{HashMap}};

use log::log_enabled;

use crate::bytecode::{Ins, ByteCode, EQUAL_TO_OP, SMALLER_THAN_OP, GREATER_THAN_OP, LOGICAL_AND, GREATER_THAN_EQUAL_TO_OP, NOT_EQUAL_TO_OP, SMALLER_THAN_EQUAL_TO_OP};

pub enum Action {
    Sleep(u32),
    Line(u32, u32, u32, u32, String),
    Rectangle(u32, u32, u32, u32, String),
    Circle(u32, u32, u32, String),
    Clear
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
    Empty,
    Line,
    Rectangle,
    Circle,
    Clear
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
                    "clear" => Value::Clear,
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
                ByteCode::Store => {
                    let v = self.stack.pop().unwrap();

                    log::debug!("store {} with value {:?}", ins.arg, v);
    
                    self.store(ins.arg, v);
                },
                ByteCode::Load => {
                    let v = self.load(ins.arg);

                    if log_enabled!(log::Level::Debug) {
                        match self.find_variable_name(ins.arg) {
                            Some(name) => {
                                log::debug!("load {} {:?}", name, v);
                            },
                            None => {
                                log::debug!("load {} {:?}", ins.arg, v);
                            }
                        }
                    }
    
                    self.stack.push(v);
                },
                ByteCode::BinMul |
                ByteCode::BinAdd |
                ByteCode::BinMinus |
                ByteCode::BinDivide => {
                    let tos = self.stack.pop().unwrap();
                    let tos1 = self.stack.pop().unwrap();

                    

                    let res = match (&ins.code, &tos1, &tos) {
                        (ByteCode::BinMul, Value::Number(a), Value::Number(b)) => {
                            Value::Number(a * b)
                        },
                        (ByteCode::BinAdd, Value::Number(a), Value::Number(b)) => {
                            Value::Number(a + b)
                        },
                        (ByteCode::BinMinus, Value::Number(a), Value::Number(b)) => {
                            Value::Number(a - b)
                        },
                        (ByteCode::BinDivide, Value::Number(a), Value::Number(b)) => {
                            Value::Number(a / b)
                        },
                        _ => {
                            panic!("invalid binary operation {:?} {:?} {:?}", tos, ins.code, tos1);
                        }
                    };

                    log::debug!("{:?} {:?} {:?} = {:?}", &tos1, ins.code, &tos, &res);

                    self.stack.push(res);
                },
                ByteCode::JumpIfFalse => {
                    let tos = self.stack.pop().unwrap();
    
                    match tos {
                        Value::Boolean(true) => {},
                        Value::Number(1.0..) => {}
                        _ => {
                            self.pc = ins.arg as usize;
                        }
                    };
                },
                ByteCode::Call => {    
                    let mut args = vec![];
    
                    for _ in 0..ins.arg {
                        let v = self.stack.pop().unwrap();
                        args.push(v);
                    }

                    let v = self.stack.pop().unwrap();
    
                    match v {
                        Value::Print => {
                            let args = args.into_iter().rev();

                            let p: String = args.map(|a| {
                                match a {
                                    Value::Number(n) => n.to_string(),
                                    Value::String(s) => s,
                                    Value::Boolean(b) => b.to_string(),
                                    _ => {
                                        panic!("arg does not support printing");
                                    }
                                }
                            }).collect::<Vec<String>>().join(" ");

                            println!("{}", p);
                        },
                        Value::Line => {   
                            if args.len() != 5 {
                                panic!("rectangle needs 5 arguments");
                            }
                            
                            let mut args = args.into_iter().rev();
    
                            let x = args.next().unwrap();
                            let y = args.next().unwrap();
                            let w = args.next().unwrap();
                            let h = args.next().unwrap();
                            let color = args.next().unwrap();
    
                            match (x, y, w, h, color) {
                                (Value::Number(x), Value::Number(y), 
                                Value::Number(w), Value::Number(h), Value::String(color)) => {    
                                    actions.push(Action::Line(x as u32, y as u32, w as u32, h as u32, color));
                                },
                                _ => {
                                    panic!("invalid line call args");
                                }
                            }
                        },
                        Value::Sleep => {
                            if args.len() != 1 {
                                panic!("sleep needs 1 argument");
                            }
    
                            match args[0] {
                                Value::Number(n) => {
                                    log::debug!("sleeping for {}", n);
    
                                    actions.push(Action::Sleep(n as u32));
    
                                    return VmRunResult::Actions(actions);
                                },
                                _ => panic!("sleep needs a number")
                            }
                        },
                        Value::Rectangle => {
                            if args.len() != 5 {
                                panic!("rectangle needs 5 arguments");
                            }

                            let mut args = args.into_iter().rev();
    
                            let x = args.next().unwrap();
                            let y = args.next().unwrap();
                            let w = args.next().unwrap();
                            let h = args.next().unwrap();
                            let color = args.next().unwrap();
    
                            match (x, y, w, h, color) {
                                (Value::Number(x), Value::Number(y), 
                                    Value::Number(w), Value::Number(h), Value::String(color)) => {    
                                    actions.push(Action::Rectangle(x as u32, y as u32, w as u32, h as u32, color));
                                },
                                _ => {
                                    panic!("invalid rectangle call args");
                                }
                            }
                        },
                        Value::Circle => {
                            if args.len() != 4 {
                                panic!("circle needs 4 arguments");
                            }

                            let mut args = args.into_iter().rev();

                            let x = args.next().unwrap();
                            let y = args.next().unwrap();
                            let r = args.next().unwrap();
                            let color = args.next().unwrap();

                            match (x, y, r, color) {
                                (Value::Number(x), Value::Number(y), Value::Number(r), Value::String(color)) => {    
                                    actions.push(Action::Circle(x as u32, y as u32, r as u32, color));
                                },
                                _ => {
                                    panic!("invalid circle call args");
                                }
                            }
                        },
                        Value::Clear => {
                            actions.push(Action::Clear);
                        },
                        _ => unimplemented!()
                    }
                },
                ByteCode::CmpEq => {
                    let tos = self.stack.pop().unwrap();
                    let tos1 = self.stack.pop().unwrap();
    
                    match (tos1, tos, ins.arg) {
                        (Value::Number(n1), Value::Number(n2), op) => {
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
    
                    log::debug!("jumping to {}", ins.arg);
                },
            };
        }

        VmRunResult::Stop 
    }

    fn find_variable_name(&self, index: u32) -> Option<&str> {
        for (key, value) in self.identifier_map.iter() {
            if value == &index {
                return Some(key);
            }
        }

        None
    }

    pub fn get_variable(&self, name: &str) -> Value {
        let index = self.identifier_map.get(name).unwrap();
        self.values.get(*index as usize).unwrap().clone()
    }

    pub fn disassemble(&self) {
        let mut pc = 0;

        while pc < self.instuctions.len() {
            let ins = &self.instuctions[pc];

            print!("Instruction: {:?} arg: {} ", ins.code, ins.arg);

            match ins.code {
                ByteCode::Load => {
                    if let Some(name) = self.find_variable_name(ins.arg) {
                        print!("{}", name);
                    } else {
                        print!("{:?}", self.load(ins.arg));
                    }
                },
                ByteCode::Store => {
                    let name = self.find_variable_name(ins.arg).unwrap();
                    print!("{:?}", name);
                },
                _ => {}
            };

            println!();

            pc += 1;
        }
    }
}