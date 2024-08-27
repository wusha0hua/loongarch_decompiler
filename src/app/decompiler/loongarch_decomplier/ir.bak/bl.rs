use crate::loongarch_decomplier::*;

pub fn bl(insn: AssemblyInstruction, irs: &mut Vec<Ir>) {
    let mut ir_operand = IrOperand::None;
    if let Some(operand) = insn.operand1 {
        ir_operand = IrOperand::Off(ir::Offset::from((operand.value as isize) << 2, None));
    }

    let ir = Ir::from(Some(insn.address), IrOpcode::BL, ir_operand, IrOperand::None, IrOperand::None);
    irs.push(ir);

}
