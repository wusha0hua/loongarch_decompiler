use crate::loongarch_decomplier::*;

pub fn lu12i_w(insn: AssemblyInstruction, irs: &mut Vec<Ir>) {

    let mut ir_operand1 = IrOperand::None;
    if let Some(operand) = insn.operand1 {
        ir_operand1 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
    }

    let mut ir_operand2 = IrOperand::None;
    if let Some(operand) = insn.operand2 {
        ir_operand2 = IrOperand::Imm(ir::Immediate::from((operand.value as isize) << 12, 64, true));
    }

    let ir = Ir::from(Some(insn.address), IrOpcode::ASGN, ir_operand1, ir_operand2, IrOperand::None);
    irs.push(ir);
}
