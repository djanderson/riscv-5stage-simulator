//! Five stage instruction execution with pipeline control.


use consts;
use hazards;
use instruction::{Function, Opcode};
use memory::data::DataMemory;
use memory::instruction::InstructionMemory;
use pipeline::{PcSrc, Pipeline};
use register::RegisterFile;
use stages;


/// IF: Instruction fetch from memory.
pub fn insn_fetch(
    write_pipeline: &mut Pipeline,
    insns: &InstructionMemory,
    reg: &mut RegisterFile,
) {
    // Read and increment program counter
    let pc = reg.pc.read() as usize;
    let npc = (pc + consts::WORD_SIZE) as u32;
    reg.pc.write(npc);

    // IF: Instruction fetch
    let raw_insn = stages::insn_fetch(insns, pc);

    write_pipeline.if_id.npc = npc;
    write_pipeline.if_id.raw_insn = raw_insn;
}


/// ID: Instruction decode and register read
pub fn insn_decode(
    read_pipeline: &Pipeline,
    write_pipeline: &mut Pipeline,
    reg: &mut RegisterFile,
) {
    // ID: Instruction decode and register file read
    let raw_insn = read_pipeline.if_id.raw_insn;
    let insn = stages::insn_decode(raw_insn);

    write_pipeline.id_ex.npc = read_pipeline.if_id.npc;
    write_pipeline.id_ex.insn = insn;


    // Do register forwarding (see Patterson & Hennessy pg 301)
    let rs1: i32;
    if write_pipeline.mem_wb.insn.semantics.reg_write &&
        (write_pipeline.mem_wb.insn.fields.rd == insn.fields.rs1)
    {
        rs1 = match write_pipeline.mem_wb.insn.semantics.mem_read {
            true => write_pipeline.mem_wb.mem_result as i32,
            false => write_pipeline.mem_wb.alu_result,
        };
    } else {
        rs1 = reg.gpr[insn.fields.rs1.unwrap_or(0) as usize].read() as i32;
    }

    let rs2: i32;
    if write_pipeline.mem_wb.insn.semantics.reg_write &&
        (write_pipeline.mem_wb.insn.fields.rd == insn.fields.rs2)
    {
        rs2 = match write_pipeline.mem_wb.insn.semantics.mem_read {
            true => write_pipeline.mem_wb.mem_result as i32,
            false => write_pipeline.mem_wb.alu_result,
        };
    } else {
        rs2 = reg.gpr[insn.fields.rs2.unwrap_or(0) as usize].read() as i32;
    }

    write_pipeline.id_ex.rs1 = rs1;
    write_pipeline.id_ex.rs2 = rs2;
}



/// EX: Execute operation or calculate address.
pub fn execute(
    read_pipeline: &Pipeline,
    write_pipeline: &mut Pipeline,
) -> Option<usize> {
    let mut npc = read_pipeline.id_ex.npc;

    let pc = if npc == 0 { 0 } else { npc - 4 };
    let mut insn = read_pipeline.id_ex.insn;

    // ALU src1 mux
    let rs1: i32;
    if hazards::ex_hazard_src1(&read_pipeline) {
        // forward from previous ALU result
        rs1 = read_pipeline.ex_mem.alu_result;
    } else if hazards::mem_hazard_src1(&read_pipeline) {
        rs1 = match read_pipeline.mem_wb.insn.semantics.mem_read {
            // forward data memory
            true => read_pipeline.mem_wb.mem_result as i32,
            // forward previous ALU result
            false => read_pipeline.mem_wb.alu_result,
        }
    } else {
        rs1 = read_pipeline.id_ex.rs1;
    }

    // ALU src2 mux
    let rs2: i32;
    if hazards::ex_hazard_src2(&read_pipeline) {
        // forward previous ALU result
        rs2 = read_pipeline.ex_mem.alu_result;
    } else if hazards::mem_hazard_src2(&read_pipeline) {
        rs2 = match read_pipeline.mem_wb.insn.semantics.mem_read {
            // forward data memory
            true => read_pipeline.mem_wb.mem_result as i32,
            // forward previous ALU result
            false => read_pipeline.mem_wb.alu_result,
        }
    } else {
        rs2 = read_pipeline.id_ex.rs2;
    }

    let alu_result = stages::execute(&mut insn, rs1, rs2);

    // Modify program counter for branch or jump
    let pc_src: PcSrc;
    if insn.semantics.branch &&
        !(insn.opcode == Opcode::Branch && alu_result != 0)
    {
        let imm = insn.fields.imm.unwrap() as i32;

        npc = match insn.opcode {
            Opcode::Jalr => alu_result & 0xfffe, // LSB -> 0
            _ => (pc as i32) + imm,
        } as u32;

        pc_src = PcSrc::Branch;
    } else {
        pc_src = PcSrc::Next;
    }

    if insn.function == Function::Halt {
        return Some(pc as usize);
    }

    write_pipeline.ex_mem.npc = npc;
    write_pipeline.ex_mem.insn = read_pipeline.id_ex.insn;
    write_pipeline.ex_mem.alu_result = alu_result;
    write_pipeline.ex_mem.rs2 = rs2;
    write_pipeline.ex_mem.pc_src = pc_src;

    None
}


/// MEM: Access memory operand.
pub fn access_memory(
    read_pipeline: &Pipeline,
    write_pipeline: &mut Pipeline,
    mut mem: &mut DataMemory,
) {
    let insn = read_pipeline.ex_mem.insn;
    let alu_result = read_pipeline.ex_mem.alu_result;
    let rs2 = read_pipeline.ex_mem.rs2;
    let mem_result = stages::access_memory(&insn, &mut mem, alu_result, rs2);

    write_pipeline.mem_wb.insn = insn;
    write_pipeline.mem_wb.alu_result = alu_result;
    write_pipeline.mem_wb.mem_result = mem_result;
}


/// WB: Write result back to register.
pub fn reg_writeback(read_pipeline: &Pipeline, mut reg: &mut RegisterFile) {
    let insn = read_pipeline.mem_wb.insn;
    let alu_result = read_pipeline.mem_wb.alu_result;
    let mem_result = read_pipeline.mem_wb.mem_result;

    stages::reg_writeback(&insn, &mut reg, alu_result, mem_result);
}
