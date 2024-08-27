use super::super::data_convert::*;

#[derive(Debug, Clone)]
pub struct Elf64Phdr{
	pub p_type: u32,			/* Segment type */
	pub p_flags: u32,		/* Segment flags */
	pub p_offset: u64,		/* Segment file offset */
	pub p_vaddr: u64,		/* Segment virtual address */
	pub p_paddr: u64,		/* Segment physical address */
	pub p_filesz: u64,		/* Segment size in file */
	pub p_memsz: u64,		/* Segment size in memory */
	pub p_align: u64,		/* Segment alignment */
}

pub const ELF64_PHDR_SIZE: u64 = 56;

impl Elf64Phdr {
    pub fn from(bytes: &[u8]) -> Self {
        Elf64Phdr {
            p_type: u32::from_le_bytes(clone_into_array(&bytes[0..4])),
            p_flags: u32::from_le_bytes(clone_into_array(&bytes[4..8])),
            p_offset: u64::from_le_bytes(clone_into_array(&bytes[8..16])),
            p_vaddr: u64::from_le_bytes(clone_into_array(&bytes[16..24])),
            p_paddr: u64::from_le_bytes(clone_into_array(&bytes[24..32])),
            p_filesz: u64::from_le_bytes(clone_into_array(&bytes[32..40])),
            p_memsz: u64::from_le_bytes(clone_into_array(&bytes[40..48])),
            p_align: u64::from_le_bytes(clone_into_array(&bytes[48..56])),
        }    
    }
}


pub const PT_NULL: u32 = 0;		/* Program header table entry unused */
pub const PT_LOAD: u32 = 1;		/* Loadable program segment */
pub const PT_DYNAMIC: u32 = 2;		/* Dynamic linking information */
pub const PT_INTERP: u32 = 3;		/* Program interpreter */
pub const PT_NOTE: u32 = 4;		/* Auxiliary information */
pub const PT_SHLIB: u32 = 5;		/* Reserved */
pub const PT_PHDR: u32 = 6;		/* Entry for header table itself */
pub const PT_TLS: u32 = 7;		/* Thread-local storage segment */
pub const PT_NUM: u32 = 8;		/* Number of defined u8s */
pub const PT_LOOS: u32 = 0x0;	/* Start of OS-specific */
pub const PT_GNU_EH_FRAME: u32 = 0x0;	/* GCC .eh_frame_hdr segment */
pub const PT_GNU_STACK: u32 = 0x1;	/* Indicates stack executability */
pub const PT_GNU_RELRO: u32 = 0x2;	/* Read-only after relocation */
pub const PT_GNU_PROPERTY: u32 = 0x3;	/* GNU property */
pub const PT_LOSUNW: u32 = 0xa;
pub const PT_SUNWBSS: u32 = 0xa;	/* Sun Specific segment */
pub const PT_SUNWSTACK: u32 = 0xb;	/* Stack segment */
pub const PT_HISUNW: u32 = 0xf;
pub const PT_HIOS: u32 = 0xf;	/* End of OS-specific */
pub const PT_LOPROC: u32 = 0x0;	/* Start of processor-specific */
pub const PT_HIPROC: u32 = 0xf;	/* End of processor-specific */

/* Legal values for p_flags (segment flags).  */

pub const PF_X: u32 = (1 << 0);	/* Segment is executable */
pub const PF_W: u32 = (1 << 1);	/* Segment is writable */
pub const PF_R: u32 = (1 << 2);	/* Segment is readable */
pub const PF_MASKOS: u32 = 0x0;	/* OS-specific */
pub const PF_MASKPROC: u32 = 0x0;	/* Processor-specific */
