

pub enum ByteCode {
    Nope,
    Load,
    LoadCost,
    Store,
    JumpIfTrue,
    JumpIfFalse
}

pub struct Ins {
    pub code: ByteCode,
    pub arg: u32,
}