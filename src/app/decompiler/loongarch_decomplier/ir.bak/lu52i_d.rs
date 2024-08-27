use crate::loongarch_decomplier::*;

pub fn lu52i_d(insn: AssemblyInstruction, irs: &mut Vec<Ir>) {

    let mut ir_operand1 = IrOperand::None;
    let mut ir_operand64 = IrOperand::None;
    if let Some(operand) = insn.operand1 {
        ir_operand1 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
        ir_operand64 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
    }

    let ir = Ir::from(Some(insn.address), IrOpcode::AND, ir_operand1.clone(), ir_operand1.clone(), IrOperand::Imm(ir::Immediate::from(((1 << 52) - 1), 64, false)));
    irs.push(ir);

    let mut ir_operand2 = IrOperand::None;
    if let Some(operand) = insn.operand2 {
        ir_operand2 = IrOperand::Imm(ir::Immediate::from((operand.value as isize) << 52, 64, true));
    }

    let ir = Ir::from(None, IrOpcode::OR, ir_operand64, ir_operand1, ir_operand2);
    irs.push(ir);
}
