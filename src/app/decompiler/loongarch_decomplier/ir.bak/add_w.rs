use super::*;
use super::super::*;
use crate::loongarch_decomplier::pre::*;
pub fn add_w(insn: AssemblyInstruction, irs: &mut Vec<Ir>) {
    let address = Some(insn.address);
    let opcode = IrOpcode::ADD;
    
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
    let ir_operand = ir_operand1.clone();
    let ir = Ir::from(address, opcode, ir_operand1, ir_operand2, ir_operand3);
    irs.push(ir);
    
    let address = None;
    let opcode = IrOpcode::SignExtend(64, 32);
    
    let ir = Ir::from(address, opcode, ir_operand64, ir_operand, IrOperand::None);
    irs.push(ir);
}    

