#[allow(unused)]
use super::*;

pub fn fcmp_cond_d(code: u32, address: u64, symbol: &HashMap<u64, SymbolRecord>) -> AssemblyInstruction {
    let mut assembly_instruction = AssemblyInstruction::new();
    let mut operand = Operand {
        operand_type: OperandType::UnsignedImm,
        value: 0,
		symbol: None,
    };
    operand.value = ((code as u64) & 7);
    assembly_instruction.operand1 = Some(operand.clone());

    operand.operand_type = OperandType::FloatRegister;
    operand.value = (code as u64 >> 5) & ((1 << 5) - 1);
    assembly_instruction.regs_read.push(Register::FR(operand.value));
    assembly_instruction.operand2 = Some(operand.clone());

    operand.value = (code as u64 >> 10) & ((1 << 5) - 1);
    assembly_instruction.regs_read.push(Register::FR(operand.value));
    assembly_instruction.operand3 = Some(operand.clone());

    assembly_instruction.opcode = match ((code as u64 >> 15) & ((1 << 5) - 1)) {
        0x8 => Opcode::FCMP_CUN_D,
        0x4 => Opcode::FCMP_CEQ_D,
        0xc => Opcode::FCMP_CUEQ_D,
        0x2 => Opcode::FCMP_CLT_D,
        0xe => Opcode::FCMP_CULT_D,
        0x6 => Opcode::FCMP_CLE_D,
        0xe => Opcode::FCMP_CULT_D,
        0x10 => Opcode::FCMP_CNE_D,
        0x14 => Opcode::FCMP_COR_D,
        0x18 => Opcode::FCMP_CUNE_D,
        0x1 => Opcode::FCMP_SAF_D,
        0x9 => Opcode::FCMP_SUN_D,
        0x5 => Opcode::FCMP_SEQ_D,
        0xd => Opcode::FCMP_SUEQ_D,
        0x3 => Opcode::FCMP_SLT_D,
        0xb => Opcode::FCMP_SULT_D,
        0x7 => Opcode::FCMP_SLE_D,
        0xf => Opcode::FCMP_SULE_D,
        0x11 => Opcode::FCMP_SNE_D,
        0x15 => Opcode::FCMP_SOR_D,
        0x19 => Opcode::FCMP_SUNE_D,
        _ => Opcode::FCMP_CAF_D,
    };


	if let Some(record) = symbol.get(&address) {
		assembly_instruction.label = Some(record.name.clone());
	}
	
    assembly_instruction
}
