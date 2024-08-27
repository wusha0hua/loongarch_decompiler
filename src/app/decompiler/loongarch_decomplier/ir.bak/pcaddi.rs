use crate::loongarch_decomplier::*;

pub fn pcaddi(insn: AssemblyInstruction, irs: &mut Vec<Ir>) {
    let mut ir_operand1 = IrOperand::None;
    if let Some(operand) = insn.operand1 {
        ir_operand1 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
    }

    let mut pc = IrOperand::Imm(ir::Immediate::from(insn.address as isize, 64, false));

    let ir = Ir::from(Some(insn.address), IrOpcode::ASGN, ir_operand1.clone(), pc, IrOperand::None);
    irs.push(ir);

    let mut ir_operand2 = IrOperand::None;
    if let Some(operand) = insn.operand2 {
        ir_operand2 = IrOperand::Imm(ir::Immediate::from((operand.value as isize) << 2, 64, true));
    }

    let ir = Ir::from(None, IrOpcode::ADD, ir_operand1.clone(), ir_operand1, ir_operand2);
    irs.push(ir);
}
