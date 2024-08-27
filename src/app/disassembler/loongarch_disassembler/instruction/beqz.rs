#[allow(unused)]
use super::*;
use super::super::*;

pub fn beqz(code: u32, address: u64, symbol: &HashMap<u64, SymbolRecord>) -> AssemblyInstruction {
    let mut assembly_instruction = AssemblyInstruction::new();
    assembly_instruction.opcode = Opcode::BEQZ;
	assembly_instruction.address = address;


    let mut operand = Operand {
        operand_type: OperandType::GeneralRegister,
        value: 0,
		symbol: None,
    };

    operand.value = (code as u64 >> 5) & ((1 << 5) - 1);
    assembly_instruction.regs_read.push(Register::GR(operand.value));
    assembly_instruction.operand1 = Some(operand.clone());

    operand.operand_type = OperandType::Offset;
    let b = (code as u64 >> 10) & ((1 << 16) - 1);
    let c = code as u64 & ((1 << 5) - 1);
    let value = b | (c << 16);
    operand.value = data_convert::sign_extend(value, 21) as u64;
    assembly_instruction.operand2 = Some(operand);


	if let Some(record) = symbol.get(&address) {
		assembly_instruction.label = Some(record.name.clone());
	}
	
    assembly_instruction
}
