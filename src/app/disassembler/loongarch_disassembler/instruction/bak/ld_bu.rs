#[allow(unused)]
use super::*;
use super::super::*;

pub fn ld_bu(code: u32, address: usize, symbol: HashMap<usize, SymbolRecord>) -> AssemblyInstruction {
	let mut assembly_instruction = AssemblyInstruction::new();
	assembly_instruction.opcode = Opcode::LD_BU;

	let mut operand = Operand {
		operand_type: OperandType::GeneralRegister,
		value: 0,
	};

	operand.value = (code as usize) & ((1 << 5) - 1);
	assembly_instruction.operand1 = Some(operand.clone());

	operand.value = (code as usize >> 5) & ((1 << 5) - 1);
	assembly_instruction.operand2 = Some(operand.clone());

	let value = (code as usize >> 10) & ((1 << 12) - 1);
    operand.value = data_convert::sign_extend(value, 12) as usize;
    operand.operand_type = OperandType::SignedImm;
	assembly_instruction.operand3 = Some(operand.clone());

	assembly_instruction
}
