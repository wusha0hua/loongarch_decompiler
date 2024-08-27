use crate::loongarch_decomplier::*;
pub fn sra_w(insn: AssemblyInstruction, irs: &mut Vec<Ir>) {
    let mut ir_operand1 = IrOperand::None;
    let mut ir_operand64 = IrOperand::None;
    if let Some(operand) = insn.operand1 {
        ir_operand1 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 32, None));
        ir_operand64 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 64, None));
    }
    
    let mut ir_operand2 = IrOperand::None;
    if let Some(operand) = insn.operand2 {
        ir_operand2 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 32, None));
    }
    
    let mut ir_operand3 = IrOperand::None;
    if let Some(operand) = insn.operand3 {
        ir_operand3 = IrOperand::Reg(ir::Register::from(Register::GR(operand.value), 32, None));
    }
    let ir = Ir::from(Some(insn.address), IrOpcode::SRA, ir_operand1.clone(), ir_operand2, ir_operand3);
    irs.push(ir);

    let ir = Ir::from(None, IrOpcode::SignExtend(64, 32), ir_operand64, ir_operand1, IrOperand::None);
    irs.push(ir);
}    

