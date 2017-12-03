//! Pipeline definition.


use instruction::Instruction;

pub mod stages;


/// Pipeline holding four inter-stage registers
#[derive(Clone, Copy, Debug)]
pub struct Pipeline {
    pub if_id: IfIdRegister,
    pub id_ex: IdExRegister,
    pub ex_mem: ExMemRegister,
    pub mem_wb: MemWbRegister,
}


impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            if_id: IfIdRegister::new(),
            id_ex: IdExRegister::new(),
            ex_mem: ExMemRegister::new(),
            mem_wb: MemWbRegister::new(),
        }
    }
}


/// Pipeline register between instruction fetch and instruction decode stages.
#[derive(Clone, Copy, Debug)]
pub struct IfIdRegister {
    /// Program Counter
    pub pc: u32,

    /// Raw instruction
    pub raw_insn: u32,
}


impl IfIdRegister {
    pub fn new() -> IfIdRegister {
        IfIdRegister {
            pc: 0,
            raw_insn: 0x00_00_00_13, // NOP
        }
    }
}


/// Pipeline register between instruction decode and execution stages.
#[derive(Clone, Copy, Debug)]
pub struct IdExRegister {
    pub pc: u32,
    pub insn: Instruction,
    pub rs1: i32,
    pub rs2: i32,
}


impl IdExRegister {
    pub fn new() -> IdExRegister {
        IdExRegister {
            pc: 0,
            insn: Instruction::default(),
            rs1: 0,
            rs2: 0,
        }
    }
}


/// Pipeline register between execution and memory stages.
#[derive(Clone, Copy, Debug)]
pub struct ExMemRegister {
    pub pc: u32,
    pub insn: Instruction,
    pub alu_result: i32,
    pub rs2: i32,
    pub halt_addr: Option<usize>,
}


impl ExMemRegister {
    pub fn new() -> ExMemRegister {
        ExMemRegister {
            pc: 0,
            insn: Instruction::default(),
            alu_result: 0,
            rs2: 0,
            halt_addr: None,
        }
    }
}


/// Pipeline register between memory and writeback stages.
#[derive(Clone, Copy, Debug)]
pub struct MemWbRegister {
    pub pc: u32,
    pub insn: Instruction,
    pub alu_result: i32,
    pub mem_result: u32,
}


impl MemWbRegister {
    pub fn new() -> MemWbRegister {
        MemWbRegister {
            pc: 0,
            insn: Instruction::default(),
            alu_result: 0,
            mem_result: 0,
        }
    }
}
