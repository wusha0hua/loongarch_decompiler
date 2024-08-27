#[allow(unused)]
use super::*;

pub fn tlbwr(code: u32, address: usize, symbol: HashMap<usize, SymbolRecord>) -> AssemblyInstruction {
    let mut assembly_instruction = AssemblyInstruction::new();
    assembly_instruction.opcode = Opcode::TLBWR; 
    assembly_instruction
}
