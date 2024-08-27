#[allow(unused)]
use super::*;
use super::super::*;

pub fn bl(code: u32, address: usize, symbol: HashMap<usize, SymbolRecord>) -> AssemblyInstruction {
    let mut assembly_instruction = AssemblyInstruction::new();
    assembly_instruction.opcode = Opcode::BL;

    let mut operand = Operand {
        operand_type: OperandType::Offset,
        value: 0,
    };
    let value = ((((code as usize) & ((1 << 10) - 1)) << 16) | ((code as usize >> 10) & ((1 << 16) - 1)));
    operand.value = data_convert::sign_extend(value << 2, 28) as usize;
    assembly_instruction.operand1 = Some(operand);

    assembly_instruction
}
