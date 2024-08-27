use super::super::data_convert::*;

#[derive(Debug, Clone)]
pub struct Elf64Dyn {
    pub d_tag: i64,			/* Dynamic entry type */
    pub d_val_ptr: u64,
}

pub const ELF64_DYN_SIZE: u64 = 16;

impl Elf64Dyn {
    pub fn new() -> Self {
        Elf64Dyn {
            d_tag: 0,
            d_val_ptr: 0,
        }
    } 

    pub fn from(bytes: &[u8]) -> Self {
        Elf64Dyn {
            d_tag: i64::from_le_bytes(clone_into_array(&bytes[0..8])),
            d_val_ptr: u64::from_le_bytes(clone_into_array(&bytes[8..16])),
        }
    }
} 

/* Legal values for d_tag (dynamic entry type).  */

pub const DT_NULL: i64 = 0;		/* Marks end of dynamic section */
pub const DT_NEEDED: i64 = 1;		/* Name of needed library */
pub const DT_PLTRELSZ: i64 = 2;		/* Size in bytes of PLT relocs */
pub const DT_PLTGOT: i64 = 3;		/* Processor defined value */
pub const DT_HASH: i64 = 4;		/* Address of symbol hash table */
pub const DT_STRTAB: i64 = 5;		/* Address of string table */
pub const DT_SYMTAB: i64 = 6;		/* Address of symbol table */
pub const DT_RELA: i64 = 7;		/* Address of Rela relocs */
pub const DT_RELASZ: i64 = 8;		/* Total size of Rela relocs */
pub const DT_RELAENT: i64 = 9;		/* Size of one Rela reloc */
pub const DT_STRSZ: i64 = 10;		/* Size of string table */
pub const DT_SYMENT: i64 = 11;		/* Size of one symbol table entry */
pub const DT_INIT: i64 = 12;		/* Address of init function */
pub const DT_FINI: i64 = 13;		/* Address of termination function */
pub const DT_SONAME: i64 = 14;		/* Name of shared object */
pub const DT_RPATH: i64 = 15;		/* Library search path (deprecated) */
pub const DT_SYMBOLIC: i64 = 16;		/* Start symbol search here */
pub const DT_REL: i64 = 17;		/* Address of Rel relocs */
pub const DT_RELSZ: i64 = 18;		/* Total size of Rel relocs */
pub const DT_RELENT: i64 = 19;		/* Size of one Rel reloc */
pub const DT_PLTREL: i64 = 20;		/* Type of reloc in PLT */
pub const DT_DEBUG: i64 = 21;		/* For debugging; unspecified */
pub const DT_TEXTREL: i64 = 22;		/* Reloc might modify .text */
pub const DT_JMPREL: i64 = 23;		/* Address of PLT relocs */
pub const DT_BIND_NOW: i64 = 24;		/* Process relocations of object */
pub const DT_INIT_ARRAY: i64 = 25;		/* Array with addresses of init fct */
pub const DT_FINI_ARRAY: i64 = 26;		/* Array with addresses of fini fct */
pub const DT_INIT_ARRAYSZ: i64 = 27;		/* Size in bytes of DT_INIT_ARRAY */
pub const DT_FINI_ARRAYSZ: i64 = 28;		/* Size in bytes of DT_FINI_ARRAY */
pub const DT_RUNPATH: i64 = 29;		/* Library search path */
pub const DT_FLAGS: i64 = 30;		/* Flags for the object being loaded */
pub const DT_ENCODING: i64 = 32;		/* Start of encoded range */
pub const DT_PREINIT_ARRAY: i64 = 32;		/* Array with addresses of preinit fct*/
pub const DT_PREINIT_ARRAYSZ: i64 = 33;		/* size in bytes of DT_PREINIT_ARRAY */
pub const DT_SYMTAB_SHNDX: i64 = 34;		/* Address of SYMTAB_SHNDX section */
pub const DT_NUM: i64 = 35;		/* Number used */
pub const DT_LOOS: i64 = 0xd;	/* Start of OS-specific */
pub const DT_HIOS: i64 = 0x0;	/* End of OS-specific */
pub const DT_LOPROC: i64 = 0x0;	/* Start of processor-specific */
pub const DT_HIPROC: i64 = 0xf;	/* End of processor-specific */
pub const DT_PROCNUM: u32 = 0x37;	/* Most used by any processor */

/* DT_* entries which fall between DT_VALRNGHI & DT_VALRNGLO use the
   Dyn.d_un.d_val field of the Elf*_Dyn structure.  This follows Sun's
   approach.  */
pub const DT_VALRNGLO: i64 = 0x0;
pub const DT_GNU_PRELINKED: i64 = 0x5;	/* Prelinking timestamp */
pub const DT_GNU_CONFLICTSZ: i64 = 0x6;	/* Size of conflict section */
pub const DT_GNU_LIBLISTSZ: i64 = 0x7;	/* Size of library list */
pub const DT_CHECKSUM: i64 = 0x8;
pub const DT_PLTPADSZ: i64 = 0x9;
pub const DT_MOVEENT: i64 = 0xa;
pub const DT_MOVESZ: i64 = 0xb;
pub const DT_FEATURE_1: i64 = 0xc;	/* Feature selection (DTF_*).  */
pub const DT_POSFLAG_1: i64 = 0xd;	/* Flags for DT_* entries, effecting
					   the following DT_* entry.  */
pub const DT_SYMINSZ: i64 = 0xe;	/* Size of syminfo table (in bytes) */
pub const DT_SYMINENT: i64 = 0xf;	/* Entry size of syminfo */
pub const DT_VALRNGHI: i64 = 0xf;
//pub const DT_VALTAGIDX(tag)	(DT_VALRNGHI - (tag))	/* Reverse order! */
fn DT_VALTAGIDX(tag: i64) -> i64 {
    DT_VALRNGHI - tag
}
pub const DT_VALNUM: i64 = 12;

/* DT_* entries which fall between DT_ADDRRNGHI & DT_ADDRRNGLO use the
   Dyn.d_un.d_ptr field of the Elf*_Dyn structure.

   If any adjustment is made to the ELF object after it has been
   built these entries will need to be adjusted.  */
pub const DT_ADDRRNGLO: i64 = 0x0;
pub const DT_GNU_HASH: i64 = 0x5;	/* GNU-style hash table.  */
pub const DT_TLSDESC_PLT: i64 = 0x6;
pub const DT_TLSDESC_GOT: i64 = 0x7;
pub const DT_GNU_CONFLICT: i64 = 0x8;	/* Start of conflict section */
pub const DT_GNU_LIBLIST: i64 = 0x9;	/* Library list */
pub const DT_CONFIG: i64 = 0xa;	/* Configuration information.  */
pub const DT_DEPAUDIT: i64 = 0xb;	/* Dependency auditing.  */
pub const DT_AUDIT: i64 = 0xc;	/* Object auditing.  */
pub const DT_PLTPAD: i64 = 0xd;	/* PLT padding.  */
pub const DT_MOVETAB: i64 = 0xe;	/* Move table.  */
pub const DT_SYMINFO: i64 = 0xf;	/* Syminfo table.  */
pub const DT_ADDRRNGHI: i64 = 0xf;
//pub const DT_ADDRTAGIDX(tag)	(DT_ADDRRNGHI - (tag))	/* Reverse order! */
fn DT_ADDRTAGIDX(tag: i64) -> i64 {
    DT_ADDRRNGHI - tag
}
pub const DT_ADDRNUM: i64 = 11;

/* The versioning entry i64s.  The next are defined as part of the
   GNU extension.  */
pub const DT_VERSYM: i64 = 0x0;

pub const DT_RELACOUNT: i64 = 0x9;
pub const DT_RELCOUNT: i64 = 0xa;

/* These were chosen by Sun.  */
pub const DT_FLAGS_1: i64 = 0xb;	/* State flags, see DF_1_* below.  */
pub const DT_VERDEF: i64 = 0xc;	/* Address of version definition
					   table */
pub const DT_VERDEFNUM: i64 = 0xd;	/* Number of version definitions */
pub const DT_VERNEED: i64 = 0xe;	/* Address of table with needed
					   versions */
pub const DT_VERNEEDNUM: i64 = 0xf;	/* Number of needed versions */
//pub const DT_VERSIONTAGIDX(tag)	(DT_VERNEEDNUM - (tag))	/* Reverse order! */
fn DT_VERSIONTAGIDX(tag: i64) -> i64 {
    DT_VERSIONTAGNUM - tag
}
pub const DT_VERSIONTAGNUM: i64 = 16;

/* Sun added these machine-independent extensions in the "processor-specific"
   range.  Be compatible.  */
pub const DT_AUXILIARY: i64 = 0xd;      /* Shared object to load before self */
pub const DT_FILTER: i64 = 0xf;      /* Shared object to get values from */
//pub const DT_EXTRATAGIDX(tag)	((u32)-((i32) (tag) <<1>>1)-1)
fn DT_EXTRATAGIDX(tag: u32) -> i64 {
    (((tag as i64) << 1 >> 1) - 1) as i64
}
pub const DT_EXTRANUM: i64 = 3;

/* Values of `d_un.d_val' in the DT_FLAGS entry.  */
pub const DF_ORIGIN: u64 = 0x1;	/* Object may use DF_ORIGIN */
pub const DF_SYMBOLIC: u64 = 0x2;	/* Symbol resolutions starts here */
pub const DF_TEXTREL: u64 = 0x4;	/* Object contains text relocations */
pub const DF_BIND_NOW: u64 = 0x8;	/* No lazy binding for this object */
pub const DF_STATIC_TLS: u64 = 0x0;	/* Module uses the static TLS model */

/* State flags selectable in the `d_un.d_val' element of the DT_FLAGS_1
   entry in the dynamic section.  */
pub const DF_1_NOW: u64 = 0x1;	/* Set RTLD_NOW for this object.  */
pub const DF_1_GLOBAL: u64 = 0x2;	/* Set RTLD_GLOBAL for this object.  */
pub const DF_1_GROUP: u64 = 0x4;	/* Set RTLD_GROUP for this object.  */
pub const DF_1_NODELETE: u64 = 0x8;	/* Set RTLD_NODELETE for this object.*/
pub const DF_1_LOADFLTR: u64 = 0x0;	/* Trigger filtee loading at runtime.*/
pub const DF_1_INITFIRST: u64 = 0x0;	/* Set RTLD_INITFIRST for this object*/
pub const DF_1_NOOPEN: u64 = 0x0;	/* Set RTLD_NOOPEN for this object.  */
pub const DF_1_ORIGIN: u64 = 0x0;	/* $ORIGIN must be handled.  */
pub const DF_1_DIRECT: u64 = 0x0;	/* Direct binding enabled.  */
pub const DF_1_TRANS: u64 = 0x0;
pub const DF_1_INTERPOSE: u64 = 0x0;	/* Object is used to interpose.  */
pub const DF_1_NODEFLIB: u64 = 0x0;	/* Ignore default lib search path.  */
pub const DF_1_NODUMP: u64 = 0x0;	/* Object can't be dldump'ed.  */
pub const DF_1_CONFALT: u64 = 0x0;	/* Configuration alternative created.*/
pub const DF_1_ENDFILTEE: u64 = 0x0;	/* Filtee terminates filters search. */
pub const DF_1_DISPRELDNE: u64 = 0x0;	/* Disp reloc applied at build time. */
pub const DF_1_DISPRELPND: u64 = 0x0;	/* Disp reloc applied at run-time.  */
pub const DF_1_NODIRECT: u64 = 0x0;	/* Object has no-direct binding. */
pub const DF_1_IGNMULDEF: u64 = 0x0;
pub const DF_1_NOKSYMS: u64 = 0x0;
pub const DF_1_NOHDR: u64 = 0x0;
pub const DF_1_EDITED: u64 = 0x0;	/* Object is modified after built.  */
pub const DF_1_NORELOC: u64 = 0x0;
pub const DF_1_SYMINTPOSE: u64 = 0x0;	/* Object has individual interposers.  */
pub const DF_1_GLOBAUDIT: u64 = 0x0;	/* Global auditing required.  */
pub const DF_1_SINGLETON: u64 = 0x0;	/* Singleton symbols are used.  */
pub const DF_1_STUB: u64 = 0x0;
pub const DF_1_PIE: u64 = 0x0;
pub const DF_1_KMOD: u64 = 0x0;
pub const DF_1_WEAKFILTER: u64 = 0x0;
pub const DF_1_NOCOMMON: u64 = 0x0;

/* Flags for the feature selection in DT_FEATURE_1.  */
pub const DTF_1_PARINIT: u64 = 0x1;
pub const DTF_1_CONFEXP: u64 = 0x2;

/* Flags in the DT_POSFLAG_1 entry effecting only the next DT_* entry.  */
pub const DF_P1_LAZYLOAD: u64 = 0x1;	/* Lazyload following object.  */
pub const DF_P1_GROUPPERM: u64 = 0x2;	/* Symbols from next object are not
					   generally available.  */
