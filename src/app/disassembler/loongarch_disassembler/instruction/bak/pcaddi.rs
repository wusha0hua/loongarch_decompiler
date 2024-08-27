#[allow(unused)]
use super::*;
use super::super::*;

pub fn pcaddi(code: u32, address: usize, symbol: HashMap<usize, SymbolRecord>) -> AssemblyInstruction {
	let mut assembly_instruction = AssemblyInstruction::new();
	assembly_instruction.opcode = Opcode::PCADDI;

	let mut operand = Operand {
		operand_type: OperandType::GeneralRegister,
		value: 0,
	};

	operand.value = (code as usize) & ((1 << 5) - 1);
	assembly_instruction.operand1 = Some(operand.clone());

	let value = (code as usize >> 10) & ((1 << 16) - 1);
    operand.value = data_convert::sign_extend(value, 20) as usize;
	operand.operand_type = OperandType::SignedImm;
	assembly_instruction.operand2 = Some(operand.clone());

	assembly_instruction
}
