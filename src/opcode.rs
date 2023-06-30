#[repr(u8)]
#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub enum OpCode {
    // Stack Manipulation
    push,
    pop,
    copy,

    // Debug
    print,
    
    // Control flow
    jmp_nonzero,
    halt,

    // Math operations
    add,
    sub,
    mul,
    div,

    // Compare
    gt, // >
    lt, // <
    et, // ==
    ne, // !=
    ge, // >=
    le, // <=
}


impl OpCode {
    pub fn from_u8(val: u8) -> Self {
        unsafe { std::mem::transmute(val)}
    }
    pub fn has_oparg(&self) -> bool {
        let opcodes_with_oparg = vec![OpCode::push];
        opcodes_with_oparg.contains(self)
    }
}
