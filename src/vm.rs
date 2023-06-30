use crate::code_obj::CodeObject;
use crate::value::Value;
use crate::opcode::OpCode;
use crate::opcode::OpCode::*;

pub struct VM {
    pc: usize,
    stack: Vec<Value>,
    vars: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            pc: 0,
            stack: vec![],
            vars: vec![],
        }
    }
    pub fn print_state(&self) {
        println!("PC: {}", self.pc);
        println!("stack: {:?}", self.stack);
        println!("vars: {:?}", self.vars);
    }
    pub fn run(&mut self, co: CodeObject) {        
        loop {
            let opcode = OpCode::from_u8(co.code[self.pc]);
            // println!("OPCODE PC {}", pc);
            self.pc += 1;

            match opcode {
                push_const => {
                    let const_index = co.code[self.pc] as usize;
                    self.pc += 1;
                    self.stack.push(co.consts[const_index].clone());
                }
                push_var => {
                    let var_index = co.code[self.pc] as usize;
                    self.pc += 1;
                    self.stack.push(self.vars[var_index].clone());
                }
                pop => {
                    self.stack.pop().unwrap();
                },
                copy => {
                    self.stack.push(self.stack[self.stack.len() - 1].clone());
                }
                store_var => {
                    let var_index = co.code[self.pc] as usize;
                    self.pc += 1;
                    let topval = self.stack.pop().unwrap();
                    if var_index == self.vars.len() {
                        self.vars.push(topval);
                    } else if var_index < self.vars.len() {
                        self.vars[var_index] = topval;
                    } else {
                        panic!("Bad bytecode")
                    }
                }
                print => {
                    println!("{:?}", self.stack.pop().unwrap());
                },
                jmp_nonzero => {
                    let top = self.stack[self.stack.len() - 1].clone();
                    if top.__ne__(Value::Integer(0)).as_bool() {
                        let jump_position = co.code[self.pc];
                        self.pc = jump_position as usize;
                    } else {
                        self.pc += 1;
                    }
                }
                halt => {
                    break;
                },
                add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.__add__(b));
                },
                sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.__sub__(b));
                },
                mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.__mul__(b));
                },
                div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.__div__(b));
                },
                gt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.__gt__(b));
                },
                lt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.__lt__(b));
                },
                et => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.__et__(b));
                },
                ne => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.__ne__(b));
                },
                ge => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.__ge__(b));
                },
                le => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.__le__(b));
                },
            }
        }
    }
}
