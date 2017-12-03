//! Data hazard detection.


use consts::{RS1_MASK, RS1_SHIFT, RS2_MASK, RS2_SHIFT};
use instruction::Instruction;
use pipeline::Pipeline;


/// Indicates ALU src1 should be forwarded from the previous ALU result.
///
/// See Patterson & Hennessy pg 300.
pub fn ex_hazard_src1(pl: &Pipeline) -> bool {
    pl.ex_mem.insn.semantics.reg_write &&
        (pl.ex_mem.insn.fields.rd != Some(0)) &&
        (pl.ex_mem.insn.fields.rd == pl.id_ex.insn.fields.rs1)
}


/// Indicates ALU src2 should be forwarded from the previous ALU result.
///
/// See Patterson & Hennessy pg 300.
pub fn ex_hazard_src2(pl: &Pipeline) -> bool {
    pl.ex_mem.insn.semantics.reg_write &&
        (pl.ex_mem.insn.fields.rd != Some(0)) &&
        (pl.ex_mem.insn.fields.rd == pl.id_ex.insn.fields.rs2)
}


/// Indicates ALU src1 should be forwarded from data memory or earlier result.
///
/// See Patterson & Hennessy pg 301.
pub fn mem_hazard_src1(pl: &Pipeline) -> bool {
    pl.mem_wb.insn.semantics.reg_write &&
        (pl.mem_wb.insn.fields.rd != Some(0)) &&
        (!(pl.ex_mem.insn.semantics.reg_write &&
               (pl.ex_mem.insn.fields.rd != Some(0)) &&
               (pl.ex_mem.insn.fields.rd == pl.id_ex.insn.fields.rs1))) &&
        (pl.mem_wb.insn.fields.rd == pl.id_ex.insn.fields.rs1)
}


/// Indicates ALU src2 should be forwarded from data memory or earlier result.
///
/// See Patterson & Hennessy pg 301.
pub fn mem_hazard_src2(pl: &Pipeline) -> bool {
    pl.mem_wb.insn.semantics.reg_write &&
        (pl.mem_wb.insn.fields.rd != Some(0)) &&
        (!(pl.ex_mem.insn.semantics.reg_write &&
               (pl.ex_mem.insn.fields.rd != Some(0)) &&
               (pl.ex_mem.insn.fields.rd == pl.id_ex.insn.fields.rs2))) &&
        (pl.mem_wb.insn.fields.rd == pl.id_ex.insn.fields.rs2)
}


/// Indicates a load-use hazard that will require a pipeline stall.
pub fn load_hazard(pl: &Pipeline) -> bool {
    // Before decode stage, rs1 and rs2 need to be extracted manually
    let if_id_rs1 = Some((pl.if_id.raw_insn & RS1_MASK) >> RS1_SHIFT);
    let if_id_rs2 = Some((pl.if_id.raw_insn & RS2_MASK) >> RS2_SHIFT);

    pl.id_ex.insn.semantics.mem_read &&
        ((pl.id_ex.insn.fields.rd == if_id_rs1) ||
             (pl.id_ex.insn.fields.rd == if_id_rs2))
}


/// Indicates src1 register was just written to and should be forwarded.
///
/// See Patterson & Hennessy pg 301.
pub fn reg_hazard_src1(insn: &Instruction, pl: &Pipeline) -> bool {
    insn.fields.rs1 != Some(0) && pl.mem_wb.insn.semantics.reg_write &&
        (pl.mem_wb.insn.fields.rd == insn.fields.rs1)
}


/// Indicates src2 register was just written to and should be forwarded.
///
/// See Patterson & Hennessy pg 301.
pub fn reg_hazard_src2(insn: &Instruction, pl: &Pipeline) -> bool {
    insn.fields.rs2 != Some(0) && pl.mem_wb.insn.semantics.reg_write &&
        (pl.mem_wb.insn.fields.rd == insn.fields.rs2)
}
