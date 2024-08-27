#[allow(unused)]
use super::*;

pub fn idle(code: u32, address: usize, symbol: HashMap<usize, SymbolRecord>) -> AssemblyInstruction {
    let mut assembly_instruction = AssemblyInstruction::new();
    assembly_instruction.opcode = Opcode::IDLE;
    let mut operand = Operand {
        operand_type: OperandType::FloatRegister,
        value: (code as usize) & (1 << 15),
    };
    assembly_instruction.operand3 = Some(operand);

    assembly_instruction
}
