//! Single cycle instruction accurate RISC-V 32I simulator.


use consts;
use instruction::{Function, Opcode};
use memory::data::DataMemory;
use memory::instruction::InstructionMemory;
use register::RegisterFile;
use stages::{insn_fetch, insn_decode, reg_read, execute, access_memory,
             reg_writeback};


/// Runs a single cycle instruction accurate RISC-V 32I simulator.
///
/// Returns the PC address of the HALT instruction.
///
pub fn run(
    insns: &InstructionMemory,
    mut mem: &mut DataMemory,
    mut reg: &mut RegisterFile,
) -> usize {
    // Clock is used to aid debugging only
    let mut clock: u64 = 0;

    loop {
        // Read and increment program counter
        let pc = reg.pc.read();
        reg.pc.write(pc + consts::WORD_SIZE as u32);

        // IF: Instruction fetch
        let raw_insn = insn_fetch(insns, pc, clock);

        // ID: Instruction decode and register file read
        let mut insn = insn_decode(raw_insn, clock);
        let (rs1, rs2) = reg_read(&insn, &reg, clock);

        // EX: Execution or address calculation
        let alu_result = execute(&mut insn, rs1, rs2, clock);

        // MEM: Data memory access
        let mem_result =
            access_memory(&insn, &mut mem, alu_result, rs2, clock);

        // WB: Write result back to register
        reg_writeback(pc, &insn, &mut reg, alu_result, mem_result, clock);

        if insn.function == Function::Halt {
            info!("Halt: {:#0x} (clock {}), exiting...", pc, clock);
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

            trace!("Jump: {:#0x} -> {:#0x} (clock {})", pc, npc, clock);

            reg.pc.write(npc as u32);
        }

        clock += 1;
    }

}
