use crate::loongarch_decomplier::*;

pub fn addi_d(insn: AssemblyInstruction, irs: &mut Vec<Ir>) {
    let address = Some(insn.address);
    let opcode = IrOpcode::ADD;
    
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
        ir_operand3 = IrOperand::Imm(Immediate::from(operand.value as isize, 64, true));
    }
    
    let ir = Ir::from(address, opcode, ir_operand1, ir_operand2, ir_operand3);
    irs.push(ir);
}
