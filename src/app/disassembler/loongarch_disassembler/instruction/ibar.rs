#[allow(unused)]
use super::*;

pub fn ibar(code: u32, address: u64, symbol: &HashMap<u64, SymbolRecord>) -> AssemblyInstruction {
    let mut assembly_instruction = AssemblyInstruction::new();
    assembly_instruction.opcode = Opcode::IBAR;
	assembly_instruction.address = address;

   
    let operand = Operand {
        operand_type: OperandType::UnsignedImm,
        value: (code as u64) & (1 << 15),
        symbol: None,
    };

    assembly_instruction.operand1 = Some(operand);

	if let Some(record) = symbol.get(&address) {
		assembly_instruction.label = Some(record.name.clone());
	}
	
    assembly_instruction
}
