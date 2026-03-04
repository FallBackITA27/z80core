pub struct LOAD_REGISTER_NUMBER;
impl super::Instruction<2> for LOAD_REGISTER_NUMBER {
    const INSTRUCTION_BITS: [u8; 2] = [0b00000110, 0];
    const MASK_BITS: [u8; 2] = [0b11000111, 0];
    const STATES: usize = 7;
    const CYCLES: usize = 2;
}

pub struct LOAD_REGISTER_HL;
impl super::Instruction<1> for LOAD_REGISTER_HL {
    const INSTRUCTION_BITS: [u8; 1] = [0b01000110];
    const MASK_BITS: [u8; 1] = [0b11000111];
    const STATES: usize = 7;
    const CYCLES: usize = 2;
}

pub struct LOAD_REGISTER_IX;
impl super::Instruction<3> for LOAD_REGISTER_IX {
    const INSTRUCTION_BITS: [u8; 3] = [0b11011101, 0b01000110, 00000000];
    const MASK_BITS: [u8; 3] = [0b11111111, 0b11000111, 00000000];
    const STATES: usize = 19;
    const CYCLES: usize = 5;
}

pub struct LOAD_REGISTER_IY;
impl super::Instruction<3> for LOAD_REGISTER_IY {
    const INSTRUCTION_BITS: [u8; 3] = [0b11111111, 0b01000110, 0b00000000];
    const MASK_BITS: [u8; 3] = [0b11111111, 0b11000111, 0b00000000];
    const STATES: usize = 19;
    const CYCLES: usize = 5;
}

pub struct LOAD_IX_REGISTER;
impl super::Instruction<3> for LOAD_IX_REGISTER {
    const INSTRUCTION_BITS: [u8; 3] = [0b11011101, 0b01110000, 0b00000000];
    const MASK_BITS: [u8; 3] = [0b11111111, 0b11111000, 0b00000000];
    const STATES: usize = 19;
    const CYCLES: usize = 5;
}

pub struct LOAD_IY_REGISTER;
impl super::Instruction<3> for LOAD_IY_REGISTER {
    const INSTRUCTION_BITS: [u8; 3] = [0b11111101, 0b01110000, 0b00000000];
    const MASK_BITS: [u8; 3] = [0b11111111, 0b11111000, 0b00000000];
    const STATES: usize = 19;
    const CYCLES: usize = 5;
}

pub struct LOAD_HL_REGISTER;
impl super::Instruction<1> for LOAD_HL_REGISTER {
    const INSTRUCTION_BITS: [u8; 1] = [0b01110000];
    const MASK_BITS: [u8; 1] = [0b11111000];
    const STATES: usize = 7;
    const CYCLES: usize = 2;
}
