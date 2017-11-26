//! 32-bit register and RV32I register file.


/// A complete RV32I register file.
///
/// Holds 32 general purpose registers and a program counter register.
#[derive(Debug)]
pub struct RegisterFile {
    pub pc: Register,
    pub gpr: [Register; 32],
}


impl RegisterFile {
    /// Constructs a new `RegisterFile`.
    pub fn new(pc: u32) -> RegisterFile {
        let mut reg_file = RegisterFile {
            pc: Register::new(pc, true),
            gpr: [Register::new(0, true); 32],
        };
        reg_file.gpr[0] = Register::new(0, false); // reinit x0 as read-only

        reg_file
    }
}


/// A write-protectable register.
#[derive(Clone, Copy, Debug)]
pub struct Register {
    /// The current register value.
    value: u32,

    /// If false, writing to the register has no effect.
    is_writable: bool,
}


impl Register {
    /// Constructs a new `Register`.
    pub fn new(value: u32, is_writable: bool) -> Register {
        Register { value, is_writable }
    }

    /// Reads the register's value.
    pub fn read(&self) -> u32 {
        self.value
    }

    /// Writes `value` to the register if it's writable, otherwise no effect.
    pub fn write(&mut self, value: u32) {
        if self.is_writable {
            self.value = value;
        }
    }
}
