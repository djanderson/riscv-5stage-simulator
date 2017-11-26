//! Cycle accurate 5-stage pipelining RISC-V 32I simulator.


use consts;
use hazards::{ex_hazard_src1, ex_hazard_src2, mem_hazard_src1,
              mem_hazard_src2, stall};
use instruction::{Function, Instruction, Opcode};
use memory::data::DataMemory;
use memory::instruction::InstructionMemory;
use pipeline::{IfIdRegister, IdExRegister, ExMemRegister, MemWbRegister};
use register::RegisterFile;
use stages;


/// Runs a cycle accurate RISC-V 32I simulator.
///
/// Returns the address of the instruction and the register file at the point
/// when a HALT hits the execution stage.
///
pub fn run(instructions: &InstructionMemory) -> (usize, RegisterFile) {
    let mut mem = DataMemory::new(1024);
    let mut reg = RegisterFile::new(0x0);

    // Pipline registers
    let mut if_id = IfIdRegister::new();
    let mut id_ex = IdExRegister::new();
    let mut ex_mem = ExMemRegister::new();
    let mut mem_wb = MemWbRegister::new();

    let mut bubble: bool = false;

    loop {

        // Read-only copy of current state of pipeline registers
        let ro_if_id = if_id;
        let ro_id_ex = id_ex;
        let ro_ex_mem = ex_mem;
        let ro_mem_wb = mem_wb;

        // IF: Instruction fetch
        if bubble {
            bubble = false;
        } else {
            // Read and increment program counter
            let pc = reg.pc.read() as usize;
            let npc = (pc + consts::WORD_SIZE) as u32;
            reg.pc.write(npc);

            let raw_insn = stages::insn_fetch(instructions, pc);

            if_id.npc = npc;
            if_id.raw_insn = raw_insn;
        }

        // ID: Instruction decode and register file read
        let raw_insn = ro_if_id.raw_insn;
        let insn = stages::insn_decode(raw_insn);

        if stall(ro_if_id, ro_id_ex) {
            bubble = true;
            id_ex.insn = Instruction::default(); // NOP
        } else {
            id_ex.npc = ro_if_id.npc;
            id_ex.insn = insn;

            let rs1: i32;
            let rs2: i32;
            //let mut (rs1, rs2) = stages::reg_read(&insn, &reg);

            // Do register forwarding (see Patterson & Hennessy pg 301)
            if mem_wb.insn.semantics.reg_write &&
                (mem_wb.insn.fields.rd == insn.fields.rs1)
            {
                rs1 = match mem_wb.insn.semantics.mem_read {
                    true => mem_wb.mem_result as i32,
                    false => mem_wb.alu_result,
                };
            } else {
                rs1 = reg.gpr[insn.fields.rs1.unwrap_or(0) as usize]
                    .read() as i32;
            }

            if mem_wb.insn.semantics.reg_write &&
                (mem_wb.insn.fields.rd == insn.fields.rs2)
            {
                rs2 = match mem_wb.insn.semantics.mem_read {
                    true => mem_wb.mem_result as i32,
                    false => mem_wb.alu_result,
                };
            } else {
                rs2 = reg.gpr[insn.fields.rs2.unwrap_or(0) as usize]
                    .read() as i32;
            }

            id_ex.rs1 = rs1;
            id_ex.rs2 = rs2;
        }


        // EX: Execution or address calculation
        let mut npc = ro_id_ex.npc;

        let pc = if npc == 0 { 0 } else { npc - 4 };
        let mut insn = ro_id_ex.insn;

        // ALU src1 mux
        let rs1: i32;
        if ex_hazard_src1(ro_id_ex, ro_ex_mem) {
            rs1 = ro_ex_mem.alu_result; // forward from previous ALU result
        } else if mem_hazard_src1(ro_id_ex, ro_ex_mem, ro_mem_wb) {
            rs1 = match ro_mem_wb.insn.semantics.mem_read {
                true => ro_mem_wb.mem_result as i32, // forward data memory
                false => ro_mem_wb.alu_result, // forward previous ALU result
            }
        } else {
            rs1 = ro_id_ex.rs1;

        }

        // ALU src2 mux
        let rs2: i32;
        if ex_hazard_src2(ro_id_ex, ro_ex_mem) {
            rs2 = ro_ex_mem.alu_result; // forward previous ALU result
        } else if mem_hazard_src2(ro_id_ex, ro_ex_mem, ro_mem_wb) {
            rs2 = match ro_mem_wb.insn.semantics.mem_read {
                true => ro_mem_wb.mem_result as i32, // forward data memory
                false => ro_mem_wb.alu_result, // forward previous ALU result
            }
        } else {
            rs2 = ro_id_ex.rs2;

        }

        let alu_result = stages::execute(&mut insn, rs1, rs2);

        // Modify program counter for branch or jump
        if insn.semantics.branch &&
            !(insn.opcode == Opcode::Branch && alu_result != 0)
        {
            let imm = insn.fields.imm.unwrap() as i32;
            npc = match insn.opcode {
                Opcode::Jalr => alu_result & 0xfffe, // LSB -> 0
                _ => (pc as i32) + imm,
            } as u32;

        }

        if insn.function == Function::Halt {
            println!("Caught halt instruction at {:#0x}, exiting...", pc);
            return (pc as usize, reg);
        }

        ex_mem.npc = npc;
        ex_mem.insn = ro_id_ex.insn;
        ex_mem.alu_result = alu_result;
        ex_mem.rs2 = rs2;

        // MEM: Data memory access
        let insn = ro_ex_mem.insn;
        let alu_result = ro_ex_mem.alu_result;
        let rs2 = ro_ex_mem.rs2;
        let mem_result =
            stages::access_memory(&insn, &mut mem, alu_result, rs2);

        mem_wb.insn = insn;
        mem_wb.alu_result = alu_result;
        mem_wb.mem_result = mem_result;


        // WB: Write result back to register
        let insn = ro_mem_wb.insn;
        let alu_result = ro_mem_wb.alu_result;
        let mem_result = ro_mem_wb.mem_result;

        stages::reg_writeback(&insn, &mut reg, alu_result, mem_result);

        //println!("{:#0x} - {:?}", pc, insn);
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use memory::instruction::TestInstructionMemory;

    /// Tests forwarding to ALU from EX/MEM and MEM/WB pipeline registers.
    ///
    /// Also tests that data to be written to a register in a certain clock
    /// cycle is made available to the instruction decode/register read stage.
    ///
    /// See Patterson & Hennessy pgs 297-302 for a description of this sequence
    /// and the logic associated with the required forwarding.
    #[test]
    fn forwarding() {
        let insn1 = Instruction::new(0x00_20_00_93); // addi, x1, x0, 2
        let insn2 = Instruction::new(0x00_10_01_93); // addi, x3, x0, 1
        let insn3 = Instruction::new(0x40_30_81_33); // sub x2, x1, x3
        let insn4 = Instruction::new(0x00_51_76_33); // and x12, x2, x5
        let insn5 = Instruction::new(0x00_23_66_b3); // or x13, x6, x2
        let insn6 = Instruction::new(0x00_21_07_33); // add x14, x2, x2
        let insn7 = Instruction::new(0x06_f1_12_23); // sh, x15, 100(x2)

        let insns = vec![
            insn1.as_u32(),
            insn2.as_u32(),
            insn3.as_u32(),
            insn4.as_u32(),
            insn5.as_u32(),
            insn6.as_u32(),
            insn7.as_u32(),
            consts::NOP,
            consts::NOP,
            consts::NOP,
            consts::NOP,
            consts::NOP,
            consts::HALT,
            consts::NOP,
            consts::NOP,
            consts::NOP,
            consts::NOP,
            consts::NOP,
        ];

        let insn_memory = TestInstructionMemory::new(insns);
        let (halt_addr, reg) = run(&insn_memory);

        assert_eq!(halt_addr, 0x30);
        assert_eq!(reg.gpr[2].read(), 1); // x2 == 1
        assert_eq!(reg.gpr[3].read(), 1); // x3 == 1
        assert_eq!(reg.gpr[12].read(), 0); // x12 == 0
        assert_eq!(reg.gpr[13].read(), 1); // x13 == 1
        assert_eq!(reg.gpr[14].read(), 2); // x14 == 2
        assert_eq!(reg.gpr[15].read(), 0); // x15 == 0
    }

}
