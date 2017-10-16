pub struct RegisterFile {
    pub pc: Register,
    pub gpr: [Register; 32]
}


impl RegisterFile {
    pub fn new(pc: u32) -> RegisterFile {
        RegisterFile {
            pc: Register::new(pc),
            gpr: [Register::new(0); 32]
        }
    }
}


#[derive(Clone, Copy)]
pub struct Register {
    value: u32
}


impl Register {
    pub fn new(value: u32) -> Register {
        Register {
            value
        }
    }

    pub fn read(&self) -> u32 {
        self.value
    }

    pub fn write(&mut self, value: u32) {
        self.value = value;
    }
}
