
#[derive(Debug)]
pub enum ByteCode {
    Nope,
    Load,
    Store,
    LoadConst,
    BinMul,
    BinAdd,
    BinMinus,
    BinDivide,
    JumpIfTrue,
    JumpIfFalse,
    ReturnValue,
    Call
}

#[derive(Debug)]
pub struct Ins {
    pub code: ByteCode,
    pub arg: u32,
}