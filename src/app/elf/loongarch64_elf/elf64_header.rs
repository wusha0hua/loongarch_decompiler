use super::super::loongarch_result::*;
use super::super::data_convert::*;
use super::super::elf_info::*;

use crate::app::*;

use std::fmt;

const EI_NIDENT: u8 = 16;

#[derive(Debug, Clone)]
pub struct Elf64Ehdr{
    e_ident: [u8; EI_NIDENT as usize],	/* Magic number and other info */
    e_type: u16,			/* Object file type */
    e_machine: u16,     /* Architecture */
    e_version: u32,		/* Object file version */
    e_entry: u64,		/* Entry point virtual address */
    pub e_phoff: u64,		/* Program header table file offset */
    pub e_shoff: u64,		/* Section header table file offset */
    e_flags: u32,		/* Processor-specific flags */
    e_ehsize: u16,		/* ELF header size in bytes */
    pub e_phentsize: u16,		/* Program header table entry size */
    pub e_phnum: u16,		/* Program header table entry count */
    pub e_shentsize: u16,		/* Section header table entry size */
    pub e_shnum: u16,		/* Section header table entry count */
    pub e_shstrndx: u16,		/* Section header string table index */
}

impl Elf64Ehdr {
    pub fn new() -> Self {
        Elf64Ehdr {
            e_ident: [0; EI_NIDENT as usize],	
            e_type: 0,		
            e_machine: 0,     
            e_version: 0,		
            e_entry: 0,		
            e_phoff: 0,		
            e_shoff: 0,		
            e_flags: 0,		
            e_ehsize: 0,		
            e_phentsize: 0,	
            e_phnum: 0,		
            e_shentsize: 0,	
            e_shnum: 0,		
            e_shstrndx: 0,	
        }
    }

    pub fn fill(&mut self, elf_bytes: &Vec<u8>) -> Result<ElfInfo, LoongArchError> {
        let mut elf_info: ElfInfo = ElfInfo{
            endian: Endianess::INVALID,
        };
        if elf_bytes.len()  < ELF64_EHDR_SIZE as usize {
            return Err(LoongArchError::NOTELFFILE); 
        }

		self.e_ident = clone_into_array(&elf_bytes[0..16]);
		self.e_type = u16::from_le_bytes(clone_into_array(&elf_bytes[16..18]));
		self.e_machine = u16::from_le_bytes(clone_into_array(&elf_bytes[18..20]));
		self.e_version = u32::from_le_bytes(clone_into_array(&elf_bytes[20..24]));
		self.e_entry = u64::from_le_bytes(clone_into_array(&elf_bytes[24..32]));
		self.e_phoff = u64::from_le_bytes(clone_into_array(&elf_bytes[32..40]));
		self.e_shoff = u64::from_le_bytes(clone_into_array(&elf_bytes[40..48]));
		self.e_flags = u32::from_le_bytes(clone_into_array(&elf_bytes[48..52]));
		self.e_ehsize = u16::from_le_bytes(clone_into_array(&elf_bytes[52..54]));
		self.e_phentsize = u16::from_le_bytes(clone_into_array(&elf_bytes[54..56]));
		self.e_phnum = u16::from_le_bytes(clone_into_array(&elf_bytes[56..58]));
		self.e_shentsize = u16::from_le_bytes(clone_into_array(&elf_bytes[58..60]));
		self.e_shnum = u16::from_le_bytes(clone_into_array(&elf_bytes[60..62]));
		self.e_shstrndx = u16::from_le_bytes(clone_into_array(&elf_bytes[62..64]));
        
        if self.e_ident[0] != ELFMAG0 || self.e_ident[1] != ELFMAG1 || self.e_ident[2] != ELFMAG2 || self.e_ident[3] != ELFMAG3 {
            return Err(LoongArchError::NOTELFFILE);
        }

        if self.e_ident[EI_CLASS as usize] != ELFCLASS64 {
            return Err(LoongArchError::NOTLOONGARCH64);
        }

        match self.e_ident[EI_DATA as usize] {
            ELFDATA2MSB => elf_info.endian = Endianess::BIGENDIAN,
            ELFDATA2LSB => elf_info.endian = Endianess::LITTLEENDIAN,
            _ => return Err(LoongArchError::UNKNOWNENDIAN(self.e_ident[EI_DATA as usize])) 
        }

        if self.e_ident[EI_VERSION as usize] != EV_CURRENT as u8 {
            return Err(LoongArchError::INVALIDELFVERSION(self.e_ident[EI_VERSION as usize])); 
        }

        if self.e_machine != EM_LOONGARCH {
            return Err(LoongArchError::NOTLOONGARCH);
        }

        if self.e_version != EV_CURRENT {
            return Err(LoongArchError::INVALIDELFVERSION(self.e_version as u8));
        }

        //println!("{}", self); 
        Ok(elf_info) 
    }
}

impl fmt::Display for Elf64Ehdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", "ELF Header:");
        write!(f, "  Magic:   ");
        for i in 0..EI_NIDENT as usize {
            write!(f, "{:02x} ", self.e_ident[i]); 
        }
        write!(f, "\n") ;
        write!(f, "  {:<35}{}\n", "Class:", "ELF64");
        write!(f, "  {:<35}", "Data:");
        if let ELFDATA2MSB = self.e_ident[EI_DATA as usize] {
            write!(f, "2's complement, big endian\n");
        } else {
            write!(f, "2's complement, little endian\n");
        }
        write!(f, "  {:<35}", "Version:");
        if let EV_CURRENT = self.e_version {
            write!(f, "1 (current)\n");
        }
        write!(f, "  {:<35}{}\n", "OS/ABI:", self.e_ident[EI_OSABI as usize]);
        write!(f, "  {:<35}{}\n", "ABI Version:", self.e_ident[EI_ABIVERSION as usize]);
        if let ET_DYN = self.e_type {
            write!(f, "  {:<35}{}\n", "Type:", "DYN (Position-Independent Executable file)");
        } 
        if let EM_LOONGARCH = self.e_machine {
            write!(f, "  {:<35}{}\n", "Machine:", "LoongArch");
        }
        write!(f, "  {:<35}0x{:x}\n", "Entry point address:", self.e_entry);
        write!(f, "  {:<35}{}\n", "Start of program headers:", self.e_phoff);
        write!(f, "  {:<35}{}\n", "Start of section headers:", self.e_shoff);
        write!(f, "  {:<35}{}\n", "Flags:", self.e_flags);
        write!(f, "  {:<35}{}\n", "Size of this header:", self.e_ehsize);
        write!(f, "  {:<35}{}\n", "Size of program headers:", self.e_phentsize);
        write!(f, "  {:<35}{}\n", "Number of program headers:", self.e_phnum); 
        write!(f, "  {:<35}{}\n", "Size of section headers:", self.e_shentsize);
        write!(f, "  {:<35}{}\n", "Number of section headers:", self.e_shnum);
        write!(f, "  {:<35}{}", "Section header string table index:", self.e_shstrndx)
    }
}


const ELF64_EHDR_SIZE: u64 = 64;

const ELFMAG0: u8 = 0x7f;		/* Magic number byte 0 */
const ELFMAG1: u8 = 'E' as u8;		/* Magic number byte 1 */
const ELFMAG2: u8 = 'L' as u8;		/* Magic number byte 2 */
const ELFMAG3: u8 = 'F' as u8;		/* Magic number byte 3 */

const EI_CLASS: u8 = 4;		/* File class byte index */
const ELFCLASSNONE: u8 = 0;		/* Invalid class */
const ELFCLASS32: u8 = 1;		/* 32-bit objects */
const ELFCLASS64: u8 = 2;		/* 64-bit objects */
const ELFCLASSNUM: u8 = 3;

const EI_DATA: u8 = 5;		/* Data encoding byte index */
const ELFDATANONE: u8 = 0;		/* Invalid data encoding */
const ELFDATA2LSB: u8 = 1;		/* 2's complement, little endian */
const ELFDATA2MSB: u8 = 2;		/* 2's complement, big endian */
const ELFDATANUM: u8 = 3;

const EI_VERSION: u8 = 6;		/* File version byte index */
					/* Value must be EV_CURRENT */

const EI_OSABI: u8 = 7;		/* OS ABI identification */
const ELFOSABI_NONE: u8 = 0;	/* UNIX System V ABI */
const ELFOSABI_SYSV: u8 = 0;	/* Alias.  */
const ELFOSABI_HPUX: u8 = 1;	/* HP-UX */
const ELFOSABI_NETBSD: u8 = 2;	/* NetBSD.  */
const ELFOSABI_GNU: u8 = 3;	/* Object uses GNU ELF extensions.  */
const ELFOSABI_LINUX: u8 = ELFOSABI_GNU; /* Compatibility alias.  */
const ELFOSABI_SOLARIS: u8 = 6;	/* Sun Solaris.  */
const ELFOSABI_AIX: u8 = 7;	/* IBM AIX.  */
const ELFOSABI_IRIX: u8 = 8;	/* SGI Irix.  */
const ELFOSABI_FREEBSD: u8 = 9;	/* FreeBSD.  */
const ELFOSABI_TRU64: u8 = 10;	/* Compaq TRU64 UNIX.  */
const ELFOSABI_MODESTO: u8 = 11;	/* Novell Modesto.  */
const ELFOSABI_OPENBSD: u8 = 12;	/* OpenBSD.  */
const ELFOSABI_ARM_AEABI: u8 = 64;	/* ARM EABI */
const ELFOSABI_ARM: u8 = 97;	/* ARM */
const ELFOSABI_STANDALONE: u8 = 255;	/* Standalone (embedded) application */

const EI_ABIVERSION: u8 = 8;		/* ABI version */

const EI_PAD: u8 = 9;		/* Byte index of padding bytes */

/* Legal values for e_u8 (object file u8).  */

const ET_NONE: u16 = 0;		/* No file u8 */
const ET_REL: u16 = 1;		/* Relocatable file */
const ET_EXEC: u16 = 2;		/* Executable file */
const ET_DYN: u16 = 3;		/* Shared object file */
const ET_CORE: u16 = 4;		/* Core file */
const ET_NUM: u16 = 5;		/* Number of defined u8s */
const ET_LOOS: u16 = 0xfe00;		/* OS-specific range start */
const ET_HIOS: u16 = 0xfeff;  	/* OS-specific range end */
const ET_LOPROC: u16 = 0xff00;		/* Processor-specific range start */
const ET_HIPROC: u16 = 0xffff;		/* Processor-specific range end */

/* Legal values for e_machine (architecture).  */

const EM_NONE: u16 = 0;	/* No machine */
const EM_M32: u16 = 1;	/* AT&T WE 32100 */
const EM_SPARC: u16 = 2;	/* SUN SPARC */
const EM_386: u16 = 3;	/* Intel 80386 */
const EM_68K: u16 = 4;	/* Motorola m68k family */
const EM_88K: u16 = 5;	/* Motorola m88k family */
const EM_IAMCU: u16 = 6;	/* Intel MCU */
const EM_860: u16 = 7;	/* Intel 80860 */
const EM_MIPS: u16 = 8;	/* MIPS R3000 big-endian */
const EM_S370: u16 = 9;	/* IBM System/370 */
const EM_MIPS_RS3_LE: u16 = 10;	/* MIPS R3000 little-endian */
				/* reserved 11-14 */
const EM_PARISC: u16 = 15;	/* HPPA */
				/* reserved 16 */
const EM_VPP500: u16 = 17;	/* Fujitsu VPP500 */
const EM_SPARC32PLUS: u16 = 18;	/* Sun's "v8plus" */
const EM_960: u16 = 19;	/* Intel 80960 */
const EM_PPC: u16 = 20;	/* PowerPC */
const EM_PPC64: u16 = 21;	/* PowerPC 64-bit */
const EM_S390: u16 = 22;	/* IBM S390 */
const EM_SPU: u16 = 23;	/* IBM SPU/SPC */
				/* reserved 24-35 */
const EM_V800: u16 = 36;	/* NEC V800 series */
const EM_FR20: u16 = 37;	/* Fujitsu FR20 */
const EM_RH32: u16 = 38;	/* TRW RH-32 */
const EM_RCE: u16 = 39;	/* Motorola RCE */
const EM_ARM: u16 = 40;	/* ARM */
const EM_FAKE_ALPHA: u16 = 41;	/* Digital Alpha */
const EM_SH: u16 = 42;	/* Hitachi SH */
const EM_SPARCV9: u16 = 43;	/* SPARC v9 64-bit */
const EM_TRICORE: u16 = 44;	/* Siemens Tricore */
const EM_ARC: u16 = 45;	/* Argonaut RISC Core */
const EM_H8_300: u16 = 46;	/* Hitachi H8/300 */
const EM_H8_300H: u16 = 47;	/* Hitachi H8/300H */
const EM_H8S: u16 = 48;	/* Hitachi H8S */
const EM_H8_500: u16 = 49;	/* Hitachi H8/500 */
const EM_IA_64: u16 = 50;	/* Intel Merced */
const EM_MIPS_X: u16 = 51;	/* Stanford MIPS-X */
const EM_COLDFIRE: u16 = 52;	/* Motorola Coldfire */
const EM_68HC12: u16 = 53;	/* Motorola M68HC12 */
const EM_MMA: u16 = 54;	/* Fujitsu MMA Multimedia Accelerator */
const EM_PCP: u16 = 55;	/* Siemens PCP */
const EM_NCPU: u16 = 56;	/* Sony nCPU embeeded RISC */
const EM_NDR1: u16 = 57;	/* Denso NDR1 microprocessor */
const EM_STARCORE: u16 = 58;	/* Motorola Start*Core processor */
const EM_ME16: u16 = 59;	/* Toyota ME16 processor */
const EM_ST100: u16 = 60;	/* STMicroelectronic ST100 processor */
const EM_TINYJ: u16 = 61;	/* Advanced Logic Corp. Tinyj emb.fam */
const EM_X86_64: u16 = 62;	/* AMD x86-64 architecture */
const EM_PDSP: u16 = 63;	/* Sony DSP Processor */
const EM_PDP10: u16 = 64;	/* Digital PDP-10 */
const EM_PDP11: u16 = 65;	/* Digital PDP-11 */
const EM_FX66: u16 = 66;	/* Siemens FX66 microcontroller */
const EM_ST9PLUS: u16 = 67;	/* STMicroelectronics ST9+ 8/16 mc */
const EM_ST7: u16 = 68;	/* STmicroelectronics ST7 8 bit mc */
const EM_68HC16: u16 = 69;	/* Motorola MC68HC16 microcontroller */
const EM_68HC11: u16 = 70;	/* Motorola MC68HC11 microcontroller */
const EM_68HC08: u16 = 71;	/* Motorola MC68HC08 microcontroller */
const EM_68HC05: u16 = 72;	/* Motorola MC68HC05 microcontroller */
const EM_SVX: u16 = 73;	/* Silicon Graphics SVx */
const EM_ST19: u16 = 74;	/* STMicroelectronics ST19 8 bit mc */
const EM_VAX: u16 = 75;	/* Digital VAX */
const EM_CRIS: u16 = 76;	/* Axis Communications 32-bit emb.proc */
const EM_JAVELIN: u16 = 77;	/* Infineon Technologies 32-bit emb.proc */
const EM_FIREPATH: u16 = 78;	/* Element 14 64-bit DSP Processor */
const EM_ZSP: u16 = 79;	/* LSI Logic 16-bit DSP Processor */
const EM_MMIX: u16 = 80;	/* Donald Knuth's educational 64-bit proc */
const EM_HUANY: u16 = 81;	/* Harvard University machine-independent object files */
const EM_PRISM: u16 = 82;	/* SiTera Prism */
const EM_AVR: u16 = 83;	/* Atmel AVR 8-bit microcontroller */
const EM_FR30: u16 = 84;	/* Fujitsu FR30 */
const EM_D10V: u16 = 85;	/* Mitsubishi D10V */
const EM_D30V: u16 = 86;	/* Mitsubishi D30V */
const EM_V850: u16 = 87;	/* NEC v850 */
const EM_M32R: u16 = 88;	/* Mitsubishi M32R */
const EM_MN10300: u16 = 89;	/* Matsushita MN10300 */
const EM_MN10200: u16 = 90;	/* Matsushita MN10200 */
const EM_PJ: u16 = 91;	/* picoJava */
const EM_OPENRISC: u16 = 92;	/* OpenRISC 32-bit embedded processor */
const EM_ARC_COMPACT: u16 = 93;	/* ARC International ARCompact */
const EM_XTENSA: u16 = 94;	/* Tensilica Xtensa Architecture */
const EM_VIDEOCORE: u16 = 95;	/* Alphamosaic VideoCore */
const EM_TMM_GPP: u16 = 96;	/* Thompson Multimedia General Purpose Proc */
const EM_NS32K: u16 = 97;	/* National Semi. 32000 */
const EM_TPC: u16 = 98;	/* Tenor Network TPC */
const EM_SNP1K: u16 = 99;	/* Trebia SNP 1000 */
const EM_ST200: u16 = 100;	/* STMicroelectronics ST200 */
const EM_IP2K: u16 = 101;	/* Ubicom IP2xxx */
const EM_MAX: u16 = 102;	/* MAX processor */
const EM_CR: u16 = 103;	/* National Semi. CompactRISC */
const EM_F2MC16: u16 = 104;	/* Fujitsu F2MC16 */
const EM_MSP430: u16 = 105;	/* Texas Instruments msp430 */
const EM_BLACKFIN: u16 = 106;	/* Analog Devices Blackfin DSP */
const EM_SE_C33: u16 = 107;	/* Seiko Epson S1C33 family */
const EM_SEP: u16 = 108;	/* Sharp embedded microprocessor */
const EM_ARCA: u16 = 109;	/* Arca RISC */
const EM_UNICORE: u16 = 110;	/* PKU-Unity & MPRC Peking Uni. mc series */
const EM_EXCESS: u16 = 111;	/* eXcess configurable cpu */
const EM_DXP: u16 = 112;	/* Icera Semi. Deep Execution Processor */
const EM_ALTERA_NIOS2: u16 = 113;	/* Altera Nios II */
const EM_CRX: u16 = 114;	/* National Semi. CompactRISC CRX */
const EM_XGATE: u16 = 115;	/* Motorola XGATE */
const EM_C166: u16 = 116;	/* Infineon C16x/XC16x */
const EM_M16C: u16 = 117;	/* Renesas M16C */
const EM_DSPIC30F: u16 = 118;	/* Microchip Technology dsPIC30F */
const EM_CE: u16 = 119;	/* Freescale Communication Engine RISC */
const EM_M32C: u16 = 120;	/* Renesas M32C */
				/* reserved 121-130 */
const EM_TSK3000: u16 = 131;	/* Altium TSK3000 */
const EM_RS08: u16 = 132;	/* Freescale RS08 */
const EM_SHARC: u16 = 133;	/* Analog Devices SHARC family */
const EM_ECOG2: u16 = 134;	/* Cyan Technology eCOG2 */
const EM_SCORE7: u16 = 135;	/* Sunplus S+core7 RISC */
const EM_DSP24: u16 = 136;	/* New Japan Radio (NJR) 24-bit DSP */
const EM_VIDEOCORE3: u16 = 137;	/* Broadcom VideoCore III */
const EM_LATTICEMICO32: u16 = 138;	/* RISC for Lattice FPGA */
const EM_SE_C17: u16 = 139;	/* Seiko Epson C17 */
const EM_TI_C6000: u16 = 140;	/* Texas Instruments TMS320C6000 DSP */
const EM_TI_C2000: u16 = 141;	/* Texas Instruments TMS320C2000 DSP */
const EM_TI_C5500: u16 = 142;	/* Texas Instruments TMS320C55x DSP */
const EM_TI_ARP32: u16 = 143;	/* Texas Instruments App. Specific RISC */
const EM_TI_PRU: u16 = 144;	/* Texas Instruments Prog. Realtime Unit */
				/* reserved 145-159 */
const EM_MMDSP_PLUS: u16 = 160;	/* STMicroelectronics 64bit VLIW DSP */
const EM_CYPRESS_M8C: u16 = 161;	/* Cypress M8C */
const EM_R32C: u16 = 162;	/* Renesas R32C */
const EM_TRIMEDIA: u16 = 163;	/* NXP Semi. TriMedia */
const EM_QDSP6: u16 = 164;	/* QUALCOMM DSP6 */
const EM_8051: u16 = 165;	/* Intel 8051 and variants */
const EM_STXP7X: u16 = 166;	/* STMicroelectronics STxP7x */
const EM_NDS32: u16 = 167;	/* Andes Tech. compact code emb. RISC */
const EM_ECOG1X: u16 = 168;	/* Cyan Technology eCOG1X */
const EM_MAXQ30: u16 = 169;	/* Dallas Semi. MAXQ30 mc */
const EM_XIMO16: u16 = 170;	/* New Japan Radio (NJR) 16-bit DSP */
const EM_MANIK: u16 = 171;	/* M2000 Reconfigurable RISC */
const EM_CRAYNV2: u16 = 172;	/* Cray NV2 vector architecture */
const EM_RX: u16 = 173;	/* Renesas RX */
const EM_METAG: u16 = 174;	/* Imagination Tech. META */
const EM_MCST_ELBRUS: u16 = 175;	/* MCST Elbrus */
const EM_ECOG16: u16 = 176;	/* Cyan Technology eCOG16 */
const EM_CR16: u16 = 177;	/* National Semi. CompactRISC CR16 */
const EM_ETPU: u16 = 178;	/* Freescale Extended Time Processing Unit */
const EM_SLE9X: u16 = 179;	/* Infineon Tech. SLE9X */
const EM_L10M: u16 = 180;	/* Intel L10M */
const EM_K10M: u16 = 181;	/* Intel K10M */
				/* reserved 182 */
const EM_AARCH64: u16 = 183;	/* ARM AARCH64 */
				/* reserved 184 */
const EM_AVR32: u16 = 185;	/* Amtel 32-bit microprocessor */
const EM_STM8: u16 = 186;	/* STMicroelectronics STM8 */
const EM_TILE64: u16 = 187;	/* Tilera TILE64 */
const EM_TILEPRO: u16 = 188;	/* Tilera TILEPro */
const EM_MICROBLAZE: u16 = 189;	/* Xilinx MicroBlaze */
const EM_CUDA: u16 = 190;	/* NVIDIA CUDA */
const EM_TILEGX: u16 = 191;	/* Tilera TILE-Gx */
const EM_CLOUDSHIELD: u16 = 192;	/* CloudShield */
const EM_COREA_1ST: u16 = 193;	/* KIPO-KAIST Core-A 1st gen. */
const EM_COREA_2ND: u16 = 194;	/* KIPO-KAIST Core-A 2nd gen. */
const EM_ARCV2: u16 = 195;	/* Synopsys ARCv2 ISA.  */
const EM_OPEN8: u16 = 196;	/* Open8 RISC */
const EM_RL78: u16 = 197;	/* Renesas RL78 */
const EM_VIDEOCORE5: u16 = 198;	/* Broadcom VideoCore V */
const EM_78KOR: u16 = 199;	/* Renesas 78KOR */
const EM_56800EX: u16 = 200;	/* Freescale 56800EX DSC */
const EM_BA1: u16 = 201;	/* Beyond BA1 */
const EM_BA2: u16 = 202;	/* Beyond BA2 */
const EM_XCORE: u16 = 203;	/* XMOS xCORE */
const EM_MCHP_PIC: u16 = 204;	/* Microchip 8-bit PIC(r) */
const EM_INTELGT: u16 = 205;	/* Intel Graphics Technology */
				/* reserved 206-209 */
const EM_KM32: u16 = 210;	/* KM211 KM32 */
const EM_KMX32: u16 = 211;	/* KM211 KMX32 */
const EM_EMX16: u16 = 212;	/* KM211 KMX16 */
const EM_EMX8: u16 = 213;	/* KM211 KMX8 */
const EM_KVARC: u16 = 214;	/* KM211 KVARC */
const EM_CDP: u16 = 215;	/* Paneve CDP */
const EM_COGE: u16 = 216;	/* Cognitive Smart Memory Processor */
const EM_COOL: u16 = 217;	/* Bluechip CoolEngine */
const EM_NORC: u16 = 218;	/* Nanoradio Optimized RISC */
const EM_CSR_KALIMBA: u16 = 219;	/* CSR Kalimba */
const EM_Z80: u16 = 220;	/* Zilog Z80 */
const EM_VISIUM: u16 = 221;	/* Controls and Data Services VISIUMcore */
const EM_FT32: u16 = 222;	/* FTDI Chip FT32 */
const EM_MOXIE: u16 = 223;	/* Moxie processor */
const EM_AMDGPU: u16 = 224;	/* AMD GPU */
				/* reserved 225-242 */
const EM_RISCV: u16 = 243;	/* RISC-V */

const EM_BPF: u16 = 247;	/* Linux BPF -- in-kernel virtual machine */
const EM_CSKY: u16 = 252;     /* C-SKY */

const EM_NUM: u16 = 253;

/* Old spellings/synonyms.  */

const EM_ARC_A5: u16	= EM_ARC_COMPACT;

const EM_LOONGARCH: u16 = 258;

/* If it is necessary to assign new unofficial EM_* values, please
   pick large random numbers (0x8523, 0xa7f2, etc.) to minimize the
   chances of collision with official or non-GNU unofficial values.  */

const EM_ALPHA: u16 = 0x9026;

/* Legal values for e_version (version).  */

const EV_NONE: u32 = 0;		/* Invalid ELF version */
const EV_CURRENT: u32 = 1;		/* Current version */
const EV_NUM: u32 = 2;
