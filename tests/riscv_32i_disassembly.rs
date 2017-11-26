//! Integration tests using disassembly files.


extern crate riscv_5stage_simulator;


use riscv_5stage_simulator::{ca_simulator, ia_simulator};
use riscv_5stage_simulator::memory::data::DataMemory;
use riscv_5stage_simulator::memory::instruction::DisassemblyInstructionMemory;
use riscv_5stage_simulator::register::RegisterFile;

use std::fs::File;


/// Tests instruction-accurate simulator on most common instructions.
#[test]
fn test_ia_simulator_riscv_32i_disassembly_1() {
    let filename = "tests/riscv_32i_disassembly_1.txt";
    let f = File::open(filename).unwrap();
    let instructions = DisassemblyInstructionMemory::new(&f);
    let halt_addr = ia_simulator::run(&instructions);
    let expected_halt_addr: u32 = 0x4c0;

    assert_eq!(halt_addr, expected_halt_addr);
}


/// Tests instruction-accurate simulator all 32I instructions.
#[test]
fn test_ia_simulator_riscv_32i_disassembly_2() {
    let filename = "tests/riscv_32i_disassembly_2.txt";
    let f = File::open(filename).unwrap();
    let instructions = DisassemblyInstructionMemory::new(&f);
    let halt_addr = ia_simulator::run(&instructions);
    let expected_halt_addr: u32 = 0x56c;

    assert_eq!(halt_addr, expected_halt_addr);
}


/// Tests cycle-accurate simulator on most common instructions.
#[test]
fn test_ca_simulator_riscv_32i_disassembly_1() {
    let filename = "tests/riscv_32i_disassembly_1.txt";
    let f = File::open(filename).unwrap();
    let insns = DisassemblyInstructionMemory::new(&f);
    let mut mem = DataMemory::new(1024);
    let mut reg = RegisterFile::new(0x0);
    let halt_addr = ca_simulator::run(&insns, &mut mem, &mut reg);
    let expected_halt_addr = 0x4c0;

    assert_eq!(halt_addr, expected_halt_addr);
}


/// Tests cycle-accurate simulator on all 32I instructions.
#[test]
fn test_ca_simulator_riscv_32i_disassembly_2() {
    let filename = "tests/riscv_32i_disassembly_2.txt";
    let f = File::open(filename).unwrap();
    let insns = DisassemblyInstructionMemory::new(&f);
    let mut mem = DataMemory::new(1024);
    let mut reg = RegisterFile::new(0x0);
    let halt_addr = ca_simulator::run(&insns, &mut mem, &mut reg);
    let expected_halt_addr = 0x56c;

    assert_eq!(halt_addr, expected_halt_addr);
}
