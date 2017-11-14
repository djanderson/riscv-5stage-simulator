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
