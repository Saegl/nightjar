#[repr(u8)]
#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub enum OpCode {
    // Stack Manipulation
    push_const,
    push_var,
    pop,
    copy,

    store_var,

    // Debug
    print,
    
    // Control flow
    jmp,
    jmp_nonzero,
    pop_jmp_ifzero,
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
}
