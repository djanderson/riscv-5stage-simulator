extern crate rust_risc_v_simulator;

use rust_risc_v_simulator::instruction::Instruction;
use rust_risc_v_simulator::memory::MainMemory;
use rust_risc_v_simulator::register::RegisterFile;

use std::env;
use std::fs::File;
use std::io::prelude::*;


const GUARD_INSTRUCTION: u32 = 0;
const LOGO: &str = "
Rust RISC-V Simulator

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

    if let Some(filename) = args.get(1) {
        let mut f = File::open(filename).expect("error opening file");
        println!("Successfully opened {}.", filename);
        // TODO: need memory with read/write, then load file into addrs 0, 1...
    } else {
        println!("Usage: {} <filename>", program_name);
        std::process::exit(1);
    }

    println!("{}", LOGO);

    let mut mem = MainMemory::new(1024);
    let mut reg = RegisterFile::new(0x0);

    let word = 4;

    loop {
        let pc = reg.pc.read() as usize;
        let insn = mem.read(pc, word);

        if insn == GUARD_INSTRUCTION {
            println!("Caught guard instruction, exiting...");
            return;
        }

        let parsed_insn = Instruction::new(insn);
        println!("{:?}", parsed_insn);
    }
}
