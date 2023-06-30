use std::collections::HashMap;

use crate::opcode::OpCode;
use crate::ast;
use crate::code_obj::CodeObject;
use crate::value::Value;


struct Vartable {
    table: HashMap<String, usize>,
}

impl Vartable {
    fn new() -> Self {
        Vartable {table: HashMap::new()}
    }

    fn insert(&mut self, name: &str) {
        let pos = self.table.len();
        self.table.insert(name.to_string(), pos);
    }

    fn get(&self, name: &str) -> u8 {
        *self.table.get(name).unwrap() as u8
    }
}

struct CompilerState {
    vartable: Vartable,
    co: CodeObject,
}

impl CompilerState {
    fn new_empty() -> Self {
        CompilerState { vartable: Vartable::new(), co: CodeObject::new_empty() }
    }
}


pub fn compile_module(module: &ast::Module) -> CodeObject {
    let mut cs = CompilerState::new_empty();

    for stmt in module.stmts.iter() {
        compile_stmt(&mut cs, stmt);
    }
    cs.co.code.push(OpCode::halt as u8);
    cs.co
}


fn compile_stmt(cs: &mut CompilerState, stmt: &ast::Stmt) {
    match stmt {
        ast::Stmt::Expr(expr) => compile_expr(cs, expr),
        ast::Stmt::VarDecl { name, value } => compile_vardecl(cs, name, value),
        _ => unimplemented!(),
    }
}


fn compile_vardecl(cs: &mut CompilerState, name: &str, value: &ast::Expr) {
    compile_expr(cs, value);
    cs.vartable.insert(name);
    cs.co.code.push(OpCode::store_var as u8);
    cs.co.code.push(cs.vartable.get(name))
}


fn compile_expr(cs: &mut CompilerState, expr: &ast::Expr) {
    match expr {
        ast::Expr::Binary{lhs, op, rhs} => {
            compile_expr(cs, &*lhs);
            compile_expr(cs, &*rhs);
            match op {
                ast::BinOp::Add => {
                    cs.co.code.push(OpCode::add as u8);
                },
                ast::BinOp::Mul => {
                    cs.co.code.push(OpCode::mul as u8);
                }
                _ => unimplemented!(),
            }
        },
        ast::Expr::Integer(x) => {
            let value = Value::Integer(*x);
            let const_index = cs.co.consts.len();
            cs.co.consts.push(value);
            cs.co.code.push(OpCode::push_const as u8);
            cs.co.code.push(const_index as u8);
        },
        ast::Expr::VarCall { name } => {
            cs.co.code.push(OpCode::push_var as u8);
            cs.co.code.push(cs.vartable.get(name));
        }
        _ => unimplemented!(),
    }
}