extern crate riscv_5stage_simulator;

use riscv_5stage_simulator::alu::{AluOp, AluSrc};
use riscv_5stage_simulator::instruction::{Function, Instruction, Opcode};
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
fn imm_gen(insn: &Instruction) -> Option<u32> {
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


fn alu(insn: &Instruction, src1: i32, src2: i32) -> i32 {
    use AluOp::*;

    let value = match insn.semantics.alu_op {
        Add => src1 + src2,
        Sub => src1 - src2, // TODO: verify
        And => src1 & src2,
        Or => src1 | src2,
        Xor => src1 ^ src2,
        BranchOnEqual => !(src1 == src2) as i32,
        BranchOnNotEqual => !(src1 != src2) as i32,
        BranchOnLessThan => !(src1 < src2) as i32,
        BranchOnLessThanUnsigned => !((src1 as u32) < (src2 as u32)) as i32,
        BranchOnGreaterOrEqual => !(src1 >= src2) as i32,
        BranchOnGreaterOrEqualUnsigned => {
            !((src1 as u32) >= (src2 as u32)) as i32
        }
        ShiftLeft => src1 << src2,
        ShiftRightLogical => ((src1 as u32) >> src2) as i32,
        ShiftRightArithmetic => src1 >> src2, // FIXME
        SetOnLessThan => (src1 < src2) as i32,
        SetOnLessThanUnsigned => ((src1 as u32) < (src2 as u32)) as i32,
    };

    value
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

    let mut mem = DataMemory::new(16384);
    let mut reg = RegisterFile::new(0x0);

    let word: usize = 4;

    loop {
        // IF: Instruction fetch from memory
        let pc = reg.pc.read() as usize;
        reg.pc.write((pc + word) as u32);
        let raw_insn = instructions.read(pc);

        // ID: Instruction decode & register read
        let mut insn = Instruction::new(raw_insn);
        let rs1 = reg.gpr[insn.fields.rs1.unwrap_or(0) as usize].read();
        let rs2 = reg.gpr[insn.fields.rs2.unwrap_or(0) as usize].read();

        // EX: Execute operation or calculate address
        insn.fields.imm = imm_gen(&insn);
        let src1 = rs1;
        let src2 = match insn.semantics.alu_src {
            AluSrc::Reg => rs2,
            AluSrc::Imm => insn.fields.imm.unwrap(),
        };
        let alu_result = alu(&insn, src1 as i32, src2 as i32);

        // MEM: Access memory operand
        let mut mem_result: u32 = 0;
        if insn.semantics.mem_read {
            mem_result =
                mem.read(alu_result as usize, insn.semantics.mem_size);
        } else if insn.semantics.mem_write {
            mem.write(
                alu_result as usize,
                insn.semantics.mem_size,
                insn.fields.rs2.unwrap(),
            );
        }

        // WB: Write result back to register
        if insn.semantics.reg_write {
            let rd = insn.fields.rd.unwrap() as usize;
            reg.gpr[rd].write(match insn.semantics.mem_to_reg {
                true => mem_result,
                false => {
                    match insn.opcode {
                        Opcode::Lui => insn.fields.imm.unwrap(),
                        Opcode::Jal | Opcode::Jalr => (pc + 4) as u32,
                        _ => alu_result as u32,
                    }
                }
            });
        }

        if insn.function == Function::Halt {
            println!("Caught halt instruction at {:#0x}, exiting...", pc);
            return;
        }

        println!("{:#0x} - {:?}", pc, insn);

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
