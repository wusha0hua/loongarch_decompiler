//use crate::app::disassembler
pub use crate::app::disassembler::*;


/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyInstruction {
    pub address: usize,
    pub label: Option<String>,
    pub bytes: [u8; 4],
    pub opcode: Opcode,
    pub operand1: Option<Operand>,
    pub operand2: Option<Operand>,
    pub operand3: Option<Operand>,
    pub operand4: Option<Operand>,
    pub regs_write: Vec<Register>,
    pub regs_read: Vec<Register>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operand {
    pub operand_type: OperandType,
    pub value: usize,
    pub symbol: Option<SymbolRecord>,
}

impl Operand {
    pub fn new() -> Self {
        Operand {
            operand_type: OperandType::GeneralRegister,
            value: 0,
            symbol: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperandType {
    GeneralRegister,
    FloatRegister,
    UnsignedImm,
    SignedImm,
    Offset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolRecord {
    pub offset: usize,
    pub name: String,
    pub size: usize,
    pub sym_type: SymbolType,
}

impl SymbolRecord {
    pub fn new() -> Self {
        SymbolRecord {
            offset: 0,
            name: String::new(),
            size: 0,
            sym_type: SymbolType::Func,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynSymbolRecord {
    pub offset: usize,
    pub name: String,
    pub size: usize,
    pub sym_type: SymbolType,
    pub reloc_type: RelactionType,
    pub value: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelactionType {
    R_LARCH_NONE,
    R_LARCH_64,
    R_LARCH_RELATIVE,
    R_LARCH_JUMP_SLOT,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Register {
    GR(usize),
    FR(usize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GR {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
    R19,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R30,
    R31,
}
#[derive(Debug, Clone)]
pub enum FR {
    F0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
}

const R0: usize = 0;
const R1: usize = 1;
const R2: usize = 2;
const R3: usize = 3;
const R4: usize = 4;
const R5: usize = 5;
const R6: usize = 6;
const R7: usize = 7;
const R8: usize = 8;
const R9: usize = 9;
const R10: usize = 10;
const R11: usize = 11;
const R12: usize = 12;
const R13: usize = 13;
const R14: usize = 14;
const R15: usize = 15;
const R16: usize = 16;
const R17: usize = 17;
const R18: usize = 18;
const R19: usize = 19;
const R20: usize = 20;
const R21: usize = 21;
const R22: usize = 22;
const R23: usize = 23;
const R24: usize = 24;
const R25: usize = 25;
const R26: usize = 26;
const R27: usize = 27;
const R28: usize = 28;
const R29: usize = 29;
const R30: usize = 30;
const R31: usize = 31;

const F0: usize = 0;
const F1: usize = 1;
const F2: usize = 2;
const F3: usize = 3;
const F4: usize = 4;
const F5: usize = 5;
const F6: usize = 6;
const F7: usize = 7;
const F8: usize = 8;
const F9: usize = 9;
const F10: usize = 10;
const F11: usize = 11;
const F12: usize = 12;
const F13: usize = 13;
const F14: usize = 14;
const F15: usize = 15;
const F16: usize = 16;

pub fn get_gr_from_value(value: usize) -> GR {
    match value {
        0 => GR::R0,
        1 => GR::R1,
        2 => GR::R2,
        3 => GR::R3,
        4 => GR::R4,
        5 => GR::R5,
        6 => GR::R6,
        7 => GR::R7,
        8 => GR::R8,
        9 => GR::R9,
        10 => GR::R10,
        11 => GR::R11,
        12 => GR::R12,
        13 => GR::R13,
        14 => GR::R14,
        15 => GR::R15,
        16 => GR::R16,
        17 => GR::R17,
        18 => GR::R18,
        19 => GR::R19,
        20 => GR::R20,
        21 => GR::R21,
        22 => GR::R22,
        23 => GR::R23,
        24 => GR::R24,
        25 => GR::R25,
        26 => GR::R26,
        27 => GR::R27,
        28 => GR::R28,
        29 => GR::R29,
        30 => GR::R30,
        31 => GR::R31,
        _ => panic!("R{}\n", &value),
    }
}
pub fn get_fr_from_value(value: usize) -> FR {
    match value {
        0 => FR::F0,
        1 => FR::F1,
        2 => FR::F2,
        3 => FR::F3,
        4 => FR::F4,
        5 => FR::F5,
        6 => FR::F6,
        7 => FR::F7,
        8 => FR::F8,
        9 => FR::F9,
        10 => FR::F10,
        11 => FR::F11,
        12 => FR::F12,
        13 => FR::F13,
        14 => FR::F14,
        15 => FR::F15,
        16 => FR::F16,
        17 => FR::F17,
        18 => FR::F18,
        19 => FR::F19,
        20 => FR::F20,
        21 => FR::F21,
        22 => FR::F22,
        23 => FR::F23,
        24 => FR::F24,
        25 => FR::F25,
        26 => FR::F26,
        27 => FR::F27,
        28 => FR::F28,
        29 => FR::F29,
        30 => FR::F30,
        31 => FR::F31,
        _ => panic!("F{}\n", &value),
    }
}
use std::fmt;
use serde_json;
use serde::{Serialize, Deserialize};


#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
// 2R
pub const CLO_W: usize = 4;
pub const CLZ_W: usize = 5;
pub const CTO_W: usize = 6;
pub const CTZ_W: usize = 7;
pub const CLO_D: usize = 8;
pub const CLZ_D: usize = 9;
pub const CTO_D: usize = 10;
pub const CTZ_D: usize = 11;
pub const REVB_2H: usize = 12;
pub const REVB_4H: usize = 13;
pub const REVB_2W: usize = 14;
pub const REVB_D: usize = 15;
pub const REVH_2W: usize = 16;
pub const REVH_D: usize = 17;
pub const BITREV_4B: usize = 18;
pub const BITREV_8B: usize = 19;
pub const BITREV_W: usize = 20;
pub const BITREV_D: usize = 21;
pub const EXT_W_H: usize = 22;
pub const EXT_W_B: usize = 23;
pub const RDTIMEL_W: usize = 24;
pub const RDTIMEH_W: usize = 25;
pub const RDTIME_D: usize = 26;
pub const CPUCFG: usize = 27;
pub const FSQRT_S: usize = 17681;
pub const FSQRT_D: usize = 17682;
pub const FRECIP_S: usize = 17683;
pub const FRECIP_D: usize = 17684; 
pub const FRSQRT_S: usize = 17689; 
pub const FRSQRT_D: usize = 17690; 
pub const FMOV_S: usize = 17701;
pub const FMOV_D: usize = 17702;
pub const MOVGR2FR_W: usize = 17705;
pub const MOVGR2FR_D: usize = 17706;
pub const MOVGR2FRH_W: usize = 17707; 
pub const MOVFR2GR_S: usize = 17709;
pub const MOVFR2GR_D: usize = 17710;
pub const MOVFRH2GR_S: usize = 17711;
pub const MOVGR2FCSR: usize = 17712;
pub const MOVFCSR2GR: usize = 17714;
pub const MOVFR2CF: usize = 17716; 
pub const MOVCF2FR: usize = 17717; 
pub const MOVGR2CF: usize = 17718; 
pub const MOVCF2GR: usize = 17719; 
pub const FCVT_S_D: usize = 17990; 
pub const FCVT_D_S: usize = 17993; 
pub const FTINTRM_W_S: usize = 18049;
pub const FTINTRM_W_D: usize = 18050;
pub const FTINTRM_L_S: usize = 18057;
pub const FTINTRM_L_D: usize = 18058;
pub const FTINTRP_W_S: usize = 18065;
pub const FTINTRP_W_D: usize = 18066;
pub const FTINTRP_L_S: usize = 18073;
pub const FTINTRP_L_D: usize = 18074;
pub const FTINTRZ_W_S: usize = 18081;
pub const FTINTRZ_W_D: usize = 18082;
pub const FTINTRZ_L_S: usize = 18089;
pub const FTINTRZ_L_D: usize = 18090;
pub const FTINTRNE_W_S: usize = 18097;
pub const FTINTRNE_W_D: usize = 18098;
pub const FTINTRNE_L_S: usize = 18105;
pub const FTINTRNE_L_D: usize = 18106;
pub const FTINT_W_S: usize = 18113;
pub const FTINT_W_D: usize = 18114;
pub const FTINT_L_S: usize = 18121;
pub const FTINT_L_D: usize = 18122;
pub const FFINT_S_W: usize = 18244;
pub const FFINT_S_L: usize = 18246;
pub const FFINT_D_W: usize = 18248;
pub const FFINT_D_L: usize = 18250;
pub const FFINT_S: usize = 18321;
pub const FRINT_D: usize = 18322;
pub const IOCSRRD_B: usize = 102912;
pub const IOCSRRD_H: usize = 102913;
pub const IOCSRRD_W: usize = 102914;
pub const IOCSRRD_D: usize = 102915;
pub const IOCSRWR_B: usize = 102916;
pub const IOCSRWR_H: usize = 102917;
pub const IOCSRWR_W: usize = 102918;
pub const IOCSRWR_D: usize = 102919;
pub const TLBCLR: usize = 102920;
pub const TLBFLUSH: usize = 102921;
pub const TLBSRCH: usize = 102922; 
pub const TLBRD: usize = 102923; 
pub const TLBWR: usize = 102924; 
pub const TLBFILL: usize = 102925; 
pub const ERTN: usize = 102926;
pub const FABS_S: usize = 17665; 
pub const FABS_D: usize = 17666; 
pub const FNEG_S: usize = 17669; 
pub const FNEG_D: usize = 17670; 
pub const FLOGB_S: usize = 17673;
pub const FLOGB_D: usize = 17674;
pub const FCLASS_S: usize = 17677;
pub const FCLASS_D: usize = 17678;

// 3R
pub const ASRTLE_D: usize = 2;
pub const ASRTGT_D: usize = 3;
pub const ADD_W: usize = 32;
pub const ADD_D: usize = 33;
pub const SUB_W: usize = 34;
pub const SUB_D: usize = 35;
pub const SLT: usize = 36;
pub const SLTU: usize = 37;
pub const MASKEQZ: usize = 38;
pub const MASKNEZ: usize = 39;
pub const NOR: usize = 40;
pub const AND: usize = 41;
pub const OR: usize = 42;
pub const XOR: usize = 43;
pub const ORN: usize = 44;
pub const ANDN: usize = 45;
pub const SLL_W: usize = 46;
pub const SRL_W: usize = 47;
pub const SRA_W: usize = 48;
pub const SLL_D: usize = 49;
pub const SRL_D: usize = 50;
pub const SRA_D: usize = 51;
pub const ROTR_W: usize = 54; 
pub const ROTR_D: usize = 55; 
pub const MUL_W: usize = 56;
pub const MULH_W: usize = 57; 
pub const MULH_WU: usize = 58;
pub const MUL_D: usize = 59;
pub const MULH_D: usize = 60; 
pub const MULH_DU: usize = 61;
pub const MULW_D_W: usize = 62;
pub const MULW_D_WU: usize = 63;
pub const DIV_W: usize = 64;
pub const MOD_W: usize = 65;
pub const DIV_WU: usize = 66; 
pub const MOD_WU: usize = 67; 
pub const DIV_D: usize = 68;
pub const MOD_D: usize = 69;
pub const DIV_DU: usize = 70; 
pub const MOD_DU: usize = 71; 
pub const CRC_W_B_W: usize = 72;
pub const CRC_W_H_W: usize = 73;
pub const CRC_W_W_W: usize = 74;
pub const CRC_W_D_W: usize = 75;
pub const CRCC_W_B_W: usize = 76;
pub const CRCC_W_H_W: usize = 77;
pub const CRCC_W_W_W: usize = 78;
pub const CRCC_W_D_W: usize = 79;
pub const BREAK: usize = 84;
pub const DBCL: usize = 85;
pub const SYSCALL: usize = 86;
pub const FADD_S: usize = 513;
pub const FADD_D: usize = 514;
pub const FSUB_S: usize = 517;
pub const FSUB_D: usize = 518;
pub const FMUL_S: usize = 521;
pub const FMUL_D: usize = 522;
pub const FDIV_S: usize = 526;
pub const FDIV_D: usize = 527;
pub const FMAX_S: usize = 529;
pub const FMAX_D: usize = 530;
pub const FMIN_S: usize = 533;
pub const FMIN_D: usize = 534;
pub const FMAXA_S: usize = 537;
pub const FMAXA_D: usize = 538;
pub const FMINA_S: usize = 541;
pub const FMINA_D: usize = 542;
pub const FSCALEB_S: usize = 545;
pub const FSCALEB_D: usize = 546;
pub const FCOPYSIGN_S: usize = 549;
pub const FCOPYSIGN_D: usize = 550;
pub const IDLE: usize = 3217;
pub const INVTLB: usize = 3219;
pub const LDX_B: usize = 28672;
pub const LDX_H: usize = 28680;
pub const LDX_W: usize = 28688;
pub const LDX_D: usize = 28696;
pub const STX_B: usize = 28704;
pub const STX_H: usize = 28712;
pub const STX_W: usize = 28720;
pub const STX_D: usize = 28728;
pub const LDX_BU: usize = 28736;
pub const LDX_HU: usize = 28744;
pub const LDX_WU: usize = 28752;
pub const PRELDX: usize = 28760;
pub const FLDX_S: usize = 28768; 
pub const FLDX_D: usize = 28776; 
pub const FSTX_S: usize = 28784; 
pub const FSTX_D: usize = 28792; 
pub const AMSWAP_W: usize = 28864;
pub const AMSWAP_D: usize = 28865;
pub const AMADD_W: usize = 28866; 
pub const AMADD_D: usize = 28867; 
pub const AMAND_W: usize = 28868; 
pub const AMAND_D: usize = 28869; 
pub const AMOR_W: usize = 28870; 
pub const AMOR_D: usize = 28871; 
pub const AMXOR_W: usize = 28872; 
pub const AMXOR_D: usize = 28873; 
pub const AMMAX_W: usize = 28874; 
pub const AMMAX_D: usize = 28875; 
pub const AMMIN_W: usize = 28876; 
pub const AMMIN_D: usize = 28877; 
pub const AMMAX_WU: usize = 28878;
pub const AMMAX_DU: usize = 28879;
pub const AMMIN_WU: usize = 28880;
pub const AMMIN_DU: usize = 28881;
pub const AMSWAP_DB_W: usize = 28882;
pub const AMSWAP_DB_D: usize = 28883;
pub const AMADD_DB_W: usize = 28884;
pub const AMADD_DB_D: usize = 28885;
pub const AMAND_DB_W: usize = 28886;
pub const AMAND_DB_D: usize = 28887;
pub const AMOR_DB_W: usize = 28888;
pub const AMOR_DB_D: usize = 28889;
pub const AMXOR_DB_W: usize = 28890;
pub const AMXOR_DB_D: usize = 28891;
pub const AMMAX_DB_W: usize = 28892;
pub const AMMAX_DB_D: usize = 28893;
pub const AMMIN_DB_W: usize = 28894;
pub const AMMIN_DB_D: usize = 28895;                  
pub const AMMAX_DB_WU: usize = 28896;                   
pub const AMMAX_DB_DU: usize = 28897;                   
pub const AMMIN_DB_WU: usize = 28898;                   
pub const AMMIN_DB_DU: usize = 28899;                  
pub const DBAR: usize = 28900;                
pub const IBAR: usize = 28901;                
pub const FLDGT_S: usize = 28904;                  
pub const FLDGT_D: usize = 28905;                  
pub const FLDLE_S: usize = 28906;                  
pub const FLDLE_D: usize = 28907;                  
pub const FSTGT_S: usize = 28908;                  
pub const FSTGT_D: usize = 28909;                  
pub const FSTLE_S: usize = 28910;                  
pub const FSTLE_D: usize = 28911;                  
pub const LDGT_B: usize = 28912;                  
pub const LDGT_H: usize = 28913;                  
pub const LDGT_W: usize = 28914;                  
pub const LDGT_D: usize = 28915;                  
pub const LDLE_B: usize = 28916;                  
pub const LDLE_H: usize = 28917;                  
pub const LDLE_W: usize = 28918;                  
pub const LDLE_D: usize = 28919;                  
pub const STGT_B: usize = 28920;                  
pub const STGT_H: usize = 28921;                  
pub const STGT_W: usize = 28922;                  
pub const STGT_D: usize = 28923;                  
pub const STLE_B: usize = 28924;                  
pub const STLE_H: usize = 28925;                  
pub const STLE_W: usize = 28926;                  
pub const STLE_D: usize = 28927;                  

// 4R
pub const FMADD_S: usize = 129;              
pub const FMADD_D: usize = 130;              
pub const FMSUB_S: usize = 133;              
pub const FMSUB_D: usize = 134;              
pub const FNMADD_S: usize = 137;              
pub const FNMADD_D: usize = 138;              
pub const FNMSUB_S: usize = 141;              
pub const FNMSUB_D: usize = 142;              
pub const FCMP_cond_S: usize = 193;              
pub const FCMP_cond_D: usize = 194;              
pub const FSEL: usize = 208;                

// 2RI8
pub const ALSL: usize = 1;  
pub const BYTEPICK_W: usize = 2; 
pub const BYTEPICK_D: usize = 3; 
pub const ALSL_D: usize = 11;                 
pub const SLLI: usize = 16; 
// pub const SLLI_W: usize = 16;                  
// pub const SLLI_D: usize = 16;                 
pub const SRLI: usize = 17;                 
/*
pub const SRAI_W: usize = 18;                  
pub const SRAI_D: usize = 18;                 
*/
pub const SRAI: usize = 18;
/*
pub const ROTRI_W: usize = 19;                  
pub const ROTRI_D: usize = 19;                 
*/
pub const ROTRI: usize = 19;
pub const LDDIR: usize = 400;               
pub const LDPTE: usize = 401;                  

// 2RI12
pub const BSTRINS_BSTRPICK_W: usize = 1;             
pub const BSTRINS_D: usize = 2;            
pub const BSTRPICK_D: usize = 3;            
pub const SLTI: usize = 8;           
pub const SLTUI: usize = 9;           
pub const ADDI_W: usize = 10;           
pub const ADDI_D: usize = 11;           
pub const LU52I_D: usize = 12;           
pub const ANDI: usize = 13;           
pub const ORI: usize = 14;           
pub const XORI: usize = 15;           
pub const CACOP: usize = 24;           
pub const LD_B: usize = 160;           
pub const LD_H: usize = 161;        
pub const LD_W: usize = 162;           
pub const LD_D: usize = 163;           
pub const ST_B: usize = 164;           
pub const ST_H: usize = 165;           
pub const ST_W: usize = 166;           
pub const ST_D: usize = 167;           
pub const LD_BU: usize = 168;           
pub const LD_HU: usize = 169;           
pub const LD_WU: usize = 170;           
pub const PRELD: usize = 171;           
pub const FLD_S: usize = 172;           
pub const FST_S: usize = 173;           
pub const FLD_D: usize = 174;           
pub const FST_D: usize =175;           

// 2RI14
/*
pub const CSRRD: usize = 4;            
pub const CSRWR: usize = 4;            
pub const CSRXCHG: usize = 4;
*/
pub const CSR: usize = 4;
pub const LL_W: usize = 32;         
pub const SC_W: usize = 33;         
pub const LL_D: usize = 34;         
pub const SC_D: usize = 35;         
pub const LDPTR_W: usize = 36;         
pub const STPTR_W: usize = 37;         
pub const LDPTR_D: usize = 38;         
pub const STPTR_D: usize = 39;         

// 2RI16
pub const ADDU16I_D: usize = 4;
pub const JIRL: usize = 19;       
pub const BEQ: usize = 22;       
pub const BNE: usize = 23;       
pub const BLT: usize = 24;       
pub const BGE: usize = 25;       
pub const BLTU: usize = 26;       
pub const BGEU: usize = 27;       

// 1RI21
pub const BEQZ: usize = 16;      
pub const BNEZ: usize = 17;      
/*
pub const BCEQZ: usize = 18;       
pub const BCNEZ: usize = 18;       
*/
pub const BC: usize = 18;


// I26
pub const B: usize = 20;     
pub const BL: usize = 21;     


// 1RI20
pub const LU12I_W: usize = 10;       
pub const LU32I_D: usize = 11;       
pub const PCADDI: usize = 12;       
pub const PCALAU12I: usize = 13;       
pub const PCADDU12I: usize = 14;       
pub const PCADDU18I: usize = 15;       


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_type, non_camel_case_types)]
pub enum Opcode {
	CLO_W,
	CLZ_W,
	CTO_W,
	CTZ_W,
	CLO_D,
	CLZ_D,
	CTO_D,
	CTZ_D,
	REVB_2H,
	REVB_4H,
	REVB_2W,
	REVB_D,
	REVH_2W,
	REVH_D,
	BITREV_4B,
	BITREV_8B,
	BITREV_W,
	BITREV_D,
	EXT_W_H,
	EXT_W_B,
	RDTIMEL_W,
	RDTIMEH_W,
	RDTIME_D,
	CPUCFG,
	ASRTLE_D,
	ASRTGT_D,
	ALSL_W,
	ALSL_WU,
	BYTEPICK_W,
	BYTEPICK_D,
	ADD_W,
	ADD_D,
	SUB_W,
	SUB_D,
	SLT,
	SLTU,
	MASKEQZ,
	MASKNEZ,
	NOR,
	AND,
	OR,
	XOR,
	ORN,
	ANDN,
	SLL_W,
	SRL_W,
	SRA_W,
	SLL_D,
	SRL_D,
	SRA_D,
	ROTR_W,
	ROTR_D,
	MUL_W,
	MULH_W,
	MULH_WU,
	MUL_D,
	MULH_D,
	MULH_DU,
	MULW_D_W,
	MULW_D_WU,
	DIV_W,
	MOD_W,
	DIV_WU,
	MOD_WU,
	DIV_D,
	MOD_D,
	DIV_DU,
	MOD_DU,
	CRC_W_B_W,
	CRC_W_H_W,
	CRC_W_W_W,
	CRC_W_D_W,
	CRCC_W_B_W,
	CRCC_W_H_W,
	CRCC_W_W_W,
	CRCC_W_D_W,
	BREAK,
	DBCL,
	SYSCALL,
	ALSL_D,
	SLLI_W,
	SLLI_D,
	SRLI_W,
	SRLI_D,
	SRAI_W,
	SRAI_D,
	ROTRI_W,
	ROTRI_D,
	BSTRINS_W,
	BSTRPICK_W,
	BSTRINS_D,
	BSTRPICK_D,
	FADD_S,
	FADD_D,
	FSUB_S,
	FSUB_D,
	FMUL_S,
	FMUL_D,
	FDIV_S,
	FDIV_D,
	FMAX_S,
	FMAX_D,
	FMIN_S,
	FMIN_D,
	FMAXA_S,
	FMAXA_D,
	FMINA_S,
	FMINA_D,
	FSCALEB_S,
	FSCALEB_D,
	FCOPYSIGN_S,
	FCOPYSIGN_D,
	FABS_S,
	FABS_D,
	FNEG_S,
	FNEG_D,
	FLOGB_S,
	FLOGB_D,
	FCLASS_S,
	FCLASS_D,
	FSQRT_S,
	FSQRT_D,
	FRECIP_S,
	FRECIP_D,
	FRSQRT_S,
	FRSQRT_D,
	FMOV_S,
	FMOV_D,
	MOVGR2FR_W,
	MOVGR2FR_D,
	MOVGR2FRH_W,
	MOVFR2GR_S,
	MOVFR2GR_D,
	MOVFRH2GR_S,
	MOVGRF2CSR,
	MOVFCSR2GR,
	MOVFR2CF,
	MOVCF2FR,
	MOVGR2CF,
	MOVCF2GR,
	FCVT_S_D,
	FCVT_D_S,
	FTINTRM_W_S,
	FTINTRM_W_D,
	FTINTRM_L_S,
	FTINTRM_L_D,
	FTINTRP_W_S,
	FTINTRP_W_D,
	FTINTRP_L_S,
	FTINTRP_L_D,
	FTINTRZ_W_S,
	FTINTRZ_W_D,
	FTINTRZ_L_S,
	FTINTRZ_L_D,
	FTINTRNE_W_S,
	FTINTRNE_W_D,
	FTINTRNE_L_S,
	FTINTRNE_L_D,
	FTINT_W_S,
	FTINT_W_D,
	FTINT_L_S,
	FTINT_L_D,
	FFINT_S_W,
	FFINT_S_L,
	FFINT_D_W,
	FFINT_D_L,
	FRINT_S,
	FRINT_D,
	SLTI,
	SLTUI,
	ADDI_W,
	ADDI_D,
	LU52I_D,
	ANDI,
	ORI,
	XORI,
	CSRRD,
	CSRWR,
	CSRXCHG,
	CACOP,
	LDDIR,
	LDPTE,
	IOCSRRD_B,
	IOCSRRD_H,
	IOCSRRD_W,
	IOCSRRD_D,
	IOCSRWR_B,
	IOCSRWR_H,
	IOCSRWR_W,
	IOCSRWR_D,
	TLBCLR,
	TLBFLUSH,
	TLBSRCH,
	TLBRD,
	TLBWR,
	TLBFILL,
	ERTN,
	IDLE,
	INVTLB,
	FMADD_S,
	FMADD_D,
	FMSUB_S,
	FMSUB_D,
	FNMADD_S,
	FNMADD_D,
	FNMSUB_S,
	FNMSUB_D,
	//FCMP_cond_S,
	//FCMP_cond_D,
    FCMP_CAF_S,
    FCMP_CUN_S,
    FCMP_CEQ_S,
    FCMP_CUEQ_S,
    FCMP_CLT_S,
    FCMP_CULT_S,
    FCMP_CLE_S,
    FCMP_CULE_S,
    FCMP_CNE_S,
    FCMP_COR_S,
    FCMP_CUNE_S,
    FCMP_SAF_S,
    FCMP_SUN_S,
    FCMP_SEQ_S,
    FCMP_SUEQ_S,
    FCMP_SLT_S,
    FCMP_SULT_S,
    FCMP_SLE_S,
    FCMP_SULE_S,
    FCMP_SNE_S,
    FCMP_SOR_S,
    FCMP_SUNE_S,
    FCMP_CAF_D,
    FCMP_CUN_D,
    FCMP_CEQ_D,
    FCMP_CUEQ_D,
    FCMP_CLT_D,
    FCMP_CULT_D,
    FCMP_CLE_D,
    FCMP_CULE_D,
    FCMP_CNE_D,
    FCMP_COR_D,
    FCMP_CUNE_D,
    FCMP_SAF_D,
    FCMP_SUN_D,
    FCMP_SEQ_D,
    FCMP_SUEQ_D,
    FCMP_SLT_D,
    FCMP_SULT_D,
    FCMP_SLE_D,
    FCMP_SULE_D,
    FCMP_SNE_D,
    FCMP_SOR_D,
    FCMP_SUNE_D,
	FSEL,
	ADDU16I_D,
	LU12I_W,
	LU32I_D,
	PCADDI,
	PCALAU12I,
	PCADDU12I,
	PCADDU18I,
	LL_W,
	SC_W,
	LL_D,
	SC_D,
	LDPTR_W,
	STPTR_W,
	LDPTR_D,
	STPTR_D,
	LD_B,
	LD_H,
	LD_W,
	LD_D,
	ST_B,
	ST_H,
	ST_W,
	ST_D,
	LD_BU,
	LD_HU,
	LD_WU,
	PRELD,
	FLD_S,
	FST_S,
	FLD_D,
	FST_D,
	LDX_B,
	LDX_H,
	LDX_W,
	LDX_D,
	STX_B,
	STX_H,
	STX_W,
	STX_D,
	LDX_BU,
	LDX_HU,
	LDX_WU,
	PRELDX,
	FLDX_S,
	FLDX_D,
	FSTX_S,
	FSTX_D,
	AMSWAP_W,
	AMSWAP_D,
	AMADD_W,
	AMADD_D,
	AMAND_W,
	AMAND_D,
	AMOR_W,
	AMOR_D,
	AMXOR_W,
	AMXOR_D,
	AMMAX_W,
	AMMAX_D,
	AMMIN_W,
	AMMIN_D,
	AMMAX_WU,
	AMMAX_DU,
	AMMIN_WU,
	AMMIN_DU,
	AMSWAP_DB_W,
	AMSWAP_DB_D,
	AMADD_DB_W,
	AMADD_DB_D,
	AMAND_DB_W,
	AMAND_DB_D,
	AMOR_DB_W,
	AMOR_DB_D,
	AMXOR_DB_W,
	AMXOR_DB_D,
	AMMAX_DB_W,
	AMMAX_DB_D,
	AMMIN_DB_W,
	AMMIN_DB_D,
	AMMAX_DB_WU,
	AMMAX_DB_DU,
	AMMIN_DB_WU,
	AMMIN_DB_DU,
	DBAR,
	IBAR,
	FLDGT_S,
	FLDGT_D,
	FLDLE_S,
	FLDLE_D,
	FSTGT_S,
	FSTGT_D,
	FSTLE_S,
	FSTLE_D,
	LDGT_B,
	LDGT_H,
	LDGT_W,
	LDGT_D,
	LDLE_B,
	LDLE_H,
	LDLE_W,
	LDLE_D,
	STGT_B,
	STGT_H,
	STGT_W,
	STGT_D,
	STLE_B,
	STLE_H,
	STLE_W,
	STLE_D,
	BEQZ,
	BNEZ,
	BCEQZ,
	BCNEZ,
	JIRL,
	B,
	BL,
	BEQ,
	BNE,
	BLT,
	BGE,
	BLTU,
	BGEU,
}


impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::CLO_W => write!(f, "clo.w"),
            Opcode::CLZ_W => write!(f, "clz.w"),
            Opcode::CTO_W => write!(f, "cto.w"),
            Opcode::CTZ_W => write!(f, "ctz.w"),
            Opcode::CLO_D => write!(f, "clo.d"),
            Opcode::CLZ_D => write!(f, "clz.d"),
            Opcode::CTO_D => write!(f, "cto.d"),
            Opcode::CTZ_D => write!(f, "ctz.d"),
            Opcode::REVB_2H => write!(f, "revb.2h"),
            Opcode::REVB_4H => write!(f, "revb.4h"),
            Opcode::REVB_2W => write!(f, "revb.2w"),
            Opcode::REVB_D => write!(f, "revb.d"),
            Opcode::REVH_2W => write!(f, "revh.2w"),
            Opcode::REVH_D => write!(f, "revh.d"),
            Opcode::BITREV_4B => write!(f, "bitrev.4b"),
            Opcode::BITREV_8B => write!(f, "bitrev.8b"),
            Opcode::BITREV_W => write!(f, "bitrev.w"),
            Opcode::BITREV_D => write!(f, "bitrev.d"),
            Opcode::EXT_W_H => write!(f, "ext.w.h"),
            Opcode::EXT_W_B => write!(f, "ext.w.b"),
            Opcode::RDTIMEL_W => write!(f, "rdtimel.w"),
            Opcode::RDTIMEH_W => write!(f, "rdtimeh.w"),
            Opcode::RDTIME_D => write!(f, "rdtime.d"),
            Opcode::CPUCFG => write!(f, "cpucfg"),
            Opcode::ASRTLE_D => write!(f, "asrtle.d"),
            Opcode::ASRTGT_D => write!(f, "asrtgt.d"),
            Opcode::ALSL_W => write!(f, "alsl.w"),
            Opcode::ALSL_WU => write!(f, "alsl.wu"),
            Opcode::BYTEPICK_W => write!(f, "bytepick.w"),
            Opcode::BYTEPICK_D => write!(f, "bytepick.d"),
            Opcode::ADD_W => write!(f, "add.w"),
            Opcode::ADD_D => write!(f, "add.d"),
            Opcode::SUB_W => write!(f, "sub.w"),
            Opcode::SUB_D => write!(f, "sub.d"),
            Opcode::SLT => write!(f, "slt"),
            Opcode::SLTU => write!(f, "sltu"),
            Opcode::MASKEQZ => write!(f, "maskeqz"),
            Opcode::MASKNEZ => write!(f, "masknez"),
            Opcode::NOR => write!(f, "nor"),
            Opcode::AND => write!(f, "and"),
            Opcode::OR => write!(f, "or"),
            Opcode::XOR => write!(f, "xor"),
            Opcode::ORN => write!(f, "orn"),
            Opcode::ANDN => write!(f, "andn"),
            Opcode::SLL_W => write!(f, "sll.w"),
            Opcode::SRL_W => write!(f, "srl.w"),
            Opcode::SRA_W => write!(f, "sra.w"),
            Opcode::SLL_D => write!(f, "sll.d"),
            Opcode::SRL_D => write!(f, "srl.d"),
            Opcode::SRA_D => write!(f, "sra.d"),
            Opcode::ROTR_W => write!(f, "rotr.w"),
            Opcode::ROTR_D => write!(f, "rotr.d"),
            Opcode::MUL_W => write!(f, "mul.w"),
            Opcode::MULH_W => write!(f, "mulh.w"),
            Opcode::MULH_WU => write!(f, "mulh.wu"),
            Opcode::MUL_D => write!(f, "mul.d"),
            Opcode::MULH_D => write!(f, "mulh.d"),
            Opcode::MULH_DU => write!(f, "mulh.du"),
            Opcode::MULW_D_W => write!(f, "mulw.d.w"),
            Opcode::MULW_D_WU => write!(f, "mulw.d.wu"),
            Opcode::DIV_W => write!(f, "div.w"),
            Opcode::MOD_W => write!(f, "mod.w"),
            Opcode::DIV_WU => write!(f, "div.wu"),
            Opcode::MOD_WU => write!(f, "mod.wu"),
            Opcode::DIV_D => write!(f, "div.d"),
            Opcode::MOD_D => write!(f, "mod.d"),
            Opcode::DIV_DU => write!(f, "div.du"),
            Opcode::MOD_DU => write!(f, "mod.du"),
            Opcode::CRC_W_B_W => write!(f, "crc.w.b.w"),
            Opcode::CRC_W_H_W => write!(f, "crc.w.h.w"),
            Opcode::CRC_W_W_W => write!(f, "crc.w.w.w"),
            Opcode::CRC_W_D_W => write!(f, "crc.w.d.w"),
            Opcode::CRCC_W_B_W => write!(f, "crc.w.b.w"),
            Opcode::CRCC_W_H_W => write!(f, "crcc.w.h.w"),
            Opcode::CRCC_W_W_W => write!(f, "crcc.w.w.w"),
            Opcode::CRCC_W_D_W => write!(f, "crcc.w.d.w"),
            Opcode::BREAK => write!(f, "break"),
            Opcode::DBCL => write!(f, "dbcl"),
            Opcode::SYSCALL => write!(f, "syscall"),
            Opcode::ALSL_D => write!(f, "alsl.d"),
            Opcode::SLLI_W => write!(f, "slli.w"),
            Opcode::SLLI_D => write!(f, "slli.d"),
            Opcode::SRLI_W => write!(f, "srli.w"),
            Opcode::SRLI_D => write!(f, "srli.d"),
            Opcode::SRAI_W => write!(f, "srai.w"),
            Opcode::SRAI_D => write!(f, "srai.d"),
            Opcode::ROTRI_W => write!(f, "rotri.w"),
            Opcode::ROTRI_D => write!(f, "rotri.d"),
            Opcode::BSTRINS_W => write!(f, "bstrins.w"),
            Opcode::BSTRPICK_W => write!(f, "bstrpick.w"),
            Opcode::BSTRINS_D => write!(f, "bstrins.d"),
            Opcode::BSTRPICK_D => write!(f, "bstrpick.d"),
            Opcode::FADD_S => write!(f, "fadd.s"),
            Opcode::FADD_D => write!(f, "fadd.d"),
            Opcode::FSUB_S => write!(f, "fsub.s"),
            Opcode::FSUB_D => write!(f, "fsub.d"),
            Opcode::FMUL_S => write!(f, "fmul.s"),
            Opcode::FMUL_D => write!(f, "fmul.d"),
            Opcode::FDIV_S => write!(f, "fdiv.s"),
            Opcode::FDIV_D => write!(f, "fdiv.d"),
            Opcode::FMAX_S => write!(f, "fmax.s"),
            Opcode::FMAX_D => write!(f, "fmax.d"),
            Opcode::FMIN_S => write!(f, "fmin.s"),
            Opcode::FMIN_D => write!(f, "fmin.d"),
            Opcode::FMAXA_S => write!(f, "fmaxa.s"),
            Opcode::FMAXA_D => write!(f, "fmamx.d"),
            Opcode::FMINA_S => write!(f, "fmina.s"),
            Opcode::FMINA_D => write!(f, "fmina.d"),
            Opcode::FSCALEB_S => write!(f, "fscaleb.s"),
            Opcode::FSCALEB_D => write!(f, "fscaleb.d"),
            Opcode::FCOPYSIGN_S => write!(f, "fcopysign.s"),
            Opcode::FCOPYSIGN_D => write!(f, "fcopysign.d"),
            Opcode::FABS_S => write!(f, "fabs.s"),
            Opcode::FABS_D => write!(f, "fabs.d"),
            Opcode::FNEG_S => write!(f, "fneg.s"),
            Opcode::FNEG_D => write!(f, "fneg.d"),
            Opcode::FLOGB_S => write!(f, "flogb.s"),
            Opcode::FLOGB_D => write!(f, "flogb.d"),
            Opcode::FCLASS_S => write!(f, "fclass.s"),
            Opcode::FCLASS_D => write!(f, "fclass.d"),
            Opcode::FSQRT_S => write!(f, "fsqrt.s"),
            Opcode::FSQRT_D => write!(f, "fsqrt.d"),
            Opcode::FRECIP_S => write!(f, "frecip.s"),
            Opcode::FRECIP_D => write!(f, "frecip.d"),
            Opcode::FRSQRT_S => write!(f, "frsqrt.s"),
            Opcode::FRSQRT_D => write!(f, "frsqrt.d"),
            Opcode::FMOV_S => write!(f, "fmov.s"),
            Opcode::FMOV_D => write!(f, "fmov.d"),
            Opcode::MOVGR2FR_W => write!(f, "movgr2fr.w"),
            Opcode::MOVGR2FR_D => write!(f, "movge2fr.d"),
            Opcode::MOVGR2FRH_W => write!(f, "movgr2frh.w"),
            Opcode::MOVFR2GR_S => write!(f, "movfr2gr.s"),
            Opcode::MOVFR2GR_D => write!(f, "movfr2gr.d"),
            Opcode::MOVFRH2GR_S => write!(f, "movfrh2gr.s"),
            Opcode::MOVGRF2CSR => write!(f, "movgrf2csr"),
            Opcode::MOVFCSR2GR => write!(f, "movfcsr2gr"),
            Opcode::MOVFR2CF => write!(f, "movfr2cf"),
            Opcode::MOVCF2FR => write!(f, "movcf2fr"),
            Opcode::MOVGR2CF => write!(f, "movgr2cf"),
            Opcode::MOVCF2GR => write!(f, "movcf2gr"),
            Opcode::FCVT_S_D => write!(f, "fcvt.s.d"),
            Opcode::FCVT_D_S => write!(f, "fcvt.d.s"),
            Opcode::FTINTRM_W_S => write!(f, "ftintrm.w.s"),
            Opcode::FTINTRM_W_D => write!(f, "ftintrm.w.d"),
            Opcode::FTINTRM_L_S => write!(f, "ftintrm.l.s"),
            Opcode::FTINTRM_L_D => write!(f, "ftintrm.l.d"),
            Opcode::FTINTRP_W_S => write!(f, "ftintrp.w.s"),
            Opcode::FTINTRP_W_D => write!(f, "ftintrp.w.d"),
            Opcode::FTINTRP_L_S => write!(f, "ftintrp.l.s"),
            Opcode::FTINTRP_L_D => write!(f, "ftintrp.l.d"),
            Opcode::FTINTRZ_W_S => write!(f, "ftintrz.w.s"),
            Opcode::FTINTRZ_W_D => write!(f, "ftintrz.w.d"),
            Opcode::FTINTRZ_L_S => write!(f, "ftintrz.l.s"),
            Opcode::FTINTRZ_L_D => write!(f, "ftintrz.l.d"),
            Opcode::FTINTRNE_W_S => write!(f, "ftintrne.w.s"),
            Opcode::FTINTRNE_W_D => write!(f, "ftintrne.w.d"),
            Opcode::FTINTRNE_L_S => write!(f, "ftintrne.l.s"),
            Opcode::FTINTRNE_L_D => write!(f, "ftintrne.l.d"),
            Opcode::FTINT_W_S => write!(f, "ftint.w.s"),
            Opcode::FTINT_W_D => write!(f, "ftint.w.d"),
            Opcode::FTINT_L_S => write!(f, "ftint.l.s"),
            Opcode::FTINT_L_D => write!(f, "ftint.l.d"),
            Opcode::FFINT_S_W => write!(f, "ffint.s.w"),
            Opcode::FFINT_S_L => write!(f, "ffint.s.l"),
            Opcode::FFINT_D_W => write!(f, "ffint.d.w"),
            Opcode::FFINT_D_L => write!(f, "ffint.d.l"),
            Opcode::FRINT_S => write!(f, "frint.s"),
            Opcode::FRINT_D => write!(f, "frint.d"),
            Opcode::SLTI => write!(f, "slti"),
            Opcode::SLTUI => write!(f, "sltui"),
            Opcode::ADDI_W => write!(f, "addi.w"),
            Opcode::ADDI_D => write!(f, "addi.d"),
            Opcode::LU52I_D => write!(f, "lu52i.d"),
            Opcode::ANDI => write!(f, "andi"),
            Opcode::ORI => write!(f, "ori"),
            Opcode::XORI => write!(f, "xori"),
            Opcode::CSRRD => write!(f, "csrrd"),
            Opcode::CSRWR => write!(f, "csrwr"),
            Opcode::CSRXCHG => write!(f, "csrxchg"),
            Opcode::CACOP => write!(f, "cacop"),
            Opcode::LDDIR => write!(f, "lddir"),
            Opcode::LDPTE => write!(f, "ldpte"),
            Opcode::IOCSRRD_B => write!(f, "iocsrrd.b"),
            Opcode::IOCSRRD_H => write!(f, "iocsrrd.h"),
            Opcode::IOCSRRD_W => write!(f, "iocsrrd.h"),
            Opcode::IOCSRRD_D => write!(f, "iocsrrd.d"),
            Opcode::IOCSRWR_B => write!(f, "iocsrer.b"),
            Opcode::IOCSRWR_H => write!(f, "iocsrwr.h"),
            Opcode::IOCSRWR_W => write!(f, "iocsrwr.w"),
            Opcode::IOCSRWR_D => write!(f, "iocsrwr.d"),
            Opcode::TLBCLR => write!(f, "tlbclr"),
            Opcode::TLBFLUSH => write!(f, "tlbflush"),
            Opcode::TLBSRCH => write!(f, "tlbsrch"),
            Opcode::TLBRD => write!(f, "tlbrd"),
            Opcode::TLBWR => write!(f, "tlbwr"),
            Opcode::TLBFILL => write!(f, "tlbfill"),
            Opcode::ERTN => write!(f, "ertn"),
            Opcode::IDLE => write!(f, "idle"),
            Opcode::INVTLB => write!(f, "invtlb"),
            Opcode::FMADD_S => write!(f, "fmadd.s"),
            Opcode::FMADD_D => write!(f, "fmadd.d"),
            Opcode::FMSUB_S => write!(f, "fmsub.s"),
            Opcode::FMSUB_D => write!(f, "fmsub.d"),
            Opcode::FNMADD_S => write!(f, "fnmadd.s"),
            Opcode::FNMADD_D => write!(f, "fnmadd.d"),
            Opcode::FNMSUB_S => write!(f, "fnmsub.s"),
            Opcode::FNMSUB_D => write!(f, "fnmsub.d"),
            /*
            Opcode::FCMP_cond_S => write!(f, "fcmp.cond.s"),
            Opcode::FCMP_cond_D => write!(f, "fcmp.cond.d"),
            */
            Opcode::FSEL => write!(f, "fsel"),
            Opcode::ADDU16I_D => write!(f, "addu16i.d"),
            Opcode::LU12I_W => write!(f, "lu12i.w"),
            Opcode::LU32I_D => write!(f, "lu32i.d"),
            Opcode::PCADDI => write!(f, "pcaddi"),
            Opcode::PCALAU12I => write!(f, "pcalau12i"),
            Opcode::PCADDU12I => write!(f, "pcaddu12i"),
            Opcode::PCADDU18I => write!(f, "pcaddu18i"),
            Opcode::LL_W => write!(f, "ll.w"),
            Opcode::SC_W => write!(f, "sc.w"),
            Opcode::LL_D => write!(f, "ll.d"),
            Opcode::SC_D => write!(f, "sc.d"),
            Opcode::LDPTR_W => write!(f, "ldptr.w"),
            Opcode::STPTR_W => write!(f, "stptr.w"),
            Opcode::LDPTR_D => write!(f, "ldptr.d"),
            Opcode::STPTR_D => write!(f, "stptr.d"),
            Opcode::LD_B => write!(f, "ld.b"),
            Opcode::LD_H => write!(f, "ld.h"),
            Opcode::LD_W => write!(f, "ld.w"),
            Opcode::LD_D => write!(f, "ld.d"),
            Opcode::ST_B => write!(f, "st.b"),
            Opcode::ST_H => write!(f, "st.h"),
            Opcode::ST_W => write!(f, "st.w"),
            Opcode::ST_D => write!(f, "st.d"),
            Opcode::LD_BU => write!(f, "ld.bu"),
            Opcode::LD_HU => write!(f, "ld.hu"),
            Opcode::LD_WU => write!(f, "ld.wu"),
            Opcode::PRELD => write!(f, "preld"),
            Opcode::FLD_S => write!(f, "fld.s"),
            Opcode::FST_S => write!(f, "fst.s"),
            Opcode::FLD_D => write!(f, "fld.s"),
            Opcode::FST_D => write!(f, "fst.d"),
            Opcode::LDX_B => write!(f, "ldx.b"),
            Opcode::LDX_H => write!(f, "ldx.h"),
            Opcode::LDX_W => write!(f, "ldx.w"),
            Opcode::LDX_D => write!(f, "ldx.d"),
            Opcode::STX_B => write!(f, "stx.b"),
            Opcode::STX_H => write!(f, "stx.h"),
            Opcode::STX_W => write!(f, "stx.w"),
            Opcode::STX_D => write!(f, "stx.d"),
            Opcode::LDX_BU => write!(f, "ldx.bu"),
            Opcode::LDX_HU => write!(f, "ldx.hu"),
            Opcode::LDX_WU => write!(f, "ldx.wu"),
            Opcode::PRELDX => write!(f, "preldx"),
            Opcode::FLDX_S => write!(f, "fldx.s"),
            Opcode::FLDX_D => write!(f, "fldx.d"),
            Opcode::FSTX_S => write!(f, "fstx.s"),
            Opcode::FSTX_D => write!(f, "fstx.d"),
            Opcode::AMSWAP_W => write!(f, "amswap.w"),
            Opcode::AMSWAP_D => write!(f, "amswap.d"),
            Opcode::AMADD_W => write!(f, "amadd.w"),
            Opcode::AMADD_D => write!(f, "amadd.d"),
            Opcode::AMAND_W => write!(f, "amand.w"),
            Opcode::AMAND_D => write!(f, "amand.d"),
            Opcode::AMOR_W => write!(f, "amor.w"),
            Opcode::AMOR_D => write!(f, "amor.d"),
            Opcode::AMXOR_W => write!(f, "amxor.w"),
            Opcode::AMXOR_D => write!(f, "amxor.d"),
            Opcode::AMMAX_W => write!(f, "ammax.w"),
            Opcode::AMMAX_D => write!(f, "ammax.d"),
            Opcode::AMMIN_W => write!(f, "ammin.w"),
            Opcode::AMMIN_D => write!(f, "ammin.d"),
            Opcode::AMMAX_WU => write!(f, "ammax.wu"),
            Opcode::AMMAX_DU => write!(f, "ammax.du"),
            Opcode::AMMIN_WU => write!(f, "ammin.wu"),
            Opcode::AMMIN_DU => write!(f, "ammin.du"),
            Opcode::AMSWAP_DB_W => write!(f, "amswap.db.w"),
            Opcode::AMSWAP_DB_D => write!(f, "amswap.db.d"),
            Opcode::AMADD_DB_W => write!(f, "amadd.db.w"),
            Opcode::AMADD_DB_D => write!(f, "amadd.db.d"),
            Opcode::AMAND_DB_W => write!(f, "amand.db.w"),
            Opcode::AMAND_DB_D => write!(f, "amand.db.d"),
            Opcode::AMOR_DB_W => write!(f, "amor.db.w"),
            Opcode::AMOR_DB_D => write!(f, "amor.db.d"),
            Opcode::AMXOR_DB_W => write!(f, "amxor.db.w"),
            Opcode::AMXOR_DB_D => write!(f, "amxor.db.d"),
            Opcode::AMMAX_DB_W => write!(f, "ammax.db.w"),
            Opcode::AMMAX_DB_D => write!(f, "ammax.db.d"),
            Opcode::AMMIN_DB_W => write!(f, "ammin.db.w"),
            Opcode::AMMIN_DB_D => write!(f, "ammin.db.d"),
            Opcode::AMMAX_DB_WU => write!(f, "ammax.db.wu"),
            Opcode::AMMAX_DB_DU => write!(f, "ammax.db.du"),
            Opcode::AMMIN_DB_WU => write!(f, "ammin.db.wu"),
            Opcode::AMMIN_DB_DU => write!(f, "ammin.db.du"),
            Opcode::DBAR => write!(f, "dbar"),
            Opcode::IBAR => write!(f, "ibar"),
            Opcode::FLDGT_S => write!(f, "fldgt.s"),
            Opcode::FLDGT_D => write!(f, "fldgt.d"),
            Opcode::FLDLE_S => write!(f, "fldle.s"),
            Opcode::FLDLE_D => write!(f, "fldle.d"),
            Opcode::FSTGT_S => write!(f, "fstgt.s"),
            Opcode::FSTGT_D => write!(f, "fstgt.d"),
            Opcode::FSTLE_S => write!(f, "fstle.s"),
            Opcode::FSTLE_D => write!(f, "fstle.d"),
            Opcode::LDGT_B => write!(f, "ldgt.b"),
            Opcode::LDGT_H => write!(f, "ldgt.h"),
            Opcode::LDGT_W => write!(f, "ldgt.w"),
            Opcode::LDGT_D => write!(f, "ldgt.d"),
            Opcode::LDLE_B => write!(f, "ldle.b"),
            Opcode::LDLE_H => write!(f, "ldle.h"),
            Opcode::LDLE_W => write!(f, "ldle.w"),
            Opcode::LDLE_D => write!(f, "ldle.d"),
            Opcode::STGT_B => write!(f, "stgt.b"),
            Opcode::STGT_H => write!(f, "stgt.h"),
            Opcode::STGT_W => write!(f, "stgt.w"),
            Opcode::STGT_D => write!(f, "stgt.d"),
            Opcode::STLE_B => write!(f, "stle.b"),
            Opcode::STLE_H => write!(f, "stle.h"),
            Opcode::STLE_W => write!(f, "stle.w"),
            Opcode::STLE_D => write!(f, "stle.d"),
            Opcode::BEQZ => write!(f, "beqz"),
            Opcode::BNEZ => write!(f, "bnez"),
            Opcode::BCEQZ => write!(f, "bceqz"),
            Opcode::BCNEZ => write!(f, "bcnez"),
            Opcode::JIRL => write!(f, "jirl"),
            Opcode::B => write!(f, "b"),
            Opcode::BL => write!(f, "bl"),
            Opcode::BEQ => write!(f, "beq"),
            Opcode::BNE => write!(f, "bne"),
            Opcode::BLT => write!(f, "blt"),
            Opcode::BGE => write!(f, "bge"),
            Opcode::BLTU => write!(f, "bltu"),
            Opcode::BGEU => write!(f, "bgeu"),
			Opcode::FCMP_CAF_S => write!(f, "fcmp.caf.s"),
			Opcode::FCMP_CUN_S => write!(f, "fcmp.cun.s"),
			Opcode::FCMP_CEQ_S => write!(f, "fcmp.ceq.s"),
			Opcode::FCMP_CUEQ_S => write!(f, "fcmp.cueq.s"),
			Opcode::FCMP_CLT_S => write!(f, "fcmp.clt.s"),
			Opcode::FCMP_CULT_S => write!(f, "fcmp.cult.s"),
			Opcode::FCMP_CLE_S => write!(f, "fcmp.cle.s"),
			Opcode::FCMP_CULE_S => write!(f, "fcmp.cule.s"),
			Opcode::FCMP_CNE_S => write!(f, "fcmp.cne.s"),
			Opcode::FCMP_COR_S => write!(f, "fcmp.cor.s"),
			Opcode::FCMP_CUNE_S => write!(f, "fcmp.cune.s"),
			Opcode::FCMP_SAF_S => write!(f, "fcmp.saf.s"),
			Opcode::FCMP_SUN_S => write!(f, "fcmp.sun.s"),
			Opcode::FCMP_SEQ_S => write!(f, "fcmp.seq.s"),
			Opcode::FCMP_SUEQ_S => write!(f, "fcmp.sueq.s"),
			Opcode::FCMP_SLT_S => write!(f, "fcmp.slt.s"),
			Opcode::FCMP_SULT_S => write!(f, "fcmp.sult.s"),
			Opcode::FCMP_SLE_S => write!(f, "fcmp.sle.s"),
			Opcode::FCMP_SULE_S => write!(f, "fcmp.sule.s"),
			Opcode::FCMP_SNE_S => write!(f, "fcmp.sne.s"),
			Opcode::FCMP_SOR_S => write!(f, "fcmp.sor.s"),
			Opcode::FCMP_SUNE_S => write!(f, "fcmp.sune.s"),
			Opcode::FCMP_CAF_D => write!(f, "fcmp.caf.d"),
			Opcode::FCMP_CUN_D => write!(f, "fcmp.cun.d"),
			Opcode::FCMP_CEQ_D => write!(f, "fcmp.ceq.d"),
			Opcode::FCMP_CUEQ_D => write!(f, "fcmp.cueq.d"),
			Opcode::FCMP_CLT_D => write!(f, "fcmp.clt.d"),
			Opcode::FCMP_CULT_D => write!(f, "fcmp.cult.d"),
			Opcode::FCMP_CLE_D => write!(f, "fcmp.cle.d"),
			Opcode::FCMP_CULE_D => write!(f, "fcmp.cule.d"),
			Opcode::FCMP_CNE_D => write!(f, "fcmp.cne.d"),
			Opcode::FCMP_COR_D => write!(f, "fcmp.cor.d"),
			Opcode::FCMP_CUNE_D => write!(f, "fcmp.cune.d"),
			Opcode::FCMP_SAF_D => write!(f, "fcmp.saf.d"),
			Opcode::FCMP_SUN_D => write!(f, "fcmp.sun.d"),
			Opcode::FCMP_SEQ_D => write!(f, "fcmp.seq.d"),
			Opcode::FCMP_SUEQ_D => write!(f, "fcmp.sueq.d"),
			Opcode::FCMP_SLT_D => write!(f, "fcmp.slt.d"),
			Opcode::FCMP_SULT_D => write!(f, "fcmp.sult.d"),
			Opcode::FCMP_SLE_D => write!(f, "fcmp.sle.d"),
			Opcode::FCMP_SULE_D => write!(f, "fcmp.sule.d"),
			Opcode::FCMP_SNE_D => write!(f, "fcmp.sne.d"),
			Opcode::FCMP_SOR_D => write!(f, "fcmp.sor.d"),
			Opcode::FCMP_SUNE_D => write!(f, "fcmp.sune.d"),
        }
    }
}

*/
