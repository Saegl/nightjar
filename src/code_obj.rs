use crate::value::Value;
use crate::opcode::OpCode;

#[derive(Debug)]
pub struct CodeObject {
    pub code: Vec<u8>,
    pub consts: Vec<Value>,
}

impl CodeObject {
    pub fn new_empty() -> Self {
        CodeObject { code: vec![], consts: vec![] }
    }
}


pub fn dis(co: &CodeObject) {
    println!("Disassembler:");

    let mut prev = OpCode::halt;
    for (ind, code) in co.code.iter().enumerate() {
        if prev == OpCode::push_const {
            let value = &co.consts[*code as usize];
            println!("{}: <const {:?} at {}>", ind, value, code);
            prev = OpCode::halt
        } else if prev == OpCode::push_var {
            println!("{}: <const at {}>", ind, code);
            prev = OpCode::halt
        } else if prev == OpCode::store_var {
            println!("{}: <pos {}>", ind, code);
            prev = OpCode::halt
        } else {
            let opcode = OpCode::from_u8(*code);
            println!("{}: {:?}", ind, opcode);
            prev = opcode
        }
    }

    println!("Consts:");
    for (ind, value) in co.consts.iter().enumerate() {
        println!("{}: {:?}", ind, value);
    }
    println!();
}
