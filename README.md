# RISC-V 5-Stage Pipeline Simulator [![Travis CI Build Status][travis-badge]][travis-link] [![Rustdoc Build Status][docs-badge]][docs-link]

  [travis-link]: https://travis-ci.org/djanderson/riscv-5stage-simulator
  [travis-badge]: https://travis-ci.org/djanderson/riscv-5stage-simulator.svg?branch=master
  [docs-link]: https://djanderson.github.io/riscv-5stage-simulator
  [docs-badge]: https://img.shields.io/badge/docs-available-brightgreen.svg

RISC-V 5-Stage Pipeline Simulator is a 32-bit integer instuction set
architecture (ISA) and pipelining RISC-V simulator written in Rust. It was
written for ECEN 4593 - Computer Organization at the University of Colorado -
Boulder. The simulator is based on the design in the book Computer Organization
and Design RISC-V Edition by Patterson and Hennessy.


## Quickstart

### General usage:

 1) Follow instructions at [rustup.rs](https://rustup.rs/) to install Rust stable for your platform.
 2) Run all tests: `cargo test`
 3) Run a specific test: `cargo test ca_simulator_riscv_32i_sorting`
 4) Run the CA simulator against a disassembly file:

    ```bash
    $ cargo run tests/riscv_32i_sorting_disassembly.txt
       Compiling riscv-5stage-simulator v0.1.0 (file:///[...]/git/riscv-5stage-simulator)
        Finished dev [unoptimized + debuginfo] target(s) in 0.66 secs
         Running `target/debug/casim tests/riscv_32i_sorting_disassembly.txt`

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


    Caught HALT instruction at 0xd8, exiting...
    ```

### Debugging a test

 1) Run the test and note the name of the exectutable

    ```bash
    $ cargo test ca_simulator_riscv_32i_sorting
    ...
         Running target/debug/deps/riscv_32i_disassembly-a87e2e4f7ca5fb9b

    running 1 test
    test test_ca_simulator_riscv_32i_sorting_disassembly ... ok
    ...
    ```

 2) Run with different log levels (warn, info, debug, trace)

    ```bash
    $ RUST_LOG=trace ./target/debug/riscv_32i_disassembly-a87e2e4f7ca5fb9b ca_simulator_riscv_32i_sorting 2&> trace.out
    $ head trace.out

    running 1 test
    TRACE:riscv_5stage_simulator::pipeline::stages: Hazard: rs1 = 11960320 forwarded from EX/MEM ALU result (clock 3)
    TRACE:riscv_5stage_simulator::stages: Writeback: x[6] = 11960320 (clock 4)
    TRACE:riscv_5stage_simulator::pipeline::stages: Hazard: rs1 = 28672 forwarded from EX/MEM ALU result (clock 5)
    TRACE:riscv_5stage_simulator::pipeline::stages: Hazard: rs2 = 2920, forwarded from previous ALU result (clock 5)
    TRACE:riscv_5stage_simulator::stages: Writeback: x[6] = 2920 (clock 5)
    TRACE:riscv_5stage_simulator::stages: Writeback: x[2] = 28672 (clock 6)
    TRACE:riscv_5stage_simulator::pipeline::stages: Jump: 0x10 -> 0x48 (clock 7)
    TRACE:riscv_5stage_simulator::stages: Writeback: x[2] = 31592 (clock 7)
    $ ll -h trace.out
    -rw-rw-r-- 1 dja dja 36M Dec  4 00:31 trace.out
    ```
 3) Attach GDB

    ```bash
    $ ./scripts/enable_integration_test_debugging.sh
    Created the following files:
    ./target/debug/tests/riscv_32i_disassembly_1.txt
    ./target/debug/tests/riscv_32i_disassembly_2.txt
    ./target/debug/tests/riscv_32i_sorting_disassembly.txt
    $ rust-gdb ./target/debug/riscv_32i_disassembly-a87e2e4f7ca5fb9b
    [...]
    Reading symbols from ./target/debug/riscv_32i_disassembly-a87e2e4f7ca5fb9b...done.
    (gdb) tbreak riscv_5stage_simulator::ca_simulator::run
    Temporary breakpoint 1 at 0x3ace1: file src/ca_simulator.rs, line 19.
    (gdb) run ca_simulator_riscv_32i_sorting
    Starting program: /home/dja/git/riscv-5stage-simulator/target/debug/riscv_32i_disassembly-a87e2e4f7ca5fb9b ca_simulator_riscv_32i_sorting
    [Thread debugging using libthread_db enabled]
    Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

    running 1 test
    [New Thread 0x7ffff69ff700 (LWP 6149)]
    [Switching to Thread 0x7ffff69ff700 (LWP 6149)]

    Thread 2 "test_ca_simulat" hit Temporary breakpoint 1, riscv_5stage_simulator::ca_simulator::run (
        insns=&InstructionMemory, mem=0x7ffff69fe430, reg=0x7ffff69fe440) at src/ca_simulator.rs:19
    19      insns: &InstructionMemory,
    (gdb) n
    20      mut mem: &mut DataMemory,
    (gdb)
    21      mut reg: &mut RegisterFile,
    (gdb)
    24      let mut clock: u64 = 0;
    (gdb)
    27      let mut write_pipeline = Pipeline::new();
    (gdb) watch clock == 12345
    Hardware watchpoint 2: clock == 12345
    (gdb) c
    Continuing.

    Thread 2 "test_ca_simulat" hit Hardware watchpoint 2: clock == 12345

    Old value = false
    New value = true
    riscv_5stage_simulator::ca_simulator::run (insns=&InstructionMemory, mem=0x7ffff69fe430,
        reg=0x7ffff69fe440) at src/ca_simulator.rs:30
    ...
    ```


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
special `HALT` instruction at the end, and the simulator returns the address of
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
