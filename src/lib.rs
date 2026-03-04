pub struct Z80State {
    memory: [u8; 0xFFFF],
    register_a: u8,
    register_f: u8,
    register_b: u8,
    register_c: u8,
    register_d: u8,
    register_e: u8,
    register_h: u8,
    register_l: u8,
    stack_pointer: u16,
    program_counter: u16,
    register_ix: u16,
    register_iy: u16,
}

pub mod instructions;

impl Z80State {
    pub fn get_a(&self) -> u8 {
        self.register_a
    }

    pub fn get_b(&self) -> u8 {
        self.register_b
    }

    pub fn get_c(&self) -> u8 {
        self.register_c
    }

    pub fn get_d(&self) -> u8 {
        self.register_d
    }

    pub fn get_e(&self) -> u8 {
        self.register_e
    }

    pub fn get_f(&self) -> u8 {
        self.register_f
    }

    pub fn get_h(&self) -> u8 {
        self.register_h
    }

    pub fn get_l(&self) -> u8 {
        self.register_l
    }

    pub fn get_af(&self) -> u16 {
        ((self.register_a as u16) << 8) & (self.register_f as u16)
    }

    pub fn get_bc(&self) -> u16 {
        ((self.register_b as u16) << 8) & (self.register_c as u16)
    }

    pub fn get_de(&self) -> u16 {
        ((self.register_d as u16) << 8) & (self.register_e as u16)
    }
    pub fn get_hl(&self) -> u16 {
        ((self.register_h as u16) << 8) & (self.register_l as u16)
    }

    pub fn get_sp(&self) -> u16 {
        self.stack_pointer
    }

    pub fn get_pc(&self) -> u16 {
        self.program_counter
    }

    pub fn get_ix(&self) -> u16 {
        self.register_ix
    }
    pub fn get_iy(&self) -> u16 {
        self.register_iy
    }

    pub fn get_register(&self, idx: u16) -> u8 {
        self.memory[idx as usize]
    }

    pub fn execute_instruction<const BYTE_LENGTH: usize>(
        &mut self,
        _instruction: impl instructions::Instruction<BYTE_LENGTH>,
    ) {
        self.program_counter += BYTE_LENGTH as u16;
    }
}

#[cfg(test)]
mod tests {}
