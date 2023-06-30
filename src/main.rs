mod vm;
mod parser;
mod compiler;
mod code_obj;
mod value;
mod opcode;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("Provide file to execute")
    } else {
        let file_path = &args[1];
        let ast = parser::parse_file(file_path);
        let co = compiler::compile_module(ast);
        let vm = vm::VM {};
        vm.run(co);
    }
}
