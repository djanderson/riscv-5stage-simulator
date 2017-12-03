//! Single cycle instruction accurate RISC-V 32I simulator.


use consts;
use instruction::{Function, Opcode};
use memory::data::DataMemory;
use memory::instruction::InstructionMemory;
use register::RegisterFile;
use stages;


/// Runs a single cycle instruction accurate RISC-V 32I simulator.
///
/// Returns the PC address of the HALT instruction.
///
pub fn run(
    insns: &InstructionMemory,
    mut mem: &mut DataMemory,
    mut reg: &mut RegisterFile,
) -> usize {
    loop {
        // Read and increment program counter
        let pc = reg.pc.read();
        reg.pc.write(pc + consts::WORD_SIZE as u32);

        // IF: Instruction fetch
        let raw_insn = stages::insn_fetch(insns, pc);

        // ID: Instruction decode and register file read
        let mut insn = stages::insn_decode(raw_insn);
        let (rs1, rs2) = stages::reg_read(&insn, &reg);

        // EX: Execution or address calculation
        let alu_result = stages::execute(&mut insn, rs1, rs2);

        // MEM: Data memory access
        let mem_result =
            stages::access_memory(&insn, &mut mem, alu_result, rs2);

        // WB: Write result back to register
        stages::reg_writeback(pc, &insn, &mut reg, alu_result, mem_result);

        if insn.function == Function::Halt {
            println!("Caught halt instruction at {:#0x}, exiting...", pc);
            return pc as usize;
        }

        // Modify program counter for branch or jump
        if insn.semantics.branch &&
            !(insn.opcode == Opcode::Branch && alu_result != 0)
        {
            let imm = insn.fields.imm.unwrap() as i32;
            let npc = match insn.opcode {
                Opcode::Jalr => alu_result & 0xfffe, // LSB -> 0
                _ => (pc as i32) + imm,
            };
            println!("Branching - {:#0x} -> {:#0x}", pc, npc);
            reg.pc.write(npc as u32);
        }

    }

}
