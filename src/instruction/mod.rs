pub mod decoder;
pub mod semantics;


pub const HALT: u32 = 0x3f;  /// Special simulator-only instruction
pub const OPCODE_MASK: u32 = 0x7f;


#[derive(Debug)]
pub struct Instruction {
    value: u32,

    /// The category of the instruction, e.g., Load, Branch, Op
    pub opcode: Opcode,

    /// The format associated with the opcode, e.g., R-type, I-type
    pub format: Format,

    /// Struct for accessing the subfields' bits
    pub fields: Fields,

    /// The specific instruction's function, e.g., Jal, Xor, Sra
    pub function: Function,

    /// Tells the rest of the processor what the instruction actually _does_
    pub semantics: Semantics,
}


impl Instruction {
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

    pub fn as_u32(&self) -> u32 {
        self.value
    }
}


// TODO: pull commented out tests from parser.rs for this fn
fn int_to_opcode(insn: u32) -> Opcode {
    let opcode = insn & OPCODE_MASK;
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


#[derive(Debug, Default, PartialEq)]
pub struct Fields {
    pub rs1: Option<u32>,
    pub rs2: Option<u32>,
    pub rd: Option<u32>,
    pub funct3: Option<u32>,
    pub funct7: Option<u32>,
    pub imm: Option<u32>,
    pub shamt: Option<u32>,
    pub opcode: Option<u32>,
}


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


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Format {
    R,
    I,
    S,
    B,
    U,
    J,
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AluOp {
    Add,
    Sub,
}


impl Default for AluOp {
    fn default() -> AluOp {
        AluOp::Add
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AluSrc {
    Reg,
    Imm,
}


impl Default for AluSrc {
    fn default() -> AluSrc {
        AluSrc::Reg
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Function {
    Lui, // Load upper immediate
    AuiPc, // Add upper immediate to PC
    // Jumps
    Jal, // Jump and link
    Jalr, // Jump and link register
    // Branches
    Beq, // Branch if equal
    Bne, // Branch if not equal
    Blt, // Branch if less than
    Bge, // Branch if greater or equal
    Bltu, // Branch if less than (unsigned)
    Bgeu, // Branch if greater or equal (unsigned)
    // Loads
    Lb, // Load byte
    Lh, // Load halfword
    Lw, // Load word
    Lbu, // Load byte (unsigned)
    Lhu, // Load halfword (unsigned)
    // Stores
    Sb, // Store byte
    Sh, // Store halfword
    Sw, // Store word
    // Operations on immediates
    Addi, // Add immediate
    Slti, // Set less than immediate
    Sltiu, // Set less than immediate (unsigned)
    Xori, // Exclusive or immediate
    Ori, // Logical Or immediate
    Andi, // Logical And immediate
    Slli, // Shift left logical immediate
    Srli, // Shift right logical immediate
    Srai, // Shift right arithmetic immediate
    // Operations on registers
    Add, // Add
    Sub, // Subtract
    Sll, // Shift left logical
    Slt, // Set less than
    Sltu, // Set less than unsigned
    Xor, // Exclusive or
    Srl, // Shift right logical
    Sra, // Shift right arithmetic
    Or, // Logical Or
    And, // Logical And
    Halt, // Halt simulator
}


#[derive(Debug, Default)]
pub struct Semantics {
    pub reg_write: bool,
    pub mem_read: bool,
    pub mem_write: bool,
    pub mem_to_reg: bool,
    pub alu_src: AluSrc,
    pub alu_op: AluOp,
}
