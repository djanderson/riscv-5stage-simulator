extern crate riscv_5stage_simulator;

use riscv_5stage_simulator::instruction::{Instruction, GUARD_INSTRUCTION};
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

    let word = 4;

    loop {
        let pc = reg.pc.read() as usize;
        let insn = instructions.read(pc);

        if insn == GUARD_INSTRUCTION {
            println!("Caught guard instruction, exiting...");
            return;
        }

        let parsed_insn = Instruction::new(insn);
        println!("{:?}", parsed_insn);

        reg.pc.write((pc as u32) + word);
    }
}
