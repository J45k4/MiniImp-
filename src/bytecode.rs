
#[derive(Debug)]
pub enum ByteCode {
    Load,
    Store,
    BinMul,
    BinAdd,
    BinMinus,
    BinDivide,
    Jump,
    JumpIfTrue,
    JumpIfFalse,
    ReturnValue,
    Call,
    CmpEq,
}

#[derive(Debug)]
pub struct Ins {
    pub code: ByteCode,
    pub arg: u32,
}

pub const SMALLER_THAN_OP: u32 = 0;
pub const GREATER_THAN_OP: u32 = 1;
pub const EQUAL_TO_OP: u32 = 2;
pub const NOT_EQUAL_TO_OP: u32 = 3;
pub const GREATER_THAN_EQUAL_TO_OP: u32 = 4;
pub const SMALLER_THAN_EQUAL_TO_OP: u32 = 5;
pub const LOGICAL_AND: u32 = 6;
pub const LOGICAL_OR: u32 = 7;