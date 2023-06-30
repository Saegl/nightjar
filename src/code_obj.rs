use crate::value::Value;
use crate::opcode::OpCode;

#[derive(Debug)]
pub struct CodeObject {
    pub code: Vec<u8>,
    pub consts: Vec<Value>,
}


pub fn dis(co: &CodeObject) {
    println!("Disassembler:");

    let mut is_oparg = false;
    for (ind, code) in co.code.iter().enumerate() {
        if is_oparg {
            let value = &co.consts[*code as usize];
            println!("{}: <oparg [{}] = {:?}>", ind, *code, value);
            is_oparg = false;
        } else {
            let opcode = OpCode::from_u8(*code);
            println!("{}: {:?}", ind, opcode);
            is_oparg = opcode.has_oparg();
        }
    }

    println!("Consts:");
    for (ind, value) in co.consts.iter().enumerate() {
        println!("{}: {:?}", ind, value);
    }
    println!();
}
