#[allow(unused)]
use super::*;

pub fn fstgt_s(code: u32, address: usize, symbol: HashMap<usize, SymbolRecord>) -> AssemblyInstruction {
	let mut assembly_instruction = AssemblyInstruction::new();
	assembly_instruction.opcode = Opcode::FSTGT_S;

	let mut operand = Operand {
		operand_type: OperandType::FloatRegister,
		value: 0,
	};

	operand.value = (code as usize) & ((1 << 5) - 1);
	assembly_instruction.operand1 = Some(operand.clone());

	operand.value = (code as usize >> 5) & ((1 << 5) - 1);
	operand.operand_type = OperandType::GeneralRegister;
	assembly_instruction.operand2 = Some(operand.clone());

	operand.value = (code as usize >> 10) & ((1 << 5) - 1);
	assembly_instruction.operand3 = Some(operand);

	assembly_instruction
}
