use crate::code_obj::CodeObject;
use crate::value::Value;
use crate::opcode::OpCode;

pub struct VM {}

impl VM {
    pub fn run(&self, co: CodeObject) {
        let mut pc = 0;
        let mut stack: Vec<Value> = vec![];
        
        loop {
            let opcode = OpCode::from_u8(co.code[pc]);
            pc += 1;

            match opcode {
                OpCode::Push => {
                    let const_index = co.code[pc];
                    pc += 1;
                    stack.push(co.consts[const_index as usize].clone());
                }
                OpCode::Pop => {
                    stack.pop().unwrap();
                },
                OpCode::Copy => {
                    stack.push(stack[stack.len() - 1].clone());
                }
                OpCode::Print => {
                    println!("{:?}", stack.pop().unwrap());
                },
                OpCode::JumpNonzero => {
                    let top = stack[stack.len() - 1].clone();
                    if top.__ne__(Value::Integer(0)).as_bool() {
                        let jump_position = co.code[pc];
                        pc = jump_position as usize;
                    } else {
                        pc += 1;
                    }
                }
                OpCode::Halt => {
                    break;
                },
                OpCode::BinaryAdd => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__add__(b));
                },
                OpCode::BinarySub => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__sub__(b));
                },
                OpCode::BinaryMul => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__mul__(b));
                },
                OpCode::BinaryDiv => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__div__(b));
                },
                OpCode::BinaryGT => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__gt__(b));
                },
                OpCode::BinaryLT => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__lt__(b));
                },
                OpCode::BinaryET => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__et__(b));
                },
                OpCode::BinaryNE => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__ne__(b));
                },
                OpCode::BinaryGE => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__ge__(b));
                },
                OpCode::BinaryLE => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__le__(b));
                },
            }
        }
    }
}
