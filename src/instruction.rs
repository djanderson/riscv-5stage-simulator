// Masks to isolate specific parts of the instruction using logical AND (&)
const FUNCT7_MASK: u32 = 0xfe000000;
const FUNCT3_MASK: u32 = 0x7000;
const RS1_MASK: u32 = 0xf8000;
const RS2_MASK: u32 = 0x1f00000;
const RD_MASK: u32 = 0xf80;
const OPCODE_MASK: u32 = 0x7f;

// Indices of instruction parts for shifting
const FUNCT7_SHIFT: u8 = 25;
const FUNCT3_SHIFT: u8 = 12;
const RS1_SHIFT: u8 = 15;
const RS2_SHIFT: u8 = 20;
const RD_SHIFT: u8 = 7;


#[derive(Debug)]
pub struct Instruction {
    instruction: u32,
    pub opcode: Opcode,
    pub fields: Fields,
    pub format: Format,
}


impl Instruction {
    pub fn new(instruction: u32) -> Instruction {
        let opcode = int_to_opcode(instruction);
        let format = opcode_to_format(opcode);
        let fields = Fields::default();
        let mut insn = Instruction {
            instruction,
            opcode,
            format,
            fields,
        };
        insn.parse();
        insn
    }

    pub fn as_u32(&self) -> u32 {
        self.instruction
    }

    fn parse(&mut self) {
        match self.format {
            Format::R => self.parse_type_r(),
            Format::I => self.parse_type_i(),
            Format::S => self.parse_type_s(),
            Format::B => self.parse_type_b(),
            Format::U => self.parse_type_u(),
            Format::J => self.parse_type_j(),
        };
    }

    fn parse_type_r(&mut self) {
        let insn = self.instruction;
        self.fields.funct3 = Some((insn & FUNCT3_MASK) >> FUNCT3_SHIFT);
        self.fields.funct7 = Some((insn & FUNCT7_MASK) >> FUNCT7_SHIFT);
        self.fields.rs1 = Some((insn & RS1_MASK) >> RS1_SHIFT);
        self.fields.rs2 = Some((insn & RS2_MASK) >> RS2_SHIFT);
        self.fields.rd = Some((insn & RD_MASK) >> RD_SHIFT);
    }

    fn parse_type_i(&mut self) {
        let insn = self.instruction;
        self.fields.funct3 = Some((insn & FUNCT3_MASK) >> FUNCT3_SHIFT);
        self.fields.rs1 = Some((insn & RS1_MASK) >> RS1_SHIFT);
        self.fields.rd = Some((insn & RD_MASK) >> RD_SHIFT);
        if self.fields.funct3 == Some(0x1) || self.fields.funct3 == Some(0x5) {
            // Shift: insn[24:20] -> shamt
            self.fields.shamt = Some((insn & RS2_MASK) >> RS2_SHIFT)
        } else {
            // Arithmetic or logical: insn[31:20] -> imm[11:0]
            self.fields.imm = Some((insn & 0xfff00000) >> 20);
        }
    }

    fn parse_type_s(&mut self) {
        let insn = self.instruction;
        self.fields.funct3 = Some((insn & FUNCT3_MASK) >> FUNCT3_SHIFT);
        self.fields.rs1 = Some((insn & RS1_MASK) >> RS1_SHIFT);
        self.fields.rs2 = Some((insn & RS2_MASK) >> RS2_SHIFT);
        // insn[31:25] -> imm[11:5]
        let imm_high = (insn & 0xfe000000) >> 20;
        // insn[11:7] -> imm[4:0]
        let imm_low = (insn & 0xF80) >> 7;
        self.fields.imm = Some(imm_high | imm_low);
    }

    fn parse_type_b(&mut self) {
        let insn = self.instruction;
        self.fields.funct3 = Some((insn & FUNCT3_MASK) >> FUNCT3_SHIFT);
        self.fields.rs1 = Some((insn & RS1_MASK) >> RS1_SHIFT);
        self.fields.rs2 = Some((insn & RS2_MASK) >> RS2_SHIFT);
        // insn[7] -> imm[11]
        let imm_bit_11 = (insn & 0x80) << 4;
        // insn[31] -> imm[12]
        let imm_bit_12 = (insn & 0x80000000) >> 19;
        // insn[30:25] -> imm[10:5]
        let imm_high = (insn & 0x7e000000) >> 20;
        // insn[11:8] -> imm[4:1]
        let imm_low = (insn & 0xf00) >> 7;
        self.fields.imm = Some(imm_bit_12 | imm_bit_11 | imm_high | imm_low);
    }

    fn parse_type_u(&mut self) {
        let insn = self.instruction;
        self.fields.rd = Some((insn & RD_MASK) >> RD_SHIFT);
        // insn[31:12] -> imm[31:12]
        self.fields.imm = Some(insn & 0xfffff000);
    }

    fn parse_type_j(&mut self) {
        let insn = self.instruction;
        self.fields.rd = Some((insn & RD_MASK) >> RD_SHIFT);
        // insn[31] -> imm[20]
        let imm_bit_20 = (insn & 0x80000000) >> 11;
        // insn[30:21] -> imm[10:1]
        let imm_low = (insn & 0xff700000) >> 20;
        // insn[20] -> imm[11]
        let imm_bit_11 = (insn & 0x100000) >> 9;
        // isns[19:12] -> imm[19:12]
        let imm_high = insn & 0xff000;
        self.fields.imm = Some(imm_bit_20 | imm_high | imm_bit_11 | imm_low);
    }
}


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
    }
}


#[derive(Clone, Copy, Debug, Default, PartialEq)]
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


#[cfg(test)]
mod tests {
    use super::*;

    /// Masks should cover entire instruction without overlapping
    #[test]
    fn masks() {
        assert_eq!(
            FUNCT7_MASK ^ FUNCT3_MASK ^ RS1_MASK ^ RS2_MASK ^ RD_MASK ^
                OPCODE_MASK,
            0xffffffff
        );
    }

    #[test]
    fn parse_type_r() {
        // add x5, x6, x7
        let raw_insn = 0x7302b3;
        let parsed_insn = Instruction::new(raw_insn);
        assert_eq!(parsed_insn.opcode, Opcode::Op);
        assert_eq!(parsed_insn.fields.funct3.unwrap(), 0x0);
        assert_eq!(parsed_insn.fields.rd.unwrap(), 0x05);
        assert_eq!(parsed_insn.fields.rs1.unwrap(), 0x06);
        assert_eq!(parsed_insn.fields.rs2.unwrap(), 0x07);
    }

    #[test]
    fn parse_type_i_arithmetic() {
        // addi x5, x6, 20
        let raw_insn = 0x1430293;
        let parsed_insn = Instruction::new(raw_insn);
        assert_eq!(parsed_insn.opcode, Opcode::OpImm);
        assert_eq!(parsed_insn.fields.funct3.unwrap(), 0x0);
        assert_eq!(parsed_insn.fields.rd.unwrap(), 0x05);
        assert_eq!(parsed_insn.fields.rs1.unwrap(), 0x06);
        assert_eq!(parsed_insn.fields.imm.unwrap(), 20);
    }

    #[test]
    fn parse_type_i_shift() {
        // slli x5, x6, 3
        let raw_insn = 0x331293;
        let parsed_insn = Instruction::new(raw_insn);
        assert_eq!(parsed_insn.opcode, Opcode::OpImm);
        assert_eq!(parsed_insn.fields.funct3.unwrap(), 0x1);
        assert_eq!(parsed_insn.fields.rd.unwrap(), 0x05);
        assert_eq!(parsed_insn.fields.rs1.unwrap(), 0x06);
        assert_eq!(parsed_insn.fields.shamt.unwrap(), 3);
    }

    #[test]
    fn parse_type_s() {
        // sw x5, 40(x6)
        let raw_insn = 0x2532423;
        let parsed_insn = Instruction::new(raw_insn);
        assert_eq!(parsed_insn.opcode, Opcode::Store);
        assert_eq!(parsed_insn.fields.funct3.unwrap(), 0x2);
        assert_eq!(parsed_insn.fields.rs1.unwrap(), 0x06);
        assert_eq!(parsed_insn.fields.rs2.unwrap(), 0x05);
        assert_eq!(parsed_insn.fields.imm.unwrap(), 40);
    }

    #[test]
    fn parse_type_b() {
        // beq x5, x6, 100
        let raw_insn = 0x6628263;
        let parsed_insn = Instruction::new(raw_insn);
        assert_eq!(parsed_insn.opcode, Opcode::Branch);
        assert_eq!(parsed_insn.fields.funct3.unwrap(), 0x0);
        assert_eq!(parsed_insn.fields.rs1.unwrap(), 0x05);
        assert_eq!(parsed_insn.fields.rs2.unwrap(), 0x06);
        assert_eq!(parsed_insn.fields.imm.unwrap(), 100);
    }

    #[test]
    fn parse_type_u() {
        // lui x5, 0x12345
        let raw_insn = 0x123452b7;
        let parsed_insn = Instruction::new(raw_insn);
        assert_eq!(parsed_insn.opcode, Opcode::Lui);
        assert_eq!(parsed_insn.fields.rd.unwrap(), 0x05);
        assert_eq!(parsed_insn.fields.imm.unwrap(), 0x12345000);
    }

    #[test]
    fn parse_type_j() {
        // jal x1, 100
        let raw_insn = 0x64000ef;
        let parsed_insn = Instruction::new(raw_insn);
        assert_eq!(parsed_insn.opcode, Opcode::Jal);
        assert_eq!(parsed_insn.fields.rd.unwrap(), 0x01);
        assert_eq!(parsed_insn.fields.imm.unwrap(), 100);

    }
}
