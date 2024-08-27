#[allow(unused)]
use super::*;
use super::super::*;

pub fn bceqz(code: u32, address: u64, symbol: &mut HashMap<u64, SymbolRecord>) -> AssemblyInstruction {
    let mut assembly_instruction = AssemblyInstruction::new();
    assembly_instruction.opcode = Opcode::BCEQZ;
	assembly_instruction.address = address;


    let mut operand = Operand {
        operand_type: OperandType::UnsignedImm,
        value: 0,
		symbol: None,
    };

    operand.value = (code as u64 >> 5) & ((1 << 3) -1);
    assembly_instruction.operand1 = Some(operand.clone());

    let b = (code as u64 >> 10) & ((1 << 16) - 1);
    let c = code as u64 & ((1 << 5) - 1);
    let value = b | (c << 16);
    operand.value = data_convert::sign_extend(value << 2, 23) as u64; 
    operand.operand_type = OperandType::Offset;
    let addr = (address as isize + operand.value as isize) as u64;
    if let Some(record) = symbol.get(&addr) {
        operand.symbol = Some(record.clone());
    } else {
        operand.symbol = Some(SymbolRecord::label_from_addr(addr));
        symbol.insert(addr, SymbolRecord::label_from_addr(addr));
    }
    assembly_instruction.operand2 = Some(operand);


	if let Some(record) = symbol.get(&address) {
		assembly_instruction.label = Some(record.name.clone());
	}
	
    assembly_instruction
}
