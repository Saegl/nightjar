#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    // Stack Manipulation
    Push,
    Pop,
    Copy,

    // Debug
    Print,
    
    // Control flow
    JumpNonzero,
    Halt,

    // Math operations
    BinaryAdd,
    BinarySub,
    BinaryMul,
    BinaryDiv,

    // Compare
    BinaryGT, // >
    BinaryLT, // <
    BinaryET, // ==
    BinaryNE, // !=
    BinaryGE, // >=
    BinaryLE, // <=
}

impl OpCode {
    pub fn from_u8(val: u8) -> Self {
        unsafe { std::mem::transmute(val)}
    }
}
