#[allow(unused)]
use super::*;

pub fn dbar(code: u32, address: usize, symbol: HashMap<usize, SymbolRecord>) -> AssemblyInstruction {
    let mut assembly_instruction = AssemblyInstruction::new();
    assembly_instruction.opcode = Opcode::DBAR;

    let operand = Operand {
        operand_type: OperandType::UnsignedImm,
        value : (code as usize) & (1 << 15),
    };

    assembly_instruction.operand1 = Some(operand);
    assembly_instruction
}
