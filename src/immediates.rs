//! Sign extend immediate value.


use instruction::{Instruction, Opcode};


/// Sign extend immediate value.
pub fn gen(insn: &Instruction) -> Option<u32> {
    let shamt = match insn.opcode {
        Opcode::Lui | Opcode::AuiPc => 0,
        Opcode::Jal | Opcode::Jalr => 12,
        Opcode::Branch => 19,
        _ => 20,
    };

    match insn.fields.imm {
        Some(v) => Some((((v as i32) << shamt) >> shamt) as u32),
        None => None,
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_back() {
        // bge x11, x10, -4
        let insn = Instruction::new(0xfea5dee3);
        let imm = gen(&insn).unwrap() as i32;
        assert_eq!(imm, -4);
    }

    #[test]
    fn branch_forward() {
        // bltu x13, x14, 16
        let insn = Instruction::new(0x00e6e863);
        let imm = gen(&insn).unwrap() as i32;
        assert_eq!(imm, 16);
    }

    // TODO: needs more tests
}
