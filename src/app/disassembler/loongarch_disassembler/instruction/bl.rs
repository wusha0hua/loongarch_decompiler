#[allow(unused)]
use super::*;
use super::super::*;

pub fn bl(code: u32, address: u64, symbol: &mut HashMap<u64, SymbolRecord>) -> AssemblyInstruction {
    let mut assembly_instruction = AssemblyInstruction::new();
    assembly_instruction.opcode = Opcode::BL;
	assembly_instruction.address = address;


    let mut operand = Operand {
        operand_type: OperandType::Offset,
        value: 0,
		symbol: None,
    };
    let value = ((((code as u64) & ((1 << 10) - 1)) << 16) | ((code as u64 >> 10) & ((1 << 16) - 1)));
    operand.value = data_convert::sign_extend(value, 26) as u64;
    let addr = (address as isize + operand.value as isize) as u64;
    if let Some(record) = symbol.get(&addr) {
        operand.symbol = Some(record.clone());
    } else {
        operand.symbol = Some(SymbolRecord::label_from_addr(addr));
        symbol.insert(addr, SymbolRecord::label_from_addr(addr));
    }
    assembly_instruction.operand1 = Some(operand);


	if let Some(record) = symbol.get(&address) {
		assembly_instruction.label = Some(record.name.clone());
	}
	
    assembly_instruction
}
