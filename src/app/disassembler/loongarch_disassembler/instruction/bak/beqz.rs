#[allow(unused)]
use super::*;
use super::super::*;

pub fn beqz(code: u32, address: usize, symbol: HashMap<usize, SymbolRecord>) -> AssemblyInstruction {
    let mut assembly_instruction = AssemblyInstruction::new();
    assembly_instruction.opcode = Opcode::BEQZ;

    let mut operand = Operand {
        operand_type: OperandType::GeneralRegister,
        value: 0,
    };

    operand.value = (code as usize >> 5) & ((1 << 5) - 1);
    assembly_instruction.operand1 = Some(operand.clone());

    operand.operand_type = OperandType::Offset;
    let b = (code as usize >> 10) & ((1 << 16) - 1);
    let c = code as usize & ((1 << 5) - 1);
    let value = b | (c << 16);
    operand.value = data_convert::sign_extend(value << 2, 23) as usize;
    assembly_instruction.operand2 = Some(operand);

    assembly_instruction
}
