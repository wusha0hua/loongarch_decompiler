use crate::loongarch_decomplier::*;

pub fn mulh_du(insn: AssemblyInstruction, irs: &mut Vec<Ir>) {

    let mut ir_operand1 = IrOperand::None;
    let mut ir_operand64 = IrOperand::None;
    if let Some(operand) = insn.operand1 {
        ir_operand1 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 128, None));
        ir_operand64 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
    }

    let mut ir_operand2 = IrOperand::None;
    if let Some(operand) = insn.operand2 {
        ir_operand2 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
    }

    let mut ir_operand3 = IrOperand::None;
    if let Some(operand) = insn.operand3 {
        ir_operand3 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
    }

    let ir = Ir::from(Some(insn.address), IrOpcode::MUL, ir_operand64.clone(), ir_operand2, ir_operand3);
    irs.push(ir);

    let imm = IrOperand::Imm(ir::Immediate::from(64, 64, false));
    let ir = Ir::from(None, IrOpcode::SRA, ir_operand64, imm, IrOperand::None);
    irs.push(ir);
}
