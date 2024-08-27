use crate::loongarch_decomplier::*;

pub fn bnez(insn: AssemblyInstruction, irs: &mut Vec<Ir>) {
    let mut operand1 = ir::CondOeprand::None;
    if let Some(operand) = insn.operand1 {
        operand1 = CondOeprand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
    } 

    let mut operand2 = IrOperand::None;
    if let Some(operand) = insn.operand2 {
        operand2 = IrOperand::Off(ir::Offset::from((operand.value as isize) << 2, None));
    }

    let imm = CondOeprand::Imm(ir::Immediate::from(0, 64, false));
    let condiction = IrOperand::Cond(ir::Condiction::from(ir::Relation::Equal, operand1, imm));

    let next = IrOperand::Off(ir::Offset::from(4, None));
    let ir = Ir::from(Some(insn.address), IrOpcode::JCC, condiction, next, operand2);
    irs.push(ir);
}
