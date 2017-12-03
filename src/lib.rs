//! Simulator components for RISC-V 32I instruction set.


pub mod alu;
pub mod ca_simulator;
pub mod consts;
pub mod hazards;
pub mod immediates;
pub mod ia_simulator;
pub mod instruction;
pub mod memory;
pub mod pipeline;
pub mod register;
pub mod stages;

#[macro_use]
extern crate log;

extern crate regex;
