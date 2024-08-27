use crate::loongarch_decomplier::*;

pub fn alsl_w(insn: AssemblyInstruction, irs: &mut Vec<Ir>) {
    let address = Some(insn.address);
    let opcode = IrOpcode::SLL;

    let mut sa = 0;
    if let Some(operand) = insn.operand4 {
        sa = operand.value;
    }

    let mut ir_operand = IrOperand::None;
    if let Some(operand) = insn.operand2 {
        ir_operand = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 32, None));
    }

    let ir = Ir::from(address, opcode, ir_operand.clone(), ir_operand.clone(), IrOperand::Imm(ir::Immediate::from(sa as isize + 1, 32, false)));
    irs.push(ir);

    let mut ir_operand3 = IrOperand::None;
    if let Some(operand) = insn.operand3 {
        ir_operand3 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 32, None));
    }

    let ir = Ir::from(None, IrOpcode::ADD, ir_operand.clone(), ir_operand.clone(), ir_operand3.clone());
    irs.push(ir);

    let mut ir_operand64 = IrOperand::None;
    if let Some(operand) = insn.operand1 {
        ir_operand64 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
    }

    let ir = Ir::from(None, IrOpcode::SignExtend(64, 32), ir_operand64, ir_operand, IrOperand::None);
    irs.push(ir);

}
