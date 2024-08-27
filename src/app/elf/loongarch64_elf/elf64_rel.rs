use serde_derive::Serialize;

use super::super::data_convert::*;
#[derive(Debug, Clone)]
pub struct Elf64Rel {
    pub r_offset: u64,
    pub r_info: u64,
}
pub const ELF64_REL_SIZE: u64 = 16;

#[derive(Debug, Clone)]
pub struct Elf64Rela {
    pub r_offset: u64,
    pub r_info: u64,
    pub r_addend: i64,
}
pub const ELF64_RELA_SIZE: u64 = 24;

impl Elf64Rel {
    pub fn from(bytes: &[u8]) -> Self {
        Elf64Rel {
            r_offset: u64::from_le_bytes(clone_into_array(&bytes[0..8])),
            r_info: u64::from_le_bytes(clone_into_array(&bytes[8..16])),
        }
    }
}

impl Elf64Rela {
    pub fn from(bytes: &[u8]) -> Self {
        Elf64Rela {
            r_offset: u64::from_le_bytes(clone_into_array(&bytes[0..8])),
            r_info: u64::from_le_bytes(clone_into_array(&bytes[8..16])),
            r_addend: i64::from_le_bytes(clone_into_array(&bytes[16..24])),
        }
    }
}

pub const R_LARCH_NONE: u64 = 0;
pub const R_LARCH_64: u64 = 2;
pub const R_LARCH_RELATIVE: u64 = 3;
pub const R_LARCH_JUMP_SLOT: u64 = 5;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RelactionType {
    R_LARCH_NONE,
    R_LARCH_64,
    R_LARCH_RELATIVE,
    R_LARCH_JUMP_SLOT,
}
