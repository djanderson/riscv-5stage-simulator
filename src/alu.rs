//! Arithmetic logic unit.


use instruction::Instruction;


/// Perform one ALU operation.
pub fn alu(insn: &Instruction, src1: i32, src2: i32, _clk: u64) -> i32 {
    use self::AluOp::*;

    let (value, overflow) = match insn.semantics.alu_op {
        Add => src1.overflowing_add(src2),
        Sub => src1.overflowing_sub(src2),
        And => (src1 & src2, false),
        Or => (src1 | src2, false),
        Xor => (src1 ^ src2, false),
        BranchOnEqual => (!(src1 == src2) as i32, false),
        BranchOnNotEqual => (!(src1 != src2) as i32, false),
        BranchOnLessThan => (!(src1 < src2) as i32, false),
        BranchOnLessThanUnsigned => (
            !((src1 as u32) < (src2 as u32)) as i32,
            false,
        ),
        BranchOnGreaterOrEqual => (!(src1 >= src2) as i32, false),
        BranchOnGreaterOrEqualUnsigned => (
            !((src1 as u32) >= (src2 as u32)) as
                i32,
            false,
        ),
        ShiftLeft => (src1 << src2, false),
        ShiftRightLogical => (((src1 as u32) >> src2) as i32, false),
        ShiftRightArithmetic => src1.overflowing_shr(src2 as u32),
        SetOnLessThan => ((src1 < src2) as i32, false),
        SetOnLessThanUnsigned => (
            ((src1 as u32) < (src2 as u32)) as i32,
            false,
        ),
    };

    if overflow {
        debug!(
            "Detected overflow {} {:?} {} (clock {})",
            src1,
            insn.semantics.alu_op,
            src2,
            _clk
        );
    }

    value
}


/// Available ALU operations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AluOp {
    // Arithmetic ops
    Add,
    Sub,
    // Logical ops
    And,
    Or,
    Xor,
    // Sets
    SetOnLessThan,
    SetOnLessThanUnsigned,
    // Shifts
    ShiftLeft,
    ShiftRightLogical,
    ShiftRightArithmetic,
    // Branches
    BranchOnEqual,
    BranchOnNotEqual,
    BranchOnLessThan,
    BranchOnLessThanUnsigned,
    BranchOnGreaterOrEqual,
    BranchOnGreaterOrEqualUnsigned,
}


impl Default for AluOp {
    fn default() -> AluOp {
        AluOp::Add
    }
}


/// Selector for ALU `src2` source.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AluSrc {
    Reg,
    Imm,
}


impl Default for AluSrc {
    fn default() -> AluSrc {
        AluSrc::Reg
    }
}
