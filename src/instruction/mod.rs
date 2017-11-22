//! Instruction decode stage.


use alu::{AluOp, AluSrc};
use consts;

pub mod decoder;


/// A single machine instruction.
#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    value: u32,

    /// Category of the instruction, e.g., load, branch, or op
    pub opcode: Opcode,

    /// Format associated with the opcode, e.g., R-type or I-type
    pub format: Format,

    /// Struct for accessing the subfields' bits
    pub fields: Fields,

    /// Instruction's mnemonic, e.g., JAL, XOR, or SRA
    pub function: Function,

    /// Control unit semantics (dictates control lines to be {de}asserted)
    pub semantics: Semantics,
}


impl Instruction {
    /// Constructs a new `Instruction`.
    pub fn new(value: u32) -> Instruction {
        let opcode = int_to_opcode(value);
        let format = opcode_to_format(opcode);
        let fields = Fields::default();
        let function = Function::Addi;
        let semantics = Semantics::default();
        let mut insn = Instruction {
            value,
            opcode,
            format,
            fields,
            function,
            semantics,
        };
        decoder::decode(&mut insn);

        insn
    }

    /// Returns the original instruction integer.
    pub fn as_u32(&self) -> u32 {
        self.value
    }
}


impl Default for Instruction {
    /// Constructs a canonical NOP encoded as ADDI x0, x0, 0.
    fn default() -> Instruction {
        Instruction::new(0x00_00_00_13)
    }
}


/// Extracts the opcode from a raw instruction integer.
fn int_to_opcode(insn: u32) -> Opcode {
    let opcode = insn & consts::OPCODE_MASK;
    match opcode {
        0b01_101_11 => Opcode::Lui,
        0b00_101_11 => Opcode::AuiPc,
        0b11_011_11 => Opcode::Jal,
        0b11_001_11 => Opcode::Jalr,
        0b11_000_11 => Opcode::Branch,
        0b00_000_11 => Opcode::Load,
        0b01_000_11 => Opcode::Store,
        0b01_100_11 => Opcode::Op,
        0b00_100_11 => Opcode::OpImm,
        0b01_111_11 => Opcode::Halt,
        _ => panic!("Unknown opcode {:#09b}", opcode),
    }
}


/// Maps an opcode to its instruction format.
fn opcode_to_format(opcode: Opcode) -> Format {
    match opcode {
        Opcode::Lui => Format::U,
        Opcode::AuiPc => Format::U,
        Opcode::Jal => Format::J,
        Opcode::Jalr => Format::I,
        Opcode::Branch => Format::B,
        Opcode::Load => Format::I,
        Opcode::Store => Format::S,
        Opcode::Op => Format::R,
        Opcode::OpImm => Format::I,
        Opcode::Halt => Format::U,  // Do minimal parsing; Halt has no format
    }
}


/// RISC-V 32I fields (shamt -> imm).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Fields {
    pub rs1: Option<u32>,
    pub rs2: Option<u32>,
    pub rd: Option<u32>,
    pub funct3: Option<u32>,
    pub funct7: Option<u32>,
    pub imm: Option<u32>,
    pub opcode: Option<u32>,
}


/// RISC-V 32I opcodes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    Lui,
    AuiPc,
    Jal,
    Jalr,
    Branch,
    Load,
    Store,
    Op,
    OpImm,
    Halt,
}


/// RISC-V 32I instruction formats.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Format {
    R,
    I,
    S,
    B,
    U,
    J,
}


/// RISC-V 32I mnemonics.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Function {
    /// Load upper immediate
    Lui,
    /// Add upper immediate to PC
    AuiPc,
    // Jumps
    /// Jump and link
    Jal,
    /// Jump and link register
    Jalr,
    // Branches
    /// Branch if equal
    Beq,
    /// Branch if not equal
    Bne,
    /// Branch if less than
    Blt,
    /// Branch if greater or equal
    Bge,
    /// Branch if less than (unsigned)
    Bltu,
    /// Branch if greater or equal (unsigned)
    Bgeu,
    // Loads
    /// Load byte
    Lb,
    /// Load halfword
    Lh,
    /// Load word
    Lw,
    /// Load byte (unsigned)
    Lbu,
    /// Load halfword (unsigned)
    Lhu,
    // Stores
    /// Store byte
    Sb,
    /// Store halfword
    Sh,
    /// Store word
    Sw,
    // Operations on immediates
    /// Add immediate
    Addi,
    /// Set less than immediate
    Slti,
    /// Set less than immediate (unsigned)
    Sltiu,
    /// Exclusive or immediate
    Xori,
    /// Logical Or immediate
    Ori,
    /// Logical And immediate
    Andi,
    /// Shift left logical immediate
    Slli,
    /// Shift right logical immediate
    Srli,
    /// Shift right arithmetic immediate
    Srai,
    // Operations on registers
    /// Add
    Add,
    /// Subtract
    Sub,
    /// Shift left logical
    Sll,
    /// Set less than
    Slt,
    /// Set less than unsigned
    Sltu,
    /// Exclusive or
    Xor,
    /// Shift right logical
    Srl,
    /// Shift right arithmetic
    Sra,
    /// Logical Or
    Or,
    /// Logical And
    And,
    /// Halt simulator
    Halt,
}


/// Control unit semantics
#[derive(Clone, Copy, Debug, Default)]
pub struct Semantics {
    pub branch: bool,
    pub mem_read: bool,
    pub mem_to_reg: bool,
    pub alu_op: AluOp,
    pub mem_write: bool,
    pub alu_src: AluSrc,
    pub reg_write: bool,
    pub mem_size: usize,
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Instruction::default() should be a NOP
    #[test]
    fn nop() {
        let insn = Instruction::default();
        assert_eq!(insn.fields.rd, Some(0));
        assert_eq!(insn.fields.rs1, Some(0));
        assert_eq!(insn.fields.rs2, None);
        assert_eq!(insn.fields.imm, Some(0));
        assert!(!insn.semantics.branch);
        assert!(!insn.semantics.mem_read);
        assert!(!insn.semantics.mem_to_reg);
        assert_eq!(insn.semantics.alu_op, AluOp::Add);
        assert_eq!(insn.semantics.alu_src, AluSrc::Imm);
        assert!(insn.semantics.reg_write);
    }

}
