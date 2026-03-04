pub trait Instruction<const BYTE_LENGTH: usize> {
    const INSTRUCTION_BITS: [u8; BYTE_LENGTH];
    const MASK_BITS: [u8; BYTE_LENGTH];
    const STATES: usize;
    const CYCLES: usize;

    /// Returns input data if OK, else if instruction doesn't match, return None
    fn matches(data: [u8; BYTE_LENGTH]) -> Option<[u8; BYTE_LENGTH]> {
        let mut out_bytes: [u8; BYTE_LENGTH] = [0; BYTE_LENGTH];
        for byte_idx in 0..BYTE_LENGTH {
            match (data[byte_idx] & Self::MASK_BITS[byte_idx]) == Self::INSTRUCTION_BITS[byte_idx] {
                false => return None,
                true => out_bytes[byte_idx] = data[byte_idx] & !Self::MASK_BITS[byte_idx],
            }
        }
        Some(out_bytes)
    }
}

pub struct NOP;
impl Instruction<1> for NOP {
    const INSTRUCTION_BITS: [u8; 1] = [0];
    const MASK_BITS: [u8; 1] = [0b11111111];
    const STATES: usize = 4;
    const CYCLES: usize = 1;
}

pub mod load;

pub enum Instructions {
    LoadRegisterNumber(load::LOAD_REGISTER_NUMBER),
    LoadRegisterHL(load::LOAD_REGISTER_HL),
    LoadRegisterIX(load::LOAD_REGISTER_IX),
    LoadRegisterIY(load::LOAD_REGISTER_IY),
    LoadIXRegister(load::LOAD_IX_REGISTER),
    LoadIYRegister(load::LOAD_IY_REGISTER),
    LoadHLRegister(load::LOAD_HL_REGISTER),
    NoOp(NOP),
}
