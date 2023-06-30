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
                OpCode::push => {
                    let const_index = co.code[pc];
                    pc += 1;
                    stack.push(co.consts[const_index as usize].clone());
                }
                OpCode::pop => {
                    stack.pop().unwrap();
                },
                OpCode::copy => {
                    stack.push(stack[stack.len() - 1].clone());
                }
                OpCode::print => {
                    println!("{:?}", stack.pop().unwrap());
                },
                OpCode::jmp_nonzero => {
                    let top = stack[stack.len() - 1].clone();
                    if top.__ne__(Value::Integer(0)).as_bool() {
                        let jump_position = co.code[pc];
                        pc = jump_position as usize;
                    } else {
                        pc += 1;
                    }
                }
                OpCode::halt => {
                    break;
                },
                OpCode::add => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__add__(b));
                },
                OpCode::sub => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__sub__(b));
                },
                OpCode::mul => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__mul__(b));
                },
                OpCode::div => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__div__(b));
                },
                OpCode::gt => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__gt__(b));
                },
                OpCode::lt => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__lt__(b));
                },
                OpCode::et => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__et__(b));
                },
                OpCode::ne => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__ne__(b));
                },
                OpCode::ge => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__ge__(b));
                },
                OpCode::le => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.__le__(b));
                },
            }
        }
    }
}
