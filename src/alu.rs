//! Arithmetic logic unit.


use instruction::Instruction;


/// Perform one ALU operation.
pub fn alu(insn: &Instruction, src1: i32, src2: i32) -> i32 {
    use self::AluOp::*;

    let value = match insn.semantics.alu_op {
        Add => src1 + src2,
        Sub => src1 - src2,
        And => src1 & src2,
        Or => src1 | src2,
        Xor => src1 ^ src2,
        BranchOnEqual => !(src1 == src2) as i32,
        BranchOnNotEqual => !(src1 != src2) as i32,
        BranchOnLessThan => !(src1 < src2) as i32,
        BranchOnLessThanUnsigned => !((src1 as u32) < (src2 as u32)) as i32,
        BranchOnGreaterOrEqual => !(src1 >= src2) as i32,
        BranchOnGreaterOrEqualUnsigned => {
            !((src1 as u32) >= (src2 as u32)) as i32
        }
        ShiftLeft => src1 << src2,
        ShiftRightLogical => ((src1 as u32) >> src2) as i32,
        ShiftRightArithmetic => src1 >> src2,
        SetOnLessThan => (src1 < src2) as i32,
        SetOnLessThanUnsigned => ((src1 as u32) < (src2 as u32)) as i32,
    };

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
