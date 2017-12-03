//! A 5-stage pipelining RISC-V 32I simulator.


extern crate env_logger;
extern crate riscv_5stage_simulator;

use riscv_5stage_simulator::ca_simulator;
use riscv_5stage_simulator::memory::data::DataMemory;
use riscv_5stage_simulator::memory::instruction::DisassemblyInstructionMemory;
use riscv_5stage_simulator::register::RegisterFile;

use std::env;
use std::fs::File;


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
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];

    let instructions: DisassemblyInstructionMemory;
    let mut data_memory = DataMemory::new(8192);
    let mut registers = RegisterFile::new(0x0);

    if let Some(filename) = args.get(1) {
        let f = File::open(filename).expect("error opening file");
        instructions = DisassemblyInstructionMemory::new(&f);
    } else {
        println!("Usage: {} <filename>", program_name);
        std::process::exit(1);
    }

    println!("{}", LOGO);

    ca_simulator::run(&instructions, &mut data_memory, &mut registers);
}
