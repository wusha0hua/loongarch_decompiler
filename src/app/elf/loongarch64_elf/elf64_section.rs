use super::super::data_convert::*;
use super::super::loongarch_result::*;

#[derive(Debug, Clone)]
pub struct Elf64Shdr{
	pub sh_name: u32,		/* Section name (string tbl index) */
	pub sh_type: u32,		/* Section type */
	pub sh_flags: u64,		/* Section flags */
	pub sh_addr: u64,		/* Section virtual addr at execution */
	pub sh_offset: u64,		/* Section file offset */
	pub sh_size: u64,		/* Section size in bytes */
	pub sh_link: u32,		/* Link to another section */
	pub sh_info: u32,		/* Additional section information */
	pub sh_addralign: u64,		/* Section alignment */
	pub sh_entsize: u64,		/* Entry size if section holds table */
}

const ELF64_SHDR_SIZE: u64 = 64;

impl Elf64Shdr {
    pub fn new() -> Self {
        Elf64Shdr {
            sh_name: 0,
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0,
        }
    }

    pub fn fill(&mut self, section_bytes: &[u8]) {
        self.sh_name = u32::from_le_bytes(clone_into_array(&section_bytes[0..4])); 
        self.sh_type = u32::from_le_bytes(clone_into_array(&section_bytes[4..8])); 
        self.sh_flags = u64::from_le_bytes(clone_into_array(&section_bytes[8..16])); 
        self.sh_addr = u64::from_le_bytes(clone_into_array(&section_bytes[16..24])); 
        self.sh_offset = u64::from_le_bytes(clone_into_array(&section_bytes[24..32])); 
        self.sh_size = u64::from_le_bytes(clone_into_array(&section_bytes[32..40])); 
        self.sh_link = u32::from_le_bytes(clone_into_array(&section_bytes[40..44])); 
        self.sh_info = u32::from_le_bytes(clone_into_array(&section_bytes[44..48])); 
        self.sh_addralign = u64::from_le_bytes(clone_into_array(&section_bytes[48..56])); 
        self.sh_entsize = u64::from_le_bytes(clone_into_array(&section_bytes[56..64]));
    }
}


 pub const SHN_UNDEF: u32 = 0;		/* Undefined section */
 pub const SHN_LORESERVE: u32 = 0x0;		/* Start of reserved indices */
 pub const SHN_LOPROC: u32 = 0x0;		/* Start of processor-specific */
 pub const SHN_BEFORE: u32 = 0x0;		/* Order section before all others
					   (Solaris).  */
 pub const SHN_AFTER: u32 = 0x1;		/* Order section after all others
					   (Solaris).  */
 pub const SHN_HIPROC: u32 = 0xf;		/* End of processor-specific */
 pub const SHN_LOOS: u32 = 0x0;		/* Start of OS-specific */
 pub const SHN_HIOS: u32 = 0xf;		/* End of OS-specific */
 pub const SHN_ABS: u32 = 0x1;		/* Associated symbol is absolute */
 pub const SHN_COMMON: u32 = 0x2;		/* Associated symbol is common */
 pub const SHN_XINDEX: u32 = 0xf;		/* Index is in extra table.  */
 pub const SHN_HIRESERVE: u32 = 0xf;		/* End of reserved indices */

/* Legal values for sh_u8 (section u8).  */

 pub const SHT_NULL: u32 = 0;		/* Section header table entry unused */
 pub const SHT_PROGBITS: u32 = 1;		/* Program data */
 pub const SHT_SYMTAB: u32 = 2;		/* Symbol table */
 pub const SHT_STRTAB: u32 = 3;		/* String table */
 pub const SHT_RELA: u32 = 4;		/* Relocation entries with addends */
 pub const SHT_HASH: u32 = 5;		/* Symbol hash table */
 pub const SHT_DYNAMIC: u32 = 6;		/* Dynamic linking information */
 pub const SHT_NOTE: u32 = 7;		/* Notes */
 pub const SHT_NOBITS: u32 = 8;		/* Program space with no data (bss) */
 pub const SHT_REL: u32 = 9;		/* Relocation entries, no addends */
 pub const SHT_SHLIB: u32 = 10;		/* Reserved */
 pub const SHT_DYNSYM: u32 = 11;		/* Dynamic linker symbol table */
 pub const SHT_INIT_ARRAY: u32 = 14;		/* Array of constructors */
 pub const SHT_FINI_ARRAY: u32 = 15;		/* Array of destructors */
 pub const SHT_PREINIT_ARRAY: u32 = 16;		/* Array of pre-constructors */
 pub const SHT_GROUP: u32 = 17;		/* Section group */
 pub const SHT_SYMTAB_SHNDX: u32 = 18;		/* Extended section indices */
 pub const SHT_NUM: u32 = 19;		/* Number of defined u8s.  */
 pub const SHT_LOOS: u32 = 0x0;	/* Start OS-specific.  */
 pub const SHT_GNU_ATTRIBUTES: u32 = 0x5;	/* Object attributes.  */
 pub const SHT_GNU_HASH: u32 = 0x6;	/* GNU-style hash table.  */
 pub const SHT_GNU_LIBLIST: u32 = 0x7;	/* Prelink library list */
 pub const SHT_CHECKSUM: u32 = 0x8;	/* Checksum for DSO content.  */
 pub const SHT_LOSUNW: u32 = 0xa;	/* Sun-specific low bound.  */
 pub const SHT_SUNW_MOVE: u32 = 0xa;
 pub const SHT_SUNW_COMDAT: u32 = 0xb;
 pub const SHT_SUNW_SYMINFO: u32 = 0xc;
 pub const SHT_GNU_VERDEF: u32 = 0xd;	/* Version definition section.  */
 pub const SHT_GNU_VERNEED: u32 = 0xe;	/* Version needs section.  */
 pub const SHT_GNU_VERSYM: u32 = 0xf;	/* Version symbol table.  */
 pub const SHT_HISUNW: u32 = 0xf;	/* Sun-specific high bound.  */
 pub const SHT_HIOS: u32 = 0xf;	/* End OS-specific u8 */
 pub const SHT_LOPROC: u32 = 0x0;	/* Start of processor-specific */
 pub const SHT_HIPROC: u32 = 0xf;	/* End of processor-specific */
 pub const SHT_LOUSER: u32 = 0x0;	/* Start of application-specific */
 pub const SHT_HIUSER: u32 = 0xf;	/* End of application-specific */

/* Legal values for sh_flags (section flags).  */

 pub const SHF_WRITE: u32 = 1 << 0;	/* Writable */
 pub const SHF_ALLOC: u32 = 1 << 1;	/* Occupies memory during execution */
 pub const SHF_EXECINSTR: u32 = 1 << 2;	/* Executable */
 pub const SHF_MERGE: u32 = 1 << 4;	/* Might be merged */
 pub const SHF_STRINGS: u32 = 1 << 5;	/* Contains nul-terminated strings */
 pub const SHF_INFO_LINK: u32 = 1 << 6;	/* `sh_info' contains SHT index */
 pub const SHF_LINK_ORDER: u32 = 1 << 7;	/* Preserve order after combining */
 pub const SHF_OS_NONCONFORMING: u32 = 1 << 8;	/* Non-standard OS specific handling
					   required */
 pub const SHF_GROUP: u32 = 1 << 9;	/* Section is member of a group.  */
 pub const SHF_TLS: u32 = 1 << 10;	/* Section hold thread-local data.  */
 pub const SHF_COMPRESSED: u32 = 1 << 11;	/* Section with compressed data. */
 pub const SHF_MASKOS: u32 = 0x0;	/* OS-specific.  */
 pub const SHF_MASKPROC: u32 = 0x0;	/* Processor-specific */
 pub const SHF_GNU_RETAIN: u32 = 1 << 21;  /* Not to be GCed by linker.  */
 pub const SHF_ORDERED: u32 = 1 << 30;	/* Special ordering requirement
					   Solaris.  */
 pub const SHF_EXCLUDE: u32 = 1u32 << 31;	/* Section is excluded unless
					   referenced or allocated Solaris.*/
