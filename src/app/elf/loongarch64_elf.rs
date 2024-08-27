mod elf64_header;
mod elf64_section;
mod elf64_sym;
mod elf64_dyn;
mod elf64_program;
mod elf64_rel;
mod elf64_plt;
pub use elf64_header::*;
pub use elf64_section::*;
pub use elf64_sym::*;
pub use elf64_dyn::*;
pub use elf64_program::*;
pub use elf64_rel::*;
pub use elf64_plt::*;


use crate::app::log;

use super::loongarch_result::*;
use super::elf_info::*;
use super::data_convert::*;

use std::convert::From;
use std::collections::HashMap;
use serde_json;
use serde::{Serialize, Deserialize};
//use serde_derive::Serialize;


#[derive(Debug, Clone)]
pub struct Elf64<'a> {
    pub elf_bytes: &'a Vec<u8>,
    pub elf_info: ElfInfo,
    pub elf_header: Elf64Ehdr,
    pub elf_sections: Vec<Elf64Shdr>,
    pub elf_programs: Vec<Elf64Phdr>,
    pub dynamic_tables: Vec<Elf64Dyn>,
    pub strtab_offset: u64,
    pub symtab_offset: u64,
    pub str_bytes: HashMap<u64, Vec<u8>>,
    pub section_name_map: HashMap<u64, String>,
    pub symbol_tables: Vec<Elf64Sym>,
    pub symbol_name_map: HashMap<u64, String>,
    pub dynsym_tables: Vec<Elf64Sym>,
    pub dynsym_name_map: HashMap<u64, String>,

    pub symbol_tables_dynamic: Vec<Elf64Sym>,
    pub symbol_name_map_dynamic: HashMap<u64, String>,

    pub symbols: Vec<SymbolRecord>,
    pub dyn_symbols: Vec<DynSymbolRecord>,
    pub sections: Vec<SectionRecord>,

    pub rels_from_section: Vec<Elf64Rel>,
    pub rels_from_segment: Vec<Elf64Rel>,

    pub relas_from_section: Vec<Elf64Rela>,
    pub relas_from_segment: Vec<Elf64Rela>,

    pub data: HashMap<u64, Vec<u8>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DynSymbolRecord {
    pub offset: u64,
    pub name: String,
    pub size: u64,
    pub sym_type: SymbolType,
    pub reloc_type: RelactionType,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SymbolRecord {
    pub offset: u64,
    pub name: String,
    pub size: u64,
    pub sym_type: SymbolType,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum SymbolType {
    Func,
    Val,
    Label,
}

#[derive(Debug, Clone, Serialize)]
pub struct SectionRecord {
    pub offset: u64,
    pub vaddr: u64,
    pub name: String,
    pub size: u64,
    pub section_type: SectionType,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum SectionType {
    Code,
    Data,
    None,
}

impl<'a> Elf64<'a> {
    pub fn analyse_elf_header(&mut self) -> Result<LoongArchOk, LoongArchError> {
        match self.elf_header.fill(self.elf_bytes) {
            Ok(info) => self.elf_info = info,
            Err(e) => return Err(e),
        };
        Ok(LoongArchOk::Ok)
    }

    pub fn analyse_program_tables(&mut self) -> Result<LoongArchOk, LoongArchError> {
        let offset = self.elf_header.e_phoff as u64;
        let num = self.elf_header.e_phnum as u64;
        for i in 0..num {
            let start = offset + i * ELF64_PHDR_SIZE;
            let end = start + ELF64_PHDR_SIZE;
            self.elf_programs.push(Elf64Phdr::from(&self.elf_bytes[start as usize..end as usize])); 
        }
        /*
        for program in &self.elf_programs {
            let offset = program.p_offset as u64;
            let vaddr = program.p_vaddr as u64;
            match program.p_type {
                PT_DYNAMIC => {
                    let mut i = 0;
                    loop {
                        let start = offset + i * ELF64_DYN_SIZE;
                        let end = start + ELF64_DYN_SIZE;
                        let dyntbl = Elf64Dyn::from(&self.elf_bytes[start..end]);
                        if dyntbl.d_tag == DT_NULL {
                            break;
                        } else {
                            self.dynamic_tables.push(dyntbl);
                        }
                        i += 1;
                    } 
                     
                    //println!("{:#?}", self.dynamic_tables);
                }
                _ => {}
            }
        }
        */

        Ok(LoongArchOk::Ok)
    }

    pub fn analyse_symbol_from_dynamic_segment(&mut self) {
        for segment in &self.elf_programs {
            if segment.p_type == PT_DYNAMIC {
                let offset = segment.p_offset as u64;
                let size = segment.p_filesz as u64;
                let num = size / ELF64_DYN_SIZE;
                let bytes = &self.elf_bytes[offset as usize..offset as usize + size as usize]; 
                for i in 0..num {
                    let dyntab = Elf64Dyn::from(&bytes[i as usize * ELF64_DYN_SIZE as usize..(i as usize + 1) * ELF64_DYN_SIZE as usize]); 
                    self.dynamic_tables.push(dyntab);
                }
                break;
            }  
        }
        
        let mut str_offset = 0;
        for dyntab in &self.dynamic_tables {
            if dyntab.d_tag == DT_STRTAB {
                str_offset = dyntab.d_val_ptr as u64;
                break;
            }
        }

        let mut sym_offset = 0;
        for dyntab in &self.dynamic_tables {
            if dyntab.d_tag == DT_SYMTAB {
                sym_offset = dyntab.d_val_ptr as u64;
                break;
            }
        }

        if sym_offset != 0 && str_offset != 0 {
            let mut i = 0;
            let sym = Elf64Sym::from(&self.elf_bytes[sym_offset as usize + i * ELF64_SYM_SIZE as usize..sym_offset as usize + (i + 1) * ELF64_SYM_SIZE as usize]);
        }
    }

    pub fn analyse_dynamic_segment(&mut self) {
        for dyn_table in &self.dynamic_tables {
            let val_ptr = dyn_table.d_val_ptr as u64;
            match dyn_table.d_tag {
                DT_STRTAB => {
                    self.strtab_offset = val_ptr;
                }
                DT_SYMTAB => {
                    self.symtab_offset = val_ptr; 
                }

                _ => {}
            } 
        } 
    }

    pub fn analyse_section_tables(&mut self) {
        let section_num = self.elf_header.e_shnum as u64;
        let section_size = self.elf_header.e_shentsize as u64;
        let section_offset = self.elf_header.e_shoff as u64;
        let section_bytes = &self.elf_bytes[section_offset as usize..section_offset as usize + section_num as usize * section_size as usize];
        //log(&format!("{}\n{:?}", self.elf_bytes.len(), self.elf_bytes));

        //log(&format!("section_num: {}\n section_size: {} \n section_offset: {} \n section_bytes: {:?}\n", section_num, section_size, section_offset, section_bytes));
        //log(&format!("{}..{}", section_offset, section_offset + section_num * section_size));

        for i in 0..section_num as usize {
            let mut section_table = Elf64Shdr::new();
            section_table.fill(&section_bytes[i * section_size as usize ..i * section_size as usize + section_size as usize]);
            self.elf_sections.push(section_table);
        }
        //println!("{:#?}", self.elf_sections);
        let strndx = self.elf_header.e_shstrndx as usize;
        let shstrtab = &self.elf_sections[strndx];
        let shstr_offset = shstrtab.sh_offset as usize;
        let shstr_size = shstrtab.sh_size as usize;
        let shstr_bytes = &self.elf_bytes[shstr_offset..shstr_offset + shstr_size];
        for section in &self.elf_sections {
            let mut name = String::new();
            let mut c = shstr_bytes[section.sh_name as usize];
            let mut i = section.sh_name as usize;
            while c != 0 {
                name.push(c as char);
                i += 1;
                c = shstr_bytes[i];
            }
            self.section_name_map.insert(section.sh_name as u64, name);
        }

        for section in &self.elf_sections {
            let offset = section.sh_offset as u64;
            let name = self.section_name_map[&(section.sh_name as u64)].clone();
            let size = section.sh_size as u64;
            let vaddr = section.sh_addr as u64;

            
            if section.sh_flags & SHF_EXECINSTR as u64 != 0 {
                let vaddr = section.sh_addr as u64;
                let mut section_record = SectionRecord::from(offset, section.sh_addr as u64, name, size, SectionType::Code);
                section_record.vaddr = section.sh_addr as u64;
                self.sections.push(section_record);
            }
     
        }

    }

    pub fn analyse_dynsym_from_section(&mut self) {
        self.dynsym_tables.clear();
        self.dynsym_name_map.clear();
        for section in &self.elf_sections {
            if self.section_name_map[&(section.sh_name as u64)] == ".dynsym" {
                let offset = section.sh_offset as u64;
                let size = section.sh_size as u64;
                let num = size / ELF64_SYM_SIZE;
                let bytes = &self.elf_bytes[offset as usize..offset as usize + size as usize ];
                for i in 0..num as usize {
                    let dynsym = Elf64Sym::from(&bytes[i as usize * ELF64_SYM_SIZE as usize ..(i + 1) * ELF64_SYM_SIZE as usize]);
                    //println!("{:x}", dynsym.st_name);
                    self.dynsym_tables.push(dynsym);
                }
            }
        }    


        for section in &self.elf_sections {
            if self.section_name_map[&(section.sh_name as u64)] == ".dynstr" {
                let bytes = &self.elf_bytes[section.sh_offset as usize..section.sh_offset as usize + section.sh_size as usize];
                for dyntab in &self.dynsym_tables {
                    let mut name = String::new();
                    let index = dyntab.st_name as usize;
                    let mut i = index;
                    let mut c = bytes[i];
                    while c != 0 {
                        name.push(c as char);
                        i += 1;
                        c = bytes[i];
                    }
                    self.dynsym_name_map.insert(index as u64, name);
                }
                break;
            }
        }

    }

    pub fn analyse_symbol_from_section(&mut self) {
        self.symbol_tables.clear();
        self.symbol_name_map.clear();
        for section in &self.elf_sections {
            if self.section_name_map[&(section.sh_name as u64)] == ".symtab" {
                let offset = section.sh_offset as u64;
                let size = section.sh_size as u64;
                let num = size / ELF64_SYM_SIZE;
                let bytes = &self.elf_bytes[offset as usize..offset as usize + size as usize];
                for i in 0..num { 
                    let sym = Elf64Sym::from(&bytes[i as usize* ELF64_SYM_SIZE as usize ..(i as usize + 1) * ELF64_SYM_SIZE as usize]);
                    let _type = sym.st_info & 0xf;
                    if _type != STT_SECTION {
                        self.symbol_tables.push(sym);
                    }
                }
            }
        }    

        for section in &self.elf_sections {
            if self.section_name_map[&(section.sh_name as u64)] == ".strtab" {
                let bytes = &self.elf_bytes[section.sh_offset as usize..section.sh_offset as usize + section.sh_size as usize];
                for symtab in &self.symbol_tables {
                    let mut name = String::new();
                    let index = symtab.st_name as usize;
                    let mut i = index;
                    let mut c = bytes[i];
                    while c != 0 {
                        name.push(c as char);
                        i += 1;
                        c = bytes[i];
                    }
                    self.symbol_name_map.insert(index as u64, name);
                }
                break;
            }
        }

        //println!("{:#?}", self.symbol_name_map);
        //println!("{}", self.symbol_name_map.len());

    }

    pub fn analyse_plt(&mut self) {
    } 


    pub fn get_excutable_segment_as_bytes(&mut self) -> &[u8] {
        for program in &self.elf_programs {
            if (program.p_flags & PF_X) != 0 {
                let start = program.p_offset as usize;
                let end = program.p_filesz as usize + start;
                return &self.elf_bytes[start..end];
            }
        }
        
        &[0u8; 0]
    }

    pub fn analyse_symbols(&mut self) {
        //println!("{:#?}", self.symbol_tables);
        //println!("{:#?}", self.symbol_name_map);
        for sym in &self.symbol_tables {

            let name = self.symbol_name_map[&(sym.st_name as u64 )].clone();
            let bind = sym.st_info >> 4;
            let _type = sym.st_info & 0xf;

            if _type == 2 {
                self.symbols.push(SymbolRecord::from(sym.st_value as u64, name, sym.st_size as u64, SymbolType::Func));
            } else if _type == 1 && bind == 1 {
                self.symbols.push(SymbolRecord::from(sym.st_value as u64, name, sym.st_size as u64, SymbolType::Val));
            }

        }

        //println!("{}", self.symbols.len());

        /*
        for sym in &self.dynsym_tables {
            let name = self.dynsym_name_map[&(sym.st_name as u64)].clone();
            let bind = sym.st_info >> 4;
            let _type = sym.st_info & 0xf;

            if _type == 2 {
                self.symbols.push(SymbolRecord::from(sym.st_value as u64, name, sym.st_size as u64, SymbolType::Func));
            } else if _type == 1 && bind == 1 {
                self.symbols.push(SymbolRecord::from(sym.st_value as u64, name, sym.st_size as u64, SymbolType::Val));
            }
       
        }
        */
        
        //println!("{}", self.relas_from_section.len());
        for rela in &self.relas_from_section {
            let offset = rela.r_offset as u64;
            let sym = (rela.r_info >> 32) as u64;
            let _type = (rela.r_info & 0xffffffff) as u64;
            let addend = rela.r_addend as i64;
            match _type {
                R_LARCH_64 => {
                    let symbol = self.dynsym_tables[sym as usize].clone();
                    let name = self.dynsym_name_map[&(symbol.st_name as u64)].clone();
                    let bind = symbol.st_info >> 4;
                    let _type = symbol.st_info & 0xf;
                    let size = symbol.st_size as u64;
                    let value = symbol.st_value as u64;
                    if _type == 2 {
                        self.dyn_symbols.push(DynSymbolRecord::from(value, name, size, SymbolType::Func, RelactionType::R_LARCH_64, value));
                    } else if bind == 1 && _type == 1 {
                        self.dyn_symbols.push(DynSymbolRecord::from(value, name, size, SymbolType::Val, RelactionType::R_LARCH_64, value));
                    }
                }
                R_LARCH_JUMP_SLOT => {
                    let symbol = self.dynsym_tables[sym as usize].clone();
                    let name = self.dynsym_name_map[&(symbol.st_name as u64)].clone();
                    let bind = symbol.st_info >> 4;
                    let _type = symbol.st_info & 0xf;
                    let size = symbol.st_size as u64;
                    let value = symbol.st_value as u64;
                    if _type == 2 {
                        self.dyn_symbols.push(DynSymbolRecord::from(offset, name, size, SymbolType::Func, RelactionType::R_LARCH_JUMP_SLOT, value));
                    } else if bind == 1 && _type == 1 {
                        self.dyn_symbols.push(DynSymbolRecord::from(offset, name, size, SymbolType::Val, RelactionType::R_LARCH_JUMP_SLOT, value));
                    }

                }
                _ => {}
            }

            //println!("{:#?}", self.symbols);
        }
        //println!("{}", self.relas_from_section.len());
        //println!("{}", self.symbol_tables.len());
        //println!("{}", self.symbols.len());
        //println!("{:#?}", self.symbols);
    }

    pub fn analyse_rel_from_section(&mut self) {
        for section in &self.elf_sections {
            if section.sh_type == SHT_RELA {
                let offset = section.sh_offset as usize;
                let size = section.sh_size as usize;
                let num = size / ELF64_RELA_SIZE as usize;
                let bytes = &self.elf_bytes[offset..offset + size];
                for i in 0..num {
                    let start = i * ELF64_RELA_SIZE as usize;
                    let end = start + ELF64_RELA_SIZE as usize;
                    let rela = Elf64Rela::from(&bytes[start..end]);
                    self.relas_from_section.push(rela);
                }
            }
        }

       //println!("{:#?}", self.relas_from_section);
    }

    pub fn analyse_data_from_program(&mut self) {
        let mut data = HashMap::<u64, Vec<u8>>::new();
        for program in &self.elf_programs{
            if program.p_type == PT_LOAD {
                let address = program.p_vaddr as u64;
                let offset = program.p_offset as u64;
                let size = program.p_memsz as u64;
                let d = self.elf_bytes[offset as usize..offset as usize + size as usize].to_vec(); 
                data.insert(address, d);
            }
        }

        self.data = data;

    }

    pub fn get_data(&self) -> &HashMap<u64, Vec<u8>> {
        &self.data
    }

    pub fn get_excutable_sections(&mut self) {
        for record in &mut self.sections {
            if record.section_type == SectionType::Code {
                let bytes = &self.elf_bytes[record.offset as usize..record.offset as usize + record.size as usize];
                record.bytes = bytes.to_vec();
            }
        }
        /*
        let mut codes = Vec::<Vec<u8>>::new();
        for section in &self.elf_sections {
            if (section.sh_flags as u64 & SHF_EXECINSTR as u64) != 0 {
                let start = section.sh_offset as u64;
                let end = start + section.sh_size as u64;
                let code = &self.elf_bytes[start..end].to_vec();
                codes.push(code.clone());
            } 
        } 
        codes
        */
    } 


    /*
    pub fn analyse(&mut self) -> Result<LoongArchOk , LoongArchError> {
        match self.elf_header.fill(self.elf_bytes) {
            Ok(info) => self.elf_info = info,
            Err(e) => return Err(e),
        };

        let mut section_offset = self.elf_header.e_shoff as u64;
        let section_size = self.elf_header.e_shentsize as u64;
        let section_num = self.elf_header.e_shnum as u64;

        if section_offset == 0 || section_offset >= self.elf_bytes.len() {
            handle_loongarch_warning(LoongArchWarning::NOTFOUNDSECTIONS);
        } else {
            let mut num = 0;
            loop {
                if section_offset + section_size > self.elf_bytes.len() {
                    break;
                } 
                let mut section = Elf64Shdr::new();
                section.fill(&self.elf_bytes[section_offset..section_offset + section_size]);
                if num > 0 && section.sh_type == SHT_NULL {
                    break;
                }
                self.elf_sections.push(section);
                num += 1;
                section_offset += section_size;
            }
            
            if self.elf_header.e_shnum as u64 != self.elf_sections.len() {
                handle_loongarch_warning(LoongArchWarning::SECTIONNUMBERABNORMAL(self.elf_header.e_shnum as u64, self.elf_sections.len()));
            }

            let shstr_index = self.elf_header.e_shstrndx as u64;
            let section = &self.elf_sections[shstr_index];
            let size = section.sh_size as u64;
            let offset = section.sh_offset as u64;
            let shstr = &self.elf_bytes[offset..offset + size];
            
            for section in &self.elf_sections {
                let name = section.sh_name as u64;
                let mut i = name;
                let mut name_str = String::new();
                loop {
                    let c = shstr[i];
                    if c == 0 {
                        break;
                    } else {
                        name_str.push(c as char);
                        i += 1;
                    }
                }
                self.section_name_map.insert(name, name_str);
            }

            let mut symbol_index = None;
            let mut dynsym_index = None;
            for i in 0..self.elf_header.e_shnum as u64{
                let section = &self.elf_sections[i];
                let name = section.sh_name as u64;
                let size = section.sh_size as u64;
                let offset = section.sh_offset as u64;
                let vaddr = section.sh_addr as u64;

                match section.sh_type {
                    SHT_STRTAB => {
                        let bytes = &self.elf_bytes[offset..offset + size].to_vec();
                        self.str_bytes.insert(i, bytes.clone());
                        if &self.section_name_map[&name] == ".strtab" {
                            symbol_index = Some(i);
                        } else if &self.section_name_map[&name] == ".dynstr" {
                            dynsym_index = Some(i);
                        }
                    }

                    SHT_SYMTAB => {
                        let bytes = &self.elf_bytes[offset..offset + size].to_vec();
                        let num = size / ELF64_SYM_SIZE;
                        for i in 0..num {
                            let start = i * ELF64_SYM_SIZE;
                            let end = start + ELF64_SYM_SIZE;
                            let sym_bytes = &bytes[start..end];
                            self.symbol_tables.push(Elf64Sym::from(sym_bytes));
                            //println!("{}", self.symbol_tables[i].st_shndx)
                        }

                    }

                    SHT_DYNSYM => {
                        let bytes = &self.elf_bytes[offset..offset + size].to_vec();
                        let num = size / ELF64_SYM_SIZE;
                        for i in 0..num {
                            let start = i * ELF64_SYM_SIZE;
                            let end = start + ELF64_SYM_SIZE;
                            let sym_bytes = &bytes[start..end];
                            self.dynsym_tables.push(Elf64Sym::from(sym_bytes));
                        }
                    }

                    _ => {}
                }
            }
            
            if let Some(index) = symbol_index {
                let sym_bytes = &self.str_bytes[&index];
                for sym in &self.symbol_tables {
                    let name_index = sym.st_name as u64;
                    let vaddr = sym.st_value as u64;
                    let mut name = String::new();
                    let mut i = name_index;
                    while sym_bytes[i] != 0 {
                        name.push(sym_bytes[i] as char);
                        i += 1;
                    }
                    self.symbol_name_map.insert(vaddr, name);
                }
            }

            /*
            if let Some(index) = dynsym_index {
                let sym_bytes = &self.str_bytes[&index];
                for i in 0..sym_bytes.len() {
                    let c = sym_bytes[i];
                    if c == 0 {
                        print!("\n{}: ", i);
                    } else {
                        print!("{}", c as char);
                    }
                }
                for sym in &self.dynsym_tables {
                    let name_index = sym.st_name as u64;
                    let vaddr = sym.st_value as u64;
                    let mut name = String::new();
                    let mut i = name_index;
                    println!("{:<08x} {:<08x}", name_index, vaddr);
                    /*
                    while sym_bytes[i] != 0 {
                        name.push(sym_bytes[i] as char);
                        i += 1;
                    }
                    self.dynsym_name_map.insert(vaddr, name);
                    */
                }
            }
            */

        }        
        Ok(LoongArchOk::Ok)
    }
    */

    pub fn get_section_record(&self) -> &Vec<SectionRecord> {
        &self.sections
    }

    pub fn get_symbol_record(&self) -> &Vec<SymbolRecord> {
        &self.symbols
    }

    pub fn get_dyn_symbol_record(&self) -> &Vec<DynSymbolRecord> {
        &self.dyn_symbols
    }
}

fn get_section_index_from_name(name: &str, map: &HashMap<u64, String>, sections: &Vec<Elf64Shdr>) -> usize {
    let mut index = usize::MAX;
    for m in map {
        if m.1 == name {
            index = *m.0 as usize;
            break;
        }
    }
    for i in 0..sections.len() {
        if sections[i].sh_name as usize == index {
            return i;
        }
    }
    return usize::MAX;
}

impl<'a> From<&'a Vec<u8>> for Elf64<'a> {
    fn from(bytes: &'a Vec<u8>) -> Self {
        Elf64 {
            elf_bytes: bytes,
            elf_info: ElfInfo{endian: Endianess::INVALID,},
            elf_header: Elf64Ehdr::new(),
            elf_sections: Vec::new(),
            elf_programs: Vec::new(),
            dynamic_tables: Vec::new(),
            strtab_offset: 0,
            symtab_offset: 0,
            str_bytes: HashMap::new(),
            section_name_map: HashMap::new(),
            symbol_tables: Vec::new(),
            symbol_name_map: HashMap::new(),
            dynsym_tables: Vec::new(),
            dynsym_name_map: HashMap::new(),

            symbol_tables_dynamic: Vec::new(),
            symbol_name_map_dynamic: HashMap::new(),

            symbols: Vec::new(),
            dyn_symbols: Vec::new(),
            sections: Vec::new(),

            rels_from_section: Vec::new(),
            rels_from_segment: Vec::new(),

            relas_from_section: Vec::new(),
            relas_from_segment: Vec::new(),

            data: HashMap::new(),
        } 
    }
}


impl SymbolRecord {
    pub fn from(offset: u64, name: String, size: u64, sym_type: SymbolType) -> SymbolRecord {
        SymbolRecord {
            offset,
            name,
            size,
            sym_type,
        }
    }
}

impl DynSymbolRecord {
    pub fn from(offset: u64, name: String, size: u64, sym_type: SymbolType, reloc_type: RelactionType, value: u64) -> DynSymbolRecord {
        DynSymbolRecord {
            offset,
            name,
            size,
            sym_type,
            reloc_type,
            value,
        }
    }
}

impl SectionRecord {
    pub fn from(offset: u64, vaddr: u64, name: String, size: u64, section_type: SectionType) -> SectionRecord {
        SectionRecord {
            offset,
            vaddr,
            name,
            size,
            section_type,
            bytes: Vec::new(),
        }
    }
}

const BBS: &'static str = ".bbs";  
const COMMENT: &'static str = ".comment";
const DATA: &'static str = ".data";
const DATA1: &'static str = ".data1";
const DEBUG: &'static str = "debug";
const DYNAMIC: &'static str = ".dynamic";
const DYNSTR: &'static str = ".dynstr";
const DYNSYM: &'static str = ".dynsym";
const FINI: &'static str = ".fini";
const GOT: &'static str = ".got";
const HASH: &'static str = ".hash";
const INIT: &'static str = ".init";
const INTERP: &'static str = "interp";
const LINE: &'static str = ".line";
const NOTE: &'static str = ".note";
const PLT: &'static str = ".plt";
const RELNAME: &'static str = ".rel"; 
const RODATA: &'static str = ".rodata";
const RODATA1: &'static str = ".rodata1";
const SHSTRTAB: &'static str = ".shstrtab";
const STRTAB: &'static str = ".strtab";
const SYMTAB: &'static str = ".symtab";
const TEXT: &'static str = ".text";

