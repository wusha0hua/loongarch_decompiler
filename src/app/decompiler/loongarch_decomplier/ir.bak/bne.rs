use crate::loongarch_decomplier::*;

pub fn bne(insn: AssemblyInstruction, irs: &mut Vec<Ir>) {
    let mut operand1 = ir::CondOeprand::None;
    if let Some(operand) = insn.operand1 {
        operand1 = CondOeprand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
    } 

    let mut operand2 = ir::CondOeprand::None;
    if let Some(operand) = insn.operand2 {
        operand2 = CondOeprand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
    }

    let mut operand3 = IrOperand::None;
    if let Some(operand) = insn.operand3 {
        operand3 = IrOperand::Off(ir::Offset::from((operand.value as isize) << 2, None));
    }

    let condiction = IrOperand::Cond(ir::Condiction::from(ir::Relation::Equal, operand1, operand2));

    let next = IrOperand::Off(ir::Offset::from(4, None));
    let ir = Ir::from(Some(insn.address), IrOpcode::JCC, condiction, next, operand3);
    irs.push(ir);
}
