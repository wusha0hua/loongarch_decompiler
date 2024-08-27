#[allow(unused)]
use super::*;

pub fn tlbsrch(code: u32, address: usize, symbol: HashMap<usize, SymbolRecord>) -> AssemblyInstruction {
    let mut assembly_instruction = AssemblyInstruction::new();
    assembly_instruction.opcode = Opcode::TLBSRCH; 
    assembly_instruction
}
