//! 5-stage instruction execution.


use alu::{alu, AluSrc};
use consts;
use instruction::{Instruction, Opcode};
use memory::data::DataMemory;
use memory::instruction::InstructionMemory;
use register::RegisterFile;


/// IF: Instruction fetch from memory.
pub fn insn_fetch(mem: &InstructionMemory, pc: u32, _clk: u64) -> u32 {
    mem.read(pc as usize)
}


/// ID 1: Instruction decode
pub fn insn_decode(raw_insn: u32, _clk: u64) -> Instruction {
    Instruction::new(raw_insn)
}


/// ID 2: Register read
pub fn reg_read(
    insn: &Instruction,
    reg: &RegisterFile,
    _clk: u64,
) -> (i32, i32) {
    let rs1 = reg.gpr[insn.fields.rs1.unwrap_or(0) as usize].read() as i32;
    let rs2 = reg.gpr[insn.fields.rs2.unwrap_or(0) as usize].read() as i32;

    (rs1, rs2)
}

/// EX: Execute operation or calculate address.
pub fn execute(insn: &mut Instruction, rs1: i32, rs2: i32, _clk: u64) -> i32 {
    let src1 = rs1;
    let src2 = match insn.semantics.alu_src {
        AluSrc::Reg => rs2,
        AluSrc::Imm => insn.fields.imm.unwrap() as i32,
    };

    alu(&insn, src1, src2, _clk)
}


/// MEM: Access memory operand.
pub fn access_memory(
    insn: &Instruction,
    mem: &mut DataMemory,
    alu_result: i32,
    rs2: i32,
    _clk: u64,
) -> u32 {
    let mut mem_result: u32 = 0;

    if insn.semantics.mem_read {
        mem_result = mem.read(alu_result as usize, insn.semantics.mem_size);
    } else if insn.semantics.mem_write {
        mem.write(alu_result as usize, insn.semantics.mem_size, rs2 as u32);
    }

    mem_result
}


/// WB: Write result back to register.
pub fn reg_writeback(
    pc: u32,
    insn: &Instruction,
    reg: &mut RegisterFile,
    alu_result: i32,
    mem_result: u32,
    _clk: u64,
) {
    if insn.semantics.reg_write {
        let rd = insn.fields.rd.unwrap() as usize;
        let npc = pc + consts::WORD_SIZE as u32;

        if rd == 0 {
            return; // x0 is read-only
        }

        let value = match insn.semantics.mem_to_reg {
            true => mem_result,
            false => {
                match insn.opcode {
                    Opcode::Lui => insn.fields.imm.unwrap(),
                    Opcode::Jal | Opcode::Jalr => npc,
                    _ => alu_result as u32,
                }
            }
        };

        trace!("Writeback: x[{}] = {} (clock {})", rd, value, _clk);

        reg.gpr[rd].write(value);
    }
}
