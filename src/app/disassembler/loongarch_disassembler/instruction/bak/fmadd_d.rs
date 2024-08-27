#[allow(unused)]
use super::*;

pub fn fmadd_d(code: u32, address: usize, symbol: HashMap<usize, SymbolRecord>) -> AssemblyInstruction {
	let mut assembly_instruction = AssemblyInstruction::new();
	assembly_instruction.opcode = Opcode::FMADD_D;

	let mut operand = Operand {
		operand_type: OperandType::FloatRegister,
		value: 0,
	};

	operand.value = (code as usize) & ((1 << 5) - 1);
	assembly_instruction.operand1 = Some(operand.clone());

	operand.value = (code as usize >> 5) & ((1 << 5) - 1);
	assembly_instruction.operand2 = Some(operand.clone());

	operand.value = (code as usize >> 10) & ((1 << 5) - 1);
	assembly_instruction.operand3 = Some(operand.clone());

	operand.value = (code as usize >> 15) & ((1 << 5) - 1);
	assembly_instruction.operand4 = Some(operand);

	assembly_instruction
}
