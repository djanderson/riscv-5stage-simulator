use super::*;


// Masks to isolate specific parts of the instruction using logical AND (&)
const FUNCT7_MASK: u32 = 0xfe000000;
const FUNCT3_MASK: u32 = 0x7000;
const RS1_MASK: u32 = 0xf8000;
const RS2_MASK: u32 = 0x1f00000;
const RD_MASK: u32 = 0xf80;
const OPCODE_MASK: u32 = 0x7f;
const BIT30_MASK: u32 = 0x40000000;

// Indices of instruction parts for shifting
const FUNCT7_SHIFT: u8 = 25;
const FUNCT3_SHIFT: u8 = 12;
const RS1_SHIFT: u8 = 15;
const RS2_SHIFT: u8 = 20;
const RD_SHIFT: u8 = 7;


pub fn decode(insn: &mut Instruction) {
    insn.fields = match insn.format {
        Format::R => parse_type_r(insn.value),
        Format::I => parse_type_i(insn.value),
        Format::S => parse_type_s(insn.value),
        Format::B => parse_type_b(insn.value),
        Format::U => parse_type_u(insn.value),
        Format::J => parse_type_j(insn.value),
    };

    insn.function = insn_to_fn(insn);
}


fn insn_to_fn(insn: &Instruction) -> Function {
    let bit30 = insn.value & BIT30_MASK;
    let funct3 = insn.fields.funct3.unwrap();
    match (insn.opcode, funct3, bit30) {
        (Opcode::Lui, ..) => Function::Lui,
        (Opcode::AuiPc, ..) => Function::AuiPc,
        (Opcode::Jal, ..) => Function::Jal,
        (Opcode::Jalr, ..) => Function::Jalr,
        (Opcode::Branch, 0b000, _) => Function::Beq,
        (Opcode::Branch, 0b001, _) => Function::Bne,
        (Opcode::Branch, 0b100, _) => Function::Blt,
        (Opcode::Branch, 0b101, _) => Function::Bge,
        (Opcode::Branch, 0b110, _) => Function::Bltu,
        (Opcode::Branch, 0b111, _) => Function::Bgeu,
        (Opcode::Load, 0b000, _) => Function::Lb,
        (Opcode::Load, 0b001, _) => Function::Lh,
        (Opcode::Load, 0b010, _) => Function::Lw,
        (Opcode::Load, 0b100, _) => Function::Lbu,
        (Opcode::Load, 0b101, _) => Function::Lhu,
        (Opcode::Store, 0b000, _) => Function::Sb,
        (Opcode::Store, 0b001, _) => Function::Sh,
        (Opcode::Store, 0b010, _) => Function::Sw,
        (Opcode::OpImm, 0b000, _) => Function::Addi,
        (Opcode::OpImm, 0b010, _) => Function::Slti,
        (Opcode::OpImm, 0b011, _) => Function::Sltiu,
        (Opcode::OpImm, 0b100, _) => Function::Xori,
        (Opcode::OpImm, 0b110, _) => Function::Ori,
        (Opcode::OpImm, 0b111, _) => Function::Andi,
        (Opcode::OpImm, 0b001, _) => Function::Slli,
        (Opcode::OpImm, 0b101, 0b0) => Function::Srli,
        (Opcode::OpImm, 0b101, 0b1) => Function::Srai,
        (Opcode::Op, 0b000, 0b0) => Function::Add,
        (Opcode::Op, 0b000, 0b1) => Function::Sub,
        (Opcode::Op, 0b001, _) => Function::Sll,
        (Opcode::Op, 0b010, _) => Function::Slt,
        (Opcode::Op, 0b011, _) => Function::Sltu,
        (Opcode::Op, 0b100, _) => Function::Xor,
        (Opcode::Op, 0b101, 0b0) => Function::Srl,
        (Opcode::Op, 0b101, 0b1) => Function::Sra,
        (Opcode::Op, 0b110, _) => Function::Or,
        (Opcode::Op, 0b111, _) => Function::And,
        _ => panic!("Failed to decode instruction {:#0b}", insn.value),
    }
}


fn parse_type_r(insn: u32) -> Fields {
    let mut fields = Fields::default();
    fields.funct3 = Some((insn & FUNCT3_MASK) >> FUNCT3_SHIFT);
    fields.funct7 = Some((insn & FUNCT7_MASK) >> FUNCT7_SHIFT);
    fields.rs1 = Some((insn & RS1_MASK) >> RS1_SHIFT);
    fields.rs2 = Some((insn & RS2_MASK) >> RS2_SHIFT);
    fields.rd = Some((insn & RD_MASK) >> RD_SHIFT);

    fields
}


fn parse_type_i(insn: u32) -> Fields {
    let mut fields = Fields::default();
    fields.funct3 = Some((insn & FUNCT3_MASK) >> FUNCT3_SHIFT);
    fields.rs1 = Some((insn & RS1_MASK) >> RS1_SHIFT);
    fields.rd = Some((insn & RD_MASK) >> RD_SHIFT);
    if fields.funct3 == Some(0x1) || fields.funct3 == Some(0x5) {
        // Shift: insn[24:20] -> shamt
        fields.shamt = Some((insn & RS2_MASK) >> RS2_SHIFT);
    } else {
        // Arithmetic or logical: insn[31:20] -> imm[11:0]
        fields.imm = Some((insn & 0xfff00000) >> 20);
    }

    fields
}


fn parse_type_s(insn: u32) -> Fields {
    let mut fields = Fields::default();
    fields.funct3 = Some((insn & FUNCT3_MASK) >> FUNCT3_SHIFT);
    fields.rs1 = Some((insn & RS1_MASK) >> RS1_SHIFT);
    fields.rs2 = Some((insn & RS2_MASK) >> RS2_SHIFT);
    // insn[31:25] -> imm[11:5]
    let imm_high = (insn & 0xfe000000) >> 20;
    // insn[11:7] -> imm[4:0]
    let imm_low = (insn & 0xF80) >> 7;
    fields.imm = Some(imm_high | imm_low);

    fields
}


fn parse_type_b(insn: u32) -> Fields {
    let mut fields = Fields::default();
    fields.funct3 = Some((insn & FUNCT3_MASK) >> FUNCT3_SHIFT);
    fields.rs1 = Some((insn & RS1_MASK) >> RS1_SHIFT);
    fields.rs2 = Some((insn & RS2_MASK) >> RS2_SHIFT);
    // insn[7] -> imm[11]
    let imm_bit_11 = (insn & 0x80) << 4;
    // insn[31] -> imm[12]
    let imm_bit_12 = (insn & 0x80000000) >> 19;
    // insn[30:25] -> imm[10:5]
    let imm_high = (insn & 0x7e000000) >> 20;
    // insn[11:8] -> imm[4:1]
    let imm_low = (insn & 0xf00) >> 7;
    fields.imm = Some(imm_bit_12 | imm_bit_11 | imm_high | imm_low);

    fields
}


fn parse_type_u(insn: u32) -> Fields {
    let mut fields = Fields::default();
    fields.rd = Some((insn & RD_MASK) >> RD_SHIFT);
    // insn[31:12] -> imm[31:12]
    fields.imm = Some(insn & 0xfffff000);

    fields
}


fn parse_type_j(insn: u32) -> Fields {
    let mut fields = Fields::default();
    fields.rd = Some((insn & RD_MASK) >> RD_SHIFT);
    // insn[31] -> imm[20]
    let imm_bit_20 = (insn & 0x80000000) >> 11;
    // insn[30:21] -> imm[10:1]
    let imm_low = (insn & 0xff700000) >> 20;
    // insn[20] -> imm[11]
    let imm_bit_11 = (insn & 0x100000) >> 9;
    // isns[19:12] -> imm[19:12]
    let imm_high = insn & 0xff000;
    fields.imm = Some(imm_bit_20 | imm_high | imm_bit_11 | imm_low);

    fields
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
    fn type_r() {
        // add x5, x6, x7
        let insn = 0x7302b3;
        //let parsed_insn = Instruction::new(raw_insn);
        //assert_eq!(parsed_insn.opcode, Opcode::Op);
        let fields = parse_type_r(insn);
        assert_eq!(fields.funct3.unwrap(), 0x0);
        assert_eq!(fields.rd.unwrap(), 0x05);
        assert_eq!(fields.rs1.unwrap(), 0x06);
        assert_eq!(fields.rs2.unwrap(), 0x07);
    }

    #[test]
    fn type_i_arithmetic() {
        // addi x5, x6, 20
        let insn = 0x1430293;
        //let parsed_insn = Instruction::new(raw_insn);
        //assert_eq!(parsed_insn.opcode, Opcode::OpImm);
        let fields = parse_type_i(insn);
        assert_eq!(fields.funct3.unwrap(), 0x0);
        assert_eq!(fields.rd.unwrap(), 0x05);
        assert_eq!(fields.rs1.unwrap(), 0x06);
        assert_eq!(fields.imm.unwrap(), 20);
    }

    #[test]
    fn type_i_shift() {
        // slli x5, x6, 3
        let insn = 0x331293;
        //let parsed_insn = Instruction::new(raw_insn);
        //assert_eq!(parsed_insn.opcode, Opcode::OpImm);
        let fields = parse_type_i(insn);
        assert_eq!(fields.funct3.unwrap(), 0x1);
        assert_eq!(fields.rd.unwrap(), 0x05);
        assert_eq!(fields.rs1.unwrap(), 0x06);
        assert_eq!(fields.shamt.unwrap(), 3);
    }

    #[test]
    fn type_s() {
        // sw x5, 40(x6)
        let insn = 0x2532423;
        //let parsed_insn = Instruction::new(raw_insn);
        //assert_eq!(parsed_insn.opcode, Opcode::Store);
        let fields = parse_type_s(insn);
        assert_eq!(fields.funct3.unwrap(), 0x2);
        assert_eq!(fields.rs1.unwrap(), 0x06);
        assert_eq!(fields.rs2.unwrap(), 0x05);
        assert_eq!(fields.imm.unwrap(), 40);
    }

    #[test]
    fn type_b() {
        // beq x5, x6, 100
        let insn = 0x6628263;
        //let parsed_insn = Instruction::new(raw_insn);
        //assert_eq!(parsed_insn.opcode, Opcode::Branch);
        let fields = parse_type_b(insn);
        assert_eq!(fields.funct3.unwrap(), 0x0);
        assert_eq!(fields.rs1.unwrap(), 0x05);
        assert_eq!(fields.rs2.unwrap(), 0x06);
        assert_eq!(fields.imm.unwrap(), 100);
    }

    #[test]
    fn type_u() {
        // lui x5, 0x12345
        let insn = 0x123452b7;
        //let parsed_insn = Instruction::new(raw_insn);
        //assert_eq!(parsed_insn.opcode, Opcode::Lui);
        let fields = parse_type_u(insn);
        assert_eq!(fields.rd.unwrap(), 0x05);
        assert_eq!(fields.imm.unwrap(), 0x12345000);
    }

    #[test]
    fn type_j() {
        // jal x1, 100
        let insn = 0x64000ef;
        //let parsed_insn = Instruction::new(raw_insn);
        //assert_eq!(parsed_insn.opcode, Opcode::Jal);
        let fields = parse_type_j(insn);
        assert_eq!(fields.rd.unwrap(), 0x01);
        assert_eq!(fields.imm.unwrap(), 100);
    }
}