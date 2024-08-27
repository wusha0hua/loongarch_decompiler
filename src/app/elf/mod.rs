mod loongarch64_elf;
mod loongarch_result;
mod data_convert;
mod elf_info;
pub use loongarch64_elf::*;
pub use loongarch_result::*;
use data_convert::*;

use std::io::{Read, Write};
use std::fs::File;
use std::collections::HashMap;

use crate::app;

#[derive(Debug, Clone)]
pub struct Elf {
    pub sections: Vec<SectionRecord>,
    pub symbols: Vec<SymbolRecord>,
    pub dyn_symbols: Vec<DynSymbolRecord>,
    pub data: HashMap<u64, Vec<u8>>,
}

impl Elf {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            symbols: Vec::new(),
            dyn_symbols: Vec::new(),
            data: HashMap::new(),
        }
    }

    pub fn from(buf: Vec<u8>) -> Result<Self, LoongArchError> {
        /*
        let mut file = File::open(path).unwrap();
        let mut buf = Vec::<u8>::new();
        file.read_to_end(&mut buf);
        */

        let mut elf64 = loongarch64_elf::Elf64::from(&buf);
        elf64.analyse_elf_header()?;
        //app::log(&format!("{:?}\n", elf64));
        //app::log(&format!("{}", buf.len()));
        elf64.analyse_section_tables();

        elf64.analyse_dynsym_from_section();
        elf64.analyse_symbol_from_section();
        elf64.analyse_program_tables();
        elf64.analyse_plt();
        elf64.analyse_rel_from_section();
        elf64.analyse_symbols();
        elf64.get_excutable_sections();
        elf64.analyse_data_from_program();
        
        let sections = elf64.get_section_record().clone();
        let symbols = elf64.get_symbol_record().clone();
        let dyn_symbols = elf64.get_dyn_symbol_record().clone();
        let data = elf64.get_data().clone();

        Ok(Elf {
            sections,
            symbols,
            dyn_symbols,
            data
        })
    }
}



/*
fn main() {
    let mut file = File::open("/disk/repository/loong/loongarch-elf/src/Test/if-else").unwrap(); 
    let mut buf = Vec::<u8>::new();
    file.read_to_end(&mut buf);

    let mut elf64 = loongarch64_elf::Elf64::from(&buf); 

    match elf64.analyse_elf_header() {
        Ok(_) => {}
        Err(e) => handle_loongarch_error(e),
    }

    elf64.analyse_section_tables();
    elf64.analyse_dynsym_from_section();
    elf64.analyse_symbol_from_section();

    elf64.analyse_program_tables();

    elf64.analyse_plt();
    elf64.analyse_rel_from_section();

    elf64.analyse_symbols();
    elf64.get_excutable_sections();

    elf64.analyse_data_from_program();

    let sections = elf64.get_section_record();
    
    let symbols = elf64.get_symbol_record();
    let dyn_symbols = elf64.get_dyn_symbol_record();

    let data = elf64.get_data();


    let sections_str = serde_json::to_string_pretty(sections).unwrap();
    let symbols_str = serde_json::to_string_pretty(symbols).unwrap(); 
    let dyn_symbols_str = serde_json::to_string_pretty(dyn_symbols).unwrap();
    let data_str = serde_json::to_string_pretty(data).unwrap();

    let mut file = File::create("/disk/repository/loong/loongarch-elf/src/section.json").unwrap();
    file.write(sections_str.as_bytes()).unwrap();

    let mut file = File::create("/disk/repository/loong/loongarch-elf/src/symbol.json").unwrap();
    file.write(symbols_str.as_bytes()).unwrap();

    let mut file = File::create("/disk/repository/loong/loongarch-elf/src/dyn_symbol.json").unwrap();
    file.write(dyn_symbols_str.as_bytes()).unwrap();

    let mut file = File::create("/disk/repository/loong/loongarch-elf/src/data.json").unwrap();
    file.write(data_str.as_bytes()).unwrap();

    //elf64.analyse_symbols();

    /*
    match elf64.analyse_program_tables() {
        Ok(_) => {}
        Err(e) => handle_loongarch_error(e),
    }
    */

   // elf64.analyse_dynamic_segment();

    //let mut file = File::create("code").unwrap();
   // let code = elf64.get_excutable_segment_as_bytes();
    //file.write(code);

    /*
    if let Err(lr) = elf64.analyse() {
        handle_loongarch_error(lr); 
    }
    */
}
*/
