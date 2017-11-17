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


// TODO: this needs tests
