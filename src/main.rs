mod vm;
mod parser;
mod compiler;
mod code_obj;
mod value;
mod opcode;
pub mod ast;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("Nightjar programming language, usage: nightjar <path>")
    } else {
        let file_path = &args[1];
        let ast_verbose = args.contains(&"--ast".to_string());
        let dis_verbose = args.contains(&"--dis".to_string());
        let vm_verbose = args.contains(&"--vm".to_string());

        let ast = parser::parse_file(file_path);
        if ast_verbose {
            ast::pretty_print(&ast);
        }

        let co = compiler::compile_module(&ast);
        if dis_verbose {
            code_obj::dis(&co);
        }
        
        let mut vm = vm::VM::new();
        vm.run(co);
        if vm_verbose {
            println!("VM final state");
            vm.print_state();
        }
    }
}
