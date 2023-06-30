mod vm;
mod parser;
mod compiler;
mod code_obj;
mod value;
mod opcode;


fn main() {
    // let co = vm::CodeObject {
    //     code: vec![
    //         vm::OpCode::Push as u8,
    //         0,
    //         vm::OpCode::Copy as u8,
    //         vm::OpCode::Print as u8,
    //         vm::OpCode::Push as u8,
    //         1,
    //         vm::OpCode::BinarySub as u8,
    //         vm::OpCode::JumpNonzero as u8,
    //         2,
    //         vm::OpCode::Print as u8,
    //         vm::OpCode::Halt as u8,
    //         opcode as u8,
    //     ],
    //     consts: vec![
    //         vm::Value::Integer(5), vm::Value::Integer(1)
    //     ],
    // };

    // let vm = vm::VM {};

    // vm.run(co);
    // println!("Size of Value {}", std::mem::size_of::<vm::Value>());

    // let source = Rc::new(String::from("Alisher"));
    // let mut ps = parser::ParserSource::from(String::from("banana"));

    // let parse_b_char = parser::char_parser('b');
    // let res = parse_b_char(&mut ps);
    let ast = parser::parse_file();
    let co = compiler::compile_module(ast);

    let vm = vm::VM {};
    vm.run(co);
    // println!("Compiles");
}
