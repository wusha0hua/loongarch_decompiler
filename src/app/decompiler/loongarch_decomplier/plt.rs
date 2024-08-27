//use crate::{loongarch_decomplier::*, NameValue};
use crate::app::elf::DynSymbolRecord;
use crate::app::decompiler::{*, NameValue};
use std::collections::HashMap;

pub fn analyse_plt(insns: &Vec<AssemblyInstruction>, functions: &mut HashMap<u64, NameValue>, symbols: &Vec<DynSymbolRecord>) {
    let mut symbol_map = HashMap::<u64, String>::new(); 
    for symbol in symbols {
        symbol_map.insert(symbol.offset, symbol.name.clone());
    }

    let mut i = 8;
    while i < insns.len() {
        let address = insns[i].address;
        let mut n1 = 0;
        let mut n2 = 0;
        if let Some(operand) = &insns[i].operand2 {
            n1 = operand.value as i64;
        } else {
            panic!("error");
        }

        if let Some(operand) = &insns[i + 1].operand3 {
            n2 = operand.value as i64;
        } else {
            panic!("error");
        }

        let addr = (address as i64 + (n1 << 12) + n2) as u64;
        if let Some(name) = symbol_map.get(&addr) {
            functions.insert(address, NameValue::Name(name.clone()));
        }
        i += 4;
    }
}
