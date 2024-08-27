use crate::loongarch_decomplier::*;

pub fn ldx_d(insn: AssemblyInstruction, irs: &mut Vec<Ir>) {
    let mut ir_operand1 = IrOperand::None;
    if let Some(operand) = insn.operand1 {
        ir_operand1 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
    }

    let mut ir_operand2 = IrOperand::None;
    if let Some(operand) = insn.operand2 {
        ir_operand2 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
    }

    let mut ir_operand3 = IrOperand::None;
    if let Some(operand) = insn.operand3 {
        ir_operand3 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
    }

    let ir = Ir::from(Some(insn.address), IrOpcode::ADD, ir_operand1.clone(), ir_operand2, ir_operand3);
    irs.push(ir);

    let ir = Ir::from(None, IrOpcode::LOAD, ir_operand1.clone(), ir_operand1.clone(), IrOperand::None);
    irs.push(ir);

}
