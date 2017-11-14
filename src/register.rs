pub struct RegisterFile {
    pub pc: Register,
    pub gpr: [Register; 32],
}


impl RegisterFile {
    pub fn new(pc: u32) -> RegisterFile {
        let mut reg_file = RegisterFile {
            pc: Register::new(pc, true),
            gpr: [Register::new(0, true); 32],
        };
        reg_file.gpr[0] = Register::new(0, false); // reinit x0 as read-only

        reg_file
    }
}


#[derive(Clone, Copy)]
pub struct Register {
    value: u32,
    is_writable: bool,
}


impl Register {
    pub fn new(value: u32, is_writable: bool) -> Register {
        Register { value, is_writable }
    }

    pub fn read(&self) -> u32 {
        self.value
    }

    pub fn write(&mut self, value: u32) {
        if self.is_writable {
            self.value = value;
        }
    }
}
