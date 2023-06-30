use crate::opcode::OpCode;
use crate::parser::ast;
use crate::code_obj::CodeObject;
use crate::value::Value;


pub fn compile_module(module: ast::Module) -> CodeObject {
    let mut co = CodeObject { code: vec![], consts: vec![] };

    for stmt in module.stmts {
        compile_stmt(&mut co, stmt);
    }
    co.code.push(OpCode::Print as u8);
    co.code.push(OpCode::Halt as u8);

    println!("CO: {:#?}", co);

    co
}


fn compile_stmt(co: &mut CodeObject, stmt: ast::Stmt) {
    match stmt {
        ast::Stmt::Expr(expr) => compile_expr(co, expr),
        _ => unimplemented!(),
    }
}


fn compile_expr(co: &mut CodeObject, expr: ast::Expr) {
    match expr {
        ast::Expr::Binary{lhs, op, rhs} => {
            compile_expr(co, *lhs);
            compile_expr(co, *rhs);
            match op {
                ast::BinOp::Add => {
                    co.code.push(OpCode::BinaryAdd as u8);
                },
                ast::BinOp::Mul => {
                    co.code.push(OpCode::BinaryMul as u8);
                }
                _ => unimplemented!(),
            }
        },
        ast::Expr::Integer(x) => {
            let value = Value::Integer(x);
            let const_index = co.consts.len();
            co.consts.push(value);
            co.code.push(OpCode::Push as u8);
            co.code.push(const_index as u8);
        },
        _ => unimplemented!(),
    }
}