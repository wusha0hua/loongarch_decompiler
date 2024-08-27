use crate::loongarch_decomplier::*;

pub fn jirl(insn: AssemblyInstruction, irs: &mut Vec<Ir>) {
    let mut ir_operand1 = IrOperand::None;
    if let Some(operand) = insn.operand1 {
        ir_operand1 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
    }


    let mut ir_operand3 = IrOperand::None;
    if let Some(operand) = insn.operand3 {
        ir_operand3 = IrOperand::Off(ir::Offset::from((operand.value as isize)<< 2, None));
    }

    let ir = Ir::from(Some(insn.address), IrOpcode::JIRL, ir_operand, IrOperand::None, IrOperand::None);
    irs.push(ir);

}
