use crate::loongarch_decomplier::*;
pub fn srli_w(insn: AssemblyInstruction, irs: &mut Vec<Ir>) {
    let mut ir_operand1 = IrOperand::None;
    if let Some(operand) = insn.operand1 {
        ir_operand1 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 32, None));
    }
    
    let mut ir_operand2 = IrOperand::None;
    if let Some(operand) = insn.operand2 {
        ir_operand2 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 32, None));
    }
    
    let mut ir_operand3 = IrOperand::None;
    if let Some(operand) = insn.operand3 {
        ir_operand3 = IrOperand::Imm(ir::Immediate::from(operand.value as isize, 32, false));
    }
    let ir = Ir::from(Some(insn.address), IrOpcode::SRL, ir_operand1.clone(), ir_operand2, ir_operand3);
    irs.push(ir);

}    

