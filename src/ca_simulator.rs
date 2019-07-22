//! Cycle accurate 5-stage pipelining RISC-V 32I simulator.


use hazards;
use instruction::Instruction;
use memory::data::DataMemory;
use memory::instruction::InstructionMemory;
use pipeline::Pipeline;
use pipeline::stages::{insn_fetch, insn_decode, execute, access_memory,
                       reg_writeback};
use register::RegisterFile;


/// Runs a cycle accurate RISC-V 32I simulator.
///
/// Returns the address of the HALT instruction.
///
pub fn run(
    insns: &InstructionMemory,
    mut mem: &mut DataMemory,
    mut reg: &mut RegisterFile,
) -> usize {
    // Clock is used to aid debugging only
    let mut clock: u64 = 0;

    // Pipline registers
    let mut write_pipeline = Pipeline::new();
    let mut read_pipeline = Pipeline::new();

    loop {
        if hazards::load_hazard(&read_pipeline) {
            write_pipeline.id_ex.insn = Instruction::default(); // NOP
        } else {
            insn_fetch(&mut write_pipeline, insns, &mut reg, clock);
            insn_decode(&read_pipeline, &mut write_pipeline, &mut reg, clock);
        }

        execute(&read_pipeline, &mut write_pipeline, clock);

        access_memory(
            &read_pipeline,
            &mut write_pipeline,
            &mut mem,
            &mut reg,
            clock,
        );

        if let Some(addr) = write_pipeline.ex_mem.halt_addr {
            info!("Halt: {:#0x} (clock {}), exiting...", addr, clock);
            return addr;
        }

        reg_writeback(&read_pipeline, &mut reg, clock);

        read_pipeline = write_pipeline;

        clock += 1;
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    use consts;
    use instruction::Instruction;
    use memory::instruction::TestInstructionMemory;


    /// Tests forwarding to ALU from EX/MEM and MEM/WB pipeline registers.
    ///
    /// Also tests that data to be written to a register in a certain clock
    /// cycle is made available to the instruction decode/register read stage.
    ///
    /// See Patterson & Hennessy pgs 297-302 for a description of this sequence
    /// and the logic associated with data hazard detection.
    #[test]
    fn forwarding() {
        let insn1 = Instruction::new(0x40_30_81_33); // sub x2, x1, x3
        let insn2 = Instruction::new(0x00_51_76_33); // and x12, x2, x5
        let insn3 = Instruction::new(0x00_23_66_b3); // or x13, x6, x2
        let insn4 = Instruction::new(0x00_21_07_33); // add x14, x2, x2
        let insn5 = Instruction::new(0x06_f1_12_23); // sh, x15, 100(x2)

        let insns = vec![
            insn1.as_u32(),
            insn2.as_u32(),
            insn3.as_u32(),
            insn4.as_u32(),
            insn5.as_u32(),
            consts::NOP,
            consts::NOP,
            consts::NOP,
            consts::HALT,
            consts::NOP,
            consts::NOP,
            consts::NOP,
        ];

        let insn_memory = TestInstructionMemory::new(insns);
        let mut data_memory = DataMemory::new(1024);
        let mut registers = RegisterFile::new(0x0);

        // Set initial registers so that sub x2, x1, x3 -> x2 = 1
        registers.gpr[1].write(2);
        registers.gpr[3].write(1);
        registers.gpr[15].write(0xffff);

        let halt_addr = run(&insn_memory, &mut data_memory, &mut registers);

        assert_eq!(halt_addr, 0x20);
        assert_eq!(registers.gpr[2].read(), 1); // x2 == 1
        assert_eq!(registers.gpr[3].read(), 1); // x3 == 1
        assert_eq!(registers.gpr[12].read(), 0); // x12 == 0
        assert_eq!(registers.gpr[13].read(), 1); // x13 == 1
        assert_eq!(registers.gpr[14].read(), 2); // x14 == 2

        assert_eq!(data_memory.read(100, consts::WORD_SIZE), 0x00ffff00);
    }

    /// Tests load-use hazard detection and bubble insertion.
    ///
    /// See Patterson & Hennessy pgs 303-306 for a description of this sequence
    /// and the logic associated with load-use hazard detection.
    #[test]
    fn bubble() {
        let insn1 = Instruction::new(0x01_40_a1_03); // lw, x2, 20(x1)
        let insn2 = Instruction::new(0x00_51_72_33); // and x4, x2, x5
        let insn3 = Instruction::new(0x00_61_64_33); // or x8, x2, x6
        let insn4 = Instruction::new(0x00_22_04_b3); // add x9, x4, x2
        let insn5 = Instruction::new(0x40_73_00_b3); // sub x1, x6, x7

        let insns = vec![
            insn1.as_u32(),
            insn2.as_u32(),
            insn3.as_u32(),
            insn4.as_u32(),
            insn5.as_u32(),
            consts::NOP,
            consts::NOP,
            consts::NOP,
            consts::HALT,
            consts::NOP,
            consts::NOP,
            consts::NOP,
        ];

        let insn_memory = TestInstructionMemory::new(insns);
        let mut data_memory = DataMemory::new(1024);
        let mut registers = RegisterFile::new(0x0);

        data_memory.write(20, consts::WORD_SIZE, 5);

        registers.gpr[4].write(1);
        registers.gpr[5].write(3);
        registers.gpr[6].write(2);
        registers.gpr[7].write(1);

        let halt_addr = run(&insn_memory, &mut data_memory, &mut registers);

        assert_eq!(halt_addr, 0x20);
        assert_eq!(registers.gpr[4].read(), 1);
        assert_eq!(registers.gpr[8].read(), 7);
        assert_eq!(registers.gpr[9].read(), 6);
        assert_eq!(registers.gpr[1].read(), 1);
    }

}
