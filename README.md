# RISC-V 5-Stage Pipeline Simulator [![Travis CI Build Status][travis-badge]][travis-link]

  [travis-link]: https://travis-ci.org/djanderson/riscv-5stage-simulator
  [travis-badge]: https://travis-ci.org/djanderson/riscv-5stage-simulator.svg?branch=master

RISC-V 5-Stage Pipeline Simulator is a 32-bit integer instuction set
architecture (ISA) and pipelining RISC-V simulator written in Rust. It was
written for ECEN 4593 - Computer Organization at CU Boulder. The simulator is
based on the design in the book Computer Organization and Design RISC-V Edition
by Patterson and Hennessy.

<p align="center">
  <img src="/docs/rusty_pipes.jpg">
</p>


## Quickstart

General usage:

 1) Follow instructions at [rustup.rs](rustup-link) to install Rust stable for your platform.
 2) Run all tests: `cargo test`
 3) Run a specific test: `cargo test ca_simulator_riscv_32i_sorting`
 4) Run the CA simulator against a disassembly file: `cargo run tests/riscv_32i_sorting_disassembly.txt`

  [rustup-link]: https://rustup.rs/


## Implemented Execution Models

 - [X] IA (instruction accurate), single cycle simulator
 - [X] CA (cycle accurate), 5-stage pipelining simulator


## Implemented Hazard Detection and Forwarding

 - [X] EX/MEM data hazard detection and ALU result forwarding
 - [X] MEM/WB data hazard detection and data memory/previous result forwarding
 - [X] Register write/read hazard detection and forwarding
 - [X] Load-use hazard detection and pipeline stall insertion


## Tests

Currently, integration tests are loaded via a disassembly loader that parses a
given disassembly file and populates the instruction memory. The loader adds a
special `HALT` instruction at the end, and the simulator return the address of
the first `HALT` instruction that it hits. Integration tests pass or fail based
on this address. The following integration tests are passing for both IA and CA
simulators:

 - [X] RISCV_32I_DISASSEMBLY_1
 - [X] RISCV_32I_DISASSEMBLY_2
 - [X] RISCV_32I_SORTING_DISASSEMBLY

In addition, there are ~30 unit tests within implementation files that test
more specific features.


## Implemented Instructions

RV32I Base Instruction Set, Version 2.0 (as defined in Volume 1: RISC-V
User-Level ISA V2.2)

 - [X] LUI
 - [X] AUIPC
 - [X] JAL
 - [X] JALR

Branches

 - [X] BEQ
 - [X] BNE
 - [X] BLT
 - [X] BGE
 - [X] BLTU
 - [X] BGEU

Loads

 - [X] LB
 - [X] LH
 - [X] LW
 - [X] LBU
 - [X] LHU

Stores

 - [X] SB
 - [X] SH
 - [X] SW

Operations on immediates

 - [X] ADDI
 - [X] SLTI
 - [X] SLTIU
 - [X] XORI
 - [X] ORI
 - [X] ANDI
 - [X] SLLI
 - [X] SRLI
 - [X] SRAI

Operations on registers

 - [X] ADD
 - [X] SUB
 - [X] SLL
 - [X] SLT
 - [X] SLTU
 - [X] XOR
 - [X] SRL
 - [X] SRA
 - [X] OR
 - [X] AND
 - [X] HALT

Control and status registers

 - [ ] FENCE
 - [ ] FENCE.I
 - [ ] ECALL
 - [ ] EBREAK
 - [ ] CSRRW
 - [ ] CSRRS
 - [ ] CSRRC
 - [ ] CSRRWI
 - [ ] CSRRSI
 - [ ] CSRRCI


## Licence

Copyright 2017 Douglas Anderson <douglas.anderson-1@colorado.edu>. Released
under GPL 3 _except for the 3 disassembly files in tests/ which are copyright
their respective authors and not covered under this license._
