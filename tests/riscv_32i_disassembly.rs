//! Integration tests using disassembly files.


extern crate riscv_5stage_simulator;


use riscv_5stage_simulator::memory::instruction::DisassemblyInstructionMemory;
use riscv_5stage_simulator::ia_simulator;

use std::fs::File;


/// Tests most common instructions is easy to debug manner.
#[test]
fn test_riscv_32i_disassembly_1() {
    let filename = "tests/riscv_32i_disassembly_1.txt";
    let f = File::open(filename).unwrap();
    let instructions = DisassemblyInstructionMemory::new(&f);
    let halt_addr = ia_simulator::run(&instructions);
    let expected_halt_addr: u32 = 0x4c0;

    assert_eq!(halt_addr, expected_halt_addr);
}


/// Tests all 32I instructions.
#[test]
fn test_riscv_32i_disassembly_2() {
    let filename = "tests/riscv_32i_disassembly_2.txt";
    let f = File::open(filename).unwrap();
    let instructions = DisassemblyInstructionMemory::new(&f);
    let halt_addr = ia_simulator::run(&instructions);
    let expected_halt_addr: u32 = 0x56c;

    assert_eq!(halt_addr, expected_halt_addr);
}
