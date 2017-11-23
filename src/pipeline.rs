 use instruction::Instruction;


/// Pipeline register between instruction fetch and instruction decode stages.
#[derive(Clone, Copy, Debug)]
pub struct IfIdRegister {
    /// Next PC
    pub npc: u32,

    /// Raw instruction
    pub raw_insn: u32,
}


impl IfIdRegister {
    pub fn new() -> IfIdRegister {
        IfIdRegister {
            npc: 0,
            raw_insn: 0x00_00_00_13, // NOP
        }
    }
}


/// Pipeline register between instruction decode and execution stages.
#[derive(Clone, Copy, Debug)]
pub struct IdExRegister {
    pub npc: u32,
    pub insn: Instruction,
    pub rs1: i32,
    pub rs2: i32,
}


impl IdExRegister {
    pub fn new() -> IdExRegister {
        IdExRegister {
            npc: 0,
            insn: Instruction::default(),
            rs1: 0,
            rs2: 0,
        }
    }
}


/// Pipeline register between execution and memory stages.
#[derive(Clone, Copy, Debug)]
pub struct ExMemRegister {
    pub npc: u32,
    pub insn: Instruction,
    pub alu_result: i32,
    pub rs2: i32,
}


impl ExMemRegister {
    pub fn new() -> ExMemRegister {
        ExMemRegister {
            npc: 0,
            insn: Instruction::default(),
            alu_result: 0,
            rs2: 0,
        }
    }
}


/// Pipeline register between memory and writeback stages.
#[derive(Clone, Copy, Debug)]
pub struct MemWbRegister {
    pub insn: Instruction,
    pub alu_result: i32,
    pub mem_result: u32,
}


impl MemWbRegister {
    pub fn new() -> MemWbRegister {
        MemWbRegister {
            insn: Instruction::default(),
            alu_result: 0,
            mem_result: 0,
        }
    }
}
