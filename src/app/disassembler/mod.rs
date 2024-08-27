mod loongarch_disassembler;
pub use crate::app::disassembler::loongarch_disassembler::instruction::*;
use crate::app::elf::SymbolRecord;
use crate::app::elf::SectionRecord;
//mod record;
//use record::*;
//use crate::loongarch_disassembler::record::*;
//use crate::app::disassembler::loongarch_disassembler::record::*;

use std::io::{Read, Write};
use std::fs::File;
use std::collections::HashMap;
//pub use serde_json;
//pub use serde::{Serialize, Deserialize};
//pub use serde_derive::Serialize;

use crate::app;
use crate::app::elf;

#[derive(Debug, Clone)]
pub struct DisassemblerInfo {
    pub assembly_instructions: HashMap<String, Vec<AssemblyInstruction>>, 
    pub symbols_map: HashMap<u64, SymbolRecord>, 
}

impl DisassemblerInfo {
    pub fn new() -> Self {
        Self {
            assembly_instructions: HashMap::new(),
            symbols_map: HashMap::new(),
        }
    }

    pub fn from(elf: &elf::Elf) -> Result<Self, ()> {
        /*
        let mut file_symbol = File::open("/disk/repository/loong/loongarch-disassembler/src/symbol.json").unwrap();
        let mut symbol_str = String::new();
        file_symbol.read_to_string(&mut symbol_str).unwrap();
        let symbols: Vec<SymbolRecord> = serde_json::from_str(&symbol_str).unwrap();
        */
        let symbols: Vec<SymbolRecord> = elf.symbols.clone();
        let mut symbols_map: HashMap<u64, SymbolRecord> = HashMap::new();
        for symbol in symbols {
            symbols_map.insert(symbol.offset, symbol);
        }

        /*
        let mut file = File::create("/disk/repository/loong/loongarch-disassembler/src/symbol_map.json").unwrap();
        let symbols_map_str = serde_json::to_string_pretty(&symbols_map).unwrap();
        file.write(symbols_map_str.as_bytes()).unwrap();
        */

        /*
        let mut file_section = File::open("/disk/repository/loong/loongarch-disassembler/src/section.json").unwrap();
        let mut section_str = String::new();
        file_section.read_to_string(&mut section_str).unwrap();
        let sections: Vec<SectionRecord> = serde_json::from_str(&section_str).unwrap();
        */
        let sections: Vec<SectionRecord> = elf.sections.clone();

        let mut section_insns = HashMap::<String, Vec<AssemblyInstruction>>::new();
        /*
        let mut file = File::create("/disk/repository/loong/loongarch-disassembler/src/assembly_instructions").unwrap();
        */
        for section in sections {
            println!("section <{}>: ", section.name);
            match dissam(&section.bytes, section.vaddr, &mut symbols_map) {
                Ok(mut insns) => {
                    for insn in &insns {
                        println!("{}", insn);
                    }

                    if section.name == ".plt".to_string() {
                        //analyse_plt(&mut insns, &mut symbols_map);
                        section_insns.insert(".plt".to_string(), insns);
                    } else if section.name == ".text".to_string() {
                        section_insns.insert(".text".to_string(), insns);
                    }

                }
                Err(_) => return Err(()),
            }
        }

        /*
        let json_str = serde_json::to_string_pretty(&section_insns).unwrap();
        file.write(json_str.as_bytes()).unwrap();
        */

        Ok(Self {
            assembly_instructions: section_insns,
            symbols_map,
        })
    }
}

/*
fn main() {
    /*
    let binary: u32 = 0x1c0000f9;
    let bytes = binary.to_le_bytes();
    match dissam(&bytes, 0, &mut HashMap::<usize, SymbolRecord>::new()) {
        Ok(insns) => {
            for insn in insns {
                println!("{}", insn);
            }
        }
        Err(_) => panic!("error"),
    }
    */
    let mut file_symbol = File::open("/disk/repository/loong/loongarch-disassembler/src/symbol.json").unwrap();
    let mut symbol_str = String::new();
    file_symbol.read_to_string(&mut symbol_str).unwrap();
    let symbols: Vec<SymbolRecord> = serde_json::from_str(&symbol_str).unwrap();
    let mut symbols_map: HashMap<usize, SymbolRecord> = HashMap::new();
    for symbol in symbols {
        symbols_map.insert(symbol.offset, symbol);
    }

    let mut file = File::create("/disk/repository/loong/loongarch-disassembler/src/symbol_map.json").unwrap();
    let symbols_map_str = serde_json::to_string_pretty(&symbols_map).unwrap();
    file.write(symbols_map_str.as_bytes()).unwrap();

    let mut file_section = File::open("/disk/repository/loong/loongarch-disassembler/src/section.json").unwrap();
    let mut section_str = String::new();
    file_section.read_to_string(&mut section_str).unwrap();
    let sections: Vec<SectionRecord> = serde_json::from_str(&section_str).unwrap();

    let mut section_insns = HashMap::<String, Vec<AssemblyInstruction>>::new();
    let mut file = File::create("/disk/repository/loong/loongarch-disassembler/src/assembly_instructions").unwrap();
    for section in sections {
        println!("section <{}>: ", section.name);
        match dissam(&section.bytes, section.vaddr, &mut symbols_map) {
            Ok(mut insns) => {
                for insn in &insns {
                    println!("{}", insn);
                }

                if section.name == ".plt".to_string() {
                    //analyse_plt(&mut insns, &mut symbols_map);
                    section_insns.insert(".plt".to_string(), insns);
                } else if section.name == ".text".to_string() {
                    section_insns.insert(".text".to_string(), insns);
                }

            }
            Err(_) => panic!("dissam error\n"),
        }
    }

    let json_str = serde_json::to_string_pretty(&section_insns).unwrap();
    file.write(json_str.as_bytes()).unwrap();

    /*
    let mut file_section = File::open("/disk/repository/loong/loongarch-disassembler/src/section.json").unwrap();
    match dissam(&bytes, 0, &symbols) {
        Ok(insns) => {
            let mut i = 1;
            let mut file = File::create("instruction").unwrap();
            for insn in insns {
                file.write(format!("{}\n", insn).as_bytes()).unwrap();
                i += 1;
            }
        }
        Err(_) => panic!("error"), 
    }
    */
}


*/
