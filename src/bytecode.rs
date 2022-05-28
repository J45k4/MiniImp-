
#[derive(Debug)]
pub enum ByteCode {
    Nope,
    Load,
    LoadConst,
    LoadGlobal,
    LoadFast,
    StoreGlobal,
    StoreFast,
    BinMul,
    BinAdd,
    BinMinus,
    BinDivide,
    Store,
    JumpIfTrue,
    JumpIfFalse,
    ReturnValue
}

#[derive(Debug)]
pub struct Ins {
    pub code: ByteCode,
    pub arg: u32,
}