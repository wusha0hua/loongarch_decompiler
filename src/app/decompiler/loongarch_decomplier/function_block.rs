use super::*;
use crate::app::elf::{SymbolRecord, SymbolType};


#[derive(Debug, Clone)]
pub struct FunctionBlock {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub instruction: Vec<AssemblyInstruction>,
}

#[derive(Debug, Clone)]
pub struct FunctionBlockIr {
    pub name: String,
    pub address: u64,
    pub size: u64,
    //pub instruction: Vec<Ir>,
}

impl FunctionBlock {
    pub fn from(assembly_instructions: Vec<AssemblyInstruction>, symbols_map: &HashMap<u64, SymbolRecord>) -> Vec<FunctionBlock> {
        let mut function_blocks = Vec::<FunctionBlock>::new();
        for symbol in symbols_map {
            let mut instruction = Vec::<AssemblyInstruction>::new();
            if symbol.1.sym_type == SymbolType::Func && symbol.1.size != 0 {
                let num = symbol.1.size / 4;
                let mut i = 0;
                for assembly_instruction in &assembly_instructions {
                    if assembly_instruction.address >= symbol.1.offset {
                        instruction.push(assembly_instruction.clone());
                        i += 1;
                    }
                    if i == num {
                        break;
                    }
                }

                let function_block = FunctionBlock {
                    name: symbol.1.name.clone(),
                    address: symbol.1.offset,
                    size: symbol.1.size,
                    instruction,
                };

                function_blocks.push(function_block);
            }

        }
        function_blocks

    }
}
