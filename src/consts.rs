//! Global constants


/// Special simulator-only instruction signal to halt simulator.
pub const HALT: u32 = 0x3f;

/// Size of a register in bytes.
pub const WORD_SIZE: usize = 4;

// Masks to isolate specific parts of the instruction using logical AND (&)
pub const FUNCT7_MASK: u32 = 0xfe000000;
pub const FUNCT3_MASK: u32 = 0x7000;
pub const RS1_MASK: u32 = 0xf8000;
pub const RS2_MASK: u32 = 0x1f00000;
pub const RD_MASK: u32 = 0xf80;
pub const OPCODE_MASK: u32 = 0x7f;
pub const BIT30_MASK: u32 = 0x40000000;

// Indices of instruction parts for shifting
pub const FUNCT7_SHIFT: u8 = 25;
pub const FUNCT3_SHIFT: u8 = 12;
pub const RS1_SHIFT: u8 = 15;
pub const RS2_SHIFT: u8 = 20;
pub const RD_SHIFT: u8 = 7;
pub const BIT30_SHIFT: u8 = 30;
