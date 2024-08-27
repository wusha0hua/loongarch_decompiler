#[allow(unused)]
use super::*;
use super::super::*;

pub fn pcaddu12i(code: u32, address: u64, symbol: &HashMap<u64, SymbolRecord>) -> AssemblyInstruction {
	let mut assembly_instruction = AssemblyInstruction::new();
	assembly_instruction.opcode = Opcode::PCADDU12I;
	assembly_instruction.address = address;


	let mut operand = Operand {
		operand_type: OperandType::GeneralRegister,
		value: 0,
		symbol: None,
	};

	operand.value = (code as u64) & ((1 << 5) - 1);
    assembly_instruction.regs_write.push(Register::GR(operand.value));
	assembly_instruction.operand1 = Some(operand.clone());

	let value = (code as u64 >> 5) & ((1 << 20) - 1);
    operand.value = sign_extend(value, 20) as u64;
	operand.operand_type = OperandType::SignedImm;
	assembly_instruction.operand2 = Some(operand.clone());


	if let Some(record) = symbol.get(&address) {
		assembly_instruction.label = Some(record.name.clone());
	}
	
	assembly_instruction
}
