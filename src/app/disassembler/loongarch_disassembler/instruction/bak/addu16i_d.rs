#[allow(unused)]
use super::*;

pub fn addu16i_d(code: u32, address: usize, symbol: &HashMap<usize, SymbolRecord>) -> AssemblyInstruction {
    let mut assembly_instruction = AssemblyInstruction::new();
    assembly_instruction.opcode = Opcode::ADDU16I_D;
    assembly_instruction.address = address;

    let mut operand = Operand {
        operand_type: OperandType::GeneralRegister,
        value: 0,
        symbol: None,
    };

    operand.value = (code as usize) & ((1 << 5) - 1);
    assembly_instruction.regs_write.push(Register::GR(operand.value));
    assembly_instruction.operand1 = Some(operand.clone());

    operand.value = (code as usize >> 5) & ((1 << 5) - 1);
    assembly_instruction.regs_read.push(Register::GR(operand.value));
    assembly_instruction.operand2 = Some(operand.clone());

    let value = (code as usize >>10) & ((1 << 16) - 1);
    operand.value = data_convert::sign_extend(value, 16) as usize;
    operand.operand_type = OperandType::SignedImm;
    assembly_instruction.operand3 = Some(operand);


    assembly_instruction
}
