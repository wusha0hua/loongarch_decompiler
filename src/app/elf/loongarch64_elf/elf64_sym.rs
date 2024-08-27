use super::super::data_convert::*;

#[derive(Debug, Clone)]
pub struct Elf64Sym {
	pub st_name: u32,		/* Symbol name (string tbl index) */
	pub st_info: u8,		/* Symbol u8 and binding */
	pub st_other: u8,		/* Symbol visibility */
	pub st_shndx: u16,		/* Section index */
	pub st_value: u64,		/* Symbol value */
	pub st_size: u64,		/* Symbol size */
}

pub const ELF64_SYM_SIZE: u64 = 24;

#[derive(Debug, Clone)]
pub struct Elf64Syminfo {
	si_boundto: u16,		/* Direct bindings, symbol bound to */
	si_flags: u16,			/* Per symbol flags */
}

impl Elf64Sym {
    pub fn new() -> Self {
        Elf64Sym {
            st_name: 0,
            st_info: 0,
            st_other: 0,
            st_shndx: 0,
            st_value: 0,
            st_size: 0,
        }
    }

    pub fn from(bytes: &[u8]) -> Self {
        Elf64Sym {
            st_name: u32::from_le_bytes(clone_into_array(&bytes[0..4])),
            st_info: u8::from_le_bytes(clone_into_array(&bytes[4..5])),
            st_other: u8::from_le_bytes(clone_into_array(&bytes[5..6])),
            st_shndx: u16::from_le_bytes(clone_into_array(&bytes[6..8])),
            st_value: u64::from_le_bytes(clone_into_array(&bytes[8..16])),
            st_size: u64::from_le_bytes(clone_into_array(&bytes[16..24])),
        }
    }
}

pub const STT_NOTYPE: u8 = 0;
pub const STT_OBJECT: u8 = 1;
pub const STT_FUNC: u8 = 2;
pub const STT_SECTION: u8 = 3;
pub const STT_FILE: u8 = 4;
pub const STT_COMMON: u8 = 5;
pub const STT_TLS: u8 = 6;
pub const STT_NUM: u8 = 7;
pub const STT_LOOS: u8 = 10;
pub const STT_GNU_IFUNC: u8 = 10;
pub const STT_HIOS: u8 = 12;
pub const STT_LOPROC: u8 = 13;
pub const STT_HIPROC: u8 = 15;
