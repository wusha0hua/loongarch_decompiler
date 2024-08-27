#[allow(unused)]
use super::*;

pub fn movgr2fr_w(code: u32, address: usize, symbol: HashMap<usize, SymbolRecord>) -> AssemblyInstruction {
    let mut assembly_instruction = AssemblyInstruction::new();
    assembly_instruction.opcode = Opcode::MOVGR2FR_W;
    
    let mut operand = Operand {
        operand_type: OperandType::FloatRegister,
        value: 0,
    };

    operand.value = (code & ((1 << 5) - 1)) as usize;
    assembly_instruction.operand1 = Some(operand.clone());

    operand.operand_type = OperandType::GeneralRegister;
    operand.value = (code as usize >> 5) & ((1 << 5) - 1);
    assembly_instruction.operand2 = Some(operand);

    assembly_instruction
}
