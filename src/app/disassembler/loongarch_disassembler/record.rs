pub use serde_json;
pub use serde::{Serialize, Deserialize};
//pub use serde_derive::Serialize;

pub use crate::app::elf::{SymbolRecord, SymbolType};

/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolRecord {
    pub offset: usize,
    pub name: String,
    pub size: usize,
    pub sym_type: SymbolType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymbolType {
    Func,
    Val,
    Label,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionRecord {
    pub offset: usize,
    pub vaddr: usize,
    pub name: String,
    pub size: usize,
    pub section_type: SectionType,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SectionType {
    Code,
    Data,
    None,
}
*/

impl SymbolRecord {
    pub fn label_from_addr(addr: u64) -> Self {
        SymbolRecord {
            offset: addr,
            name: String::new(),
            size: 0,
            sym_type: SymbolType::Label,
        }
    }

    pub fn from_plt(addr: u64, name: String) -> Self {
        SymbolRecord {
            offset: addr,
            name,
            size: 0,
            sym_type: SymbolType::Func,
        }
    }
}
