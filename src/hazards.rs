//! Data hazard detection.


use consts::{RS1_MASK, RS1_SHIFT, RS2_MASK, RS2_SHIFT};
use pipeline::{IfIdRegister, IdExRegister, ExMemRegister, MemWbRegister};


/// Indicates ALU src1 should be forwarded from the previous ALU result.
///
/// See Patterson & Hennessy pg 300.
pub fn ex_hazard_src1(id_ex: IdExRegister, ex_mem: ExMemRegister) -> bool {
    ex_mem.insn.semantics.reg_write && (ex_mem.insn.fields.rd != Some(0)) &&
        (ex_mem.insn.fields.rd == id_ex.insn.fields.rs1)
}


/// Indicates ALU src2 should be forwarded from the previous ALU result.
///
/// See Patterson & Hennessy pg 300.
pub fn ex_hazard_src2(id_ex: IdExRegister, ex_mem: ExMemRegister) -> bool {
    ex_mem.insn.semantics.reg_write && (ex_mem.insn.fields.rd != Some(0)) &&
        (ex_mem.insn.fields.rd == id_ex.insn.fields.rs2)
}


/// Indicates ALU src1 should be forwarded from data memory or earlier result.
///
/// See Patterson & Hennessy pg 301.
pub fn mem_hazard_src1(
    id_ex: IdExRegister,
    ex_mem: ExMemRegister,
    mem_wb: MemWbRegister,
) -> bool {
    mem_wb.insn.semantics.reg_write && (mem_wb.insn.fields.rd != Some(0)) &&
        (!(ex_mem.insn.semantics.reg_write &&
               (ex_mem.insn.fields.rd != Some(0)) &&
               (ex_mem.insn.fields.rd == id_ex.insn.fields.rs1))) &&
        (mem_wb.insn.fields.rd == id_ex.insn.fields.rs1)
}


/// Indicates ALU src2 should be forwarded from data memory or earlier result.
///
/// See Patterson & Hennessy pg 301.
pub fn mem_hazard_src2(
    id_ex: IdExRegister,
    ex_mem: ExMemRegister,
    mem_wb: MemWbRegister,
) -> bool {
    mem_wb.insn.semantics.reg_write && (mem_wb.insn.fields.rd != Some(0)) &&
        (!(ex_mem.insn.semantics.reg_write &&
               (ex_mem.insn.fields.rd != Some(0)) &&
               (ex_mem.insn.fields.rd == id_ex.insn.fields.rs2))) &&
        (mem_wb.insn.fields.rd == id_ex.insn.fields.rs2)
}


/// Indicates a load-use hazard that will require a pipeline stall.
pub fn load_hazard(if_id: IfIdRegister, id_ex: IdExRegister) -> bool {
    // Before decode stage, rs1 and rs2 need to be extracted manually
    let if_id_rs1 = Some((if_id.raw_insn & RS1_MASK) >> RS1_SHIFT);
    let if_id_rs2 = Some((if_id.raw_insn & RS2_MASK) >> RS2_SHIFT);

    id_ex.insn.semantics.mem_read &&
        ((id_ex.insn.fields.rd == if_id_rs1) ||
             (id_ex.insn.fields.rd == if_id_rs2))
}
