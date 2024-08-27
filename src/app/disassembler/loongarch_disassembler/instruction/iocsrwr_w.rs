#[allow(unused)]
use super::*;

pub fn iocsrwr_w(code: u32, address: u64, symbol: &HashMap<u64, SymbolRecord>) -> AssemblyInstruction {
    let mut assembly_instruction = AssemblyInstruction::new();
    assembly_instruction.opcode = Opcode::IOCSRWR_W;
	assembly_instruction.address = address;
 
    let mut operand = Operand {
        operand_type: OperandType::GeneralRegister,
        value: 0,
		symbol: None,
    };
    
    operand.value = (code & ((1 << 5) - 1)) as u64;
    assembly_instruction.operand1 = Some(operand.clone());

    operand.value = (code as u64 >> 5) & ((1 << 5) - 1);
    assembly_instruction.operand2 = Some(operand.clone());


	if let Some(record) = symbol.get(&address) {
		assembly_instruction.label = Some(record.name.clone());
	}
	
    assembly_instruction
}
