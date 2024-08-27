#[allow(unused)]
use super::*;

pub fn ertn(code: u32, address: usize, symbol: HashMap<usize, SymbolRecord>) -> AssemblyInstruction {
    let mut assembly_instruction = AssemblyInstruction::new();
    assembly_instruction.opcode = Opcode::ERTN; 
    assembly_instruction
}
