//! Single cycle instruction accurate RISC-V 32I simulator.


use consts::WORD_SIZE;
use instruction::{Function, Opcode};
use memory::data::DataMemory;
use memory::instruction::InstructionMemory;
use register::RegisterFile;
use stages;


/// Runs a single cycle instruction accurate RISC-V 32I simulator.
pub fn run(instructions: &InstructionMemory) {
    let mut mem = DataMemory::new(16384);
    let mut reg = RegisterFile::new(0x0);

    loop {
        // Read and increment program counter
        let pc = reg.pc.read() as usize;
        reg.pc.write((pc + WORD_SIZE) as u32);

        // IF: Instruction fetch from memory
        let raw_insn = stages::insn_fetch(instructions, pc);

        // ID: Instruction decode & register read
        let mut insn = stages::insn_decode(raw_insn);
        let (rs1, rs2) = stages::reg_read(&insn, &reg);

        // EX: Execute operation or calculate address
        let alu_result = stages::execute(&mut insn, rs1, rs2);

        // MEM: Access memory operand
        let mem_result = stages::access_memory(&insn, &mut mem, alu_result);

        // WB: Write result back to register
        stages::reg_writeback(&insn, &mut reg, alu_result, mem_result);

        if insn.function == Function::Halt {
            println!("Caught halt instruction at {:#0x}, exiting...", pc);
            return;
        }

        println!("{:#0x} - {:?}", pc, insn);

        // Modify program counter for branch or jump
        if insn.semantics.branch &&
            !(insn.opcode == Opcode::Branch && alu_result != 0)
        {
            let imm = insn.fields.imm.unwrap() as i32;
            let npc = match insn.opcode {
                Opcode::Jalr => alu_result & 0xfffe, // LSB -> 0
                _ => (pc as i32) + imm,
            };
            reg.pc.write(npc as u32);
        }

    }

}
