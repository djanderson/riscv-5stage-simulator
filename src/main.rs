extern crate riscv_5stage_simulator;

use riscv_5stage_simulator::alu::{AluOp, AluSrc};
use riscv_5stage_simulator::instruction::{Function, Instruction};
use riscv_5stage_simulator::memory::data::DataMemory;
use riscv_5stage_simulator::memory::instruction::InstructionMemory;
use riscv_5stage_simulator::register::RegisterFile;

use std::env;
use std::fs::File;
use std::io::prelude::*;


const LOGO: &str = "
RISC-V 5-Stage Simulator

              vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
                  vvvvvvvvvvvvvvvvvvvvvvvvvvvv
rrrrrrrrrrrrr       vvvvvvvvvvvvvvvvvvvvvvvvvv
rrrrrrrrrrrrrrrr      vvvvvvvvvvvvvvvvvvvvvvvv
rrrrrrrrrrrrrrrrrr    vvvvvvvvvvvvvvvvvvvvvvvv
rrrrrrrrrrrrrrrrrr    vvvvvvvvvvvvvvvvvvvvvvvv
rrrrrrrrrrrrrrrrrr    vvvvvvvvvvvvvvvvvvvvvvvv
rrrrrrrrrrrrrrrr      vvvvvvvvvvvvvvvvvvvvvv
rrrrrrrrrrrrr       vvvvvvvvvvvvvvvvvvvvvv
rr                vvvvvvvvvvvvvvvvvvvvvv
rr            vvvvvvvvvvvvvvvvvvvvvvvv      rr
rrrr      vvvvvvvvvvvvvvvvvvvvvvvvvv      rrrr
rrrrrr      vvvvvvvvvvvvvvvvvvvvvv      rrrrrr
rrrrrrrr      vvvvvvvvvvvvvvvvvv      rrrrrrrr
rrrrrrrrrr      vvvvvvvvvvvvvv      rrrrrrrrrr
rrrrrrrrrrrr      vvvvvvvvvv      rrrrrrrrrrrr
rrrrrrrrrrrrrr      vvvvvv      rrrrrrrrrrrrrr
rrrrrrrrrrrrrrrr      vv      rrrrrrrrrrrrrrrr
rrrrrrrrrrrrrrrrrr          rrrrrrrrrrrrrrrrrr
rrrrrrrrrrrrrrrrrrrr      rrrrrrrrrrrrrrrrrrrr
rrrrrrrrrrrrrrrrrrrrrr  rrrrrrrrrrrrrrrrrrrrrr
";


/// Sign extend immediate value
fn imm_gen(imm: Option<u32>) -> Option<u32> {
    match imm {
        Some(v) => Some((((v as i32) << 20) >> 20) as u32),
        None => None,
    }
}


fn alu(insn: &Instruction, src1: u32, src2: u32) -> (u32, bool) {
    use AluOp::*;

    let value = match insn.semantics.alu_op {
        Add => src1 + src2,
        Sub => src1 - src2, // TODO: verify
        And => src1 & src2,
        Or => src1 | src2,
        Xor => src1 ^ src2,
        _ => 0x0,
    };

    let branch: bool = match insn.semantics.alu_op {
        BranchOnEqual => src1 == src2,
        BranchOnNotEqual => src1 != src2,
        BranchOnLessThan => src1 < src2, // FIXME: sign extend
        BranchOnLessThanUnsigned => src1 < src2,
        BranchOnGreaterOrEqual => src1 >= src2, // FIXME: sign extend
        BranchOnGreaterOrEqualUnsigned => src1 >= src2,
        _ => false,
    };

    (value, branch)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];

    let instructions: InstructionMemory;

    if let Some(filename) = args.get(1) {
        let f = File::open(filename).expect("error opening file");
        instructions = InstructionMemory::new(&f);
        println!("Successfully opened {}.", filename);
    } else {
        println!("Usage: {} <filename>", program_name);
        std::process::exit(1);
    }

    println!("{}", LOGO);

    let mut mem = DataMemory::new(1024);
    let mut reg = RegisterFile::new(0x0);

    let word: usize = 4;

    loop {
        // IF: Instruction fetch from memory
        let pc = reg.pc.read() as usize;
        let raw_insn = instructions.read(pc);

        // ID: Instruction decode & register read
        let mut insn = Instruction::new(raw_insn);
        let rs1 = reg.gpr[insn.fields.rs1.unwrap_or(0) as usize].read();
        let rs2 = reg.gpr[insn.fields.rs2.unwrap_or(0) as usize].read();

        // EX: Execute operation or calculate address
        insn.fields.imm = imm_gen(insn.fields.imm);
        let src1 = rs1;
        let src2 = match insn.semantics.alu_src {
            AluSrc::Reg => rs2,
            AluSrc::Imm => insn.fields.imm.unwrap(),
        };
        let (alu_result, branch_result) = alu(&insn, src1, src2);

        // MEM: Access memory operand
        let mem_result: u32 = 0;
        if insn.semantics.mem_read {
            let mem_result = mem.read(alu_result as usize, word); // FIXME, size!
        } else if insn.semantics.mem_write {
            mem.write(alu_result as usize, word, insn.fields.rs2.unwrap()); // FIXME, size!
        }

        // WB: Write result back to register
        if insn.semantics.reg_write {
            let rd = insn.fields.rd.unwrap() as usize;
            reg.gpr[rd].write(match insn.semantics.mem_to_reg {
                true => mem_result,
                false => alu_result,
            });
        }

        if insn.function == Function::Halt {
            println!("Caught halt instruction at {:#0x}, exiting...", pc);
            return;
        }

        println!("{:#0x} - {:?}", pc, insn);

        let npc = match branch_result {
            true => alu_result,
            false => (pc + word) as u32,
        };

        reg.pc.write(npc);
    }
}
