use std::fs::File;
use std::fmt;
use std::collections::HashMap;
use serde_json;
use serde::{Serialize, Deserialize};


mod opcode;
mod register;
pub use opcode::*;
pub use register::*;

use super::record::*;

mod clo_w;
mod clz_w;
mod cto_w;
mod ctz_w;
mod clo_d;
mod clz_d;
mod cto_d;
mod ctz_d;
mod revb_2h;
mod revb_4h;
mod revb_2w;
mod revb_d;
mod revh_2w;
mod revh_d;
mod bitrev_4b;
mod bitrev_8b;
mod bitrev_w;
mod bitrev_d;
mod ext_w_h;
mod ext_w_b;
mod rdtimel_w;
mod rdtimeh_w;
mod rdtime_d;
mod cpucfg;
mod asrtle_d;
mod asrtgt_d;
mod alsl_w;
mod alsl_wu;
mod bytepick_w;
mod bytepick_d;
mod add_w;
mod add_d;
mod sub_w;
mod sub_d;
mod slt;
mod sltu;
mod maskeqz;
mod masknez;
mod nor;
mod and;
mod or;
mod xor;
mod orn;
mod andn;
mod sll_w;
mod srl_w;
mod sra_w;
mod sll_d;
mod srl_d;
mod sra_d;
mod rotr_w;
mod rotr_d;
mod mul_w;
mod mulh_w;
mod mulh_wu;
mod mul_d;
mod mulh_d;
mod mulh_du;
mod mulw_d_w;
mod mulw_d_wu;
mod div_w;
mod mod_w;
mod div_wu;
mod mod_wu;
mod div_d;
mod mod_d;
mod div_du;
mod mod_du;
mod crc_w_b_w;
mod crc_w_h_w;
mod crc_w_w_w;
mod crc_w_d_w;
mod crcc_w_b_w;
mod crcc_w_h_w;
mod crcc_w_w_w;
mod crcc_w_d_w;
mod _break;
mod dbcl;
mod syscall;
mod alsl_d;
mod slli_w;
mod slli_d;
mod srli_w;
mod srli_d;
mod srai_w;
mod srai_d;
mod rotri_w;
mod rotri_d;
mod bstrins_w;
mod bstrpick_w;
mod bstrins_d;
mod bstrpick_d;
mod fadd_s;
mod fadd_d;
mod fsub_s;
mod fsub_d;
mod fmul_s;
mod fmul_d;
mod fdiv_s;
mod fdiv_d;
mod fmax_s;
mod fmax_d;
mod fmin_s;
mod fmin_d;
mod fmaxa_s;
mod fmaxa_d;
mod fmina_s;
mod fmina_d;
mod fscaleb_s;
mod fscaleb_d;
mod fcopysign_s;
mod fcopysign_d;
mod fabs_s;
mod fabs_d;
mod fneg_s;
mod fneg_d;
mod flogb_s;
mod flogb_d;
mod fclass_s;
mod fclass_d;
mod fsqrt_s;
mod fsqrt_d;
mod frecip_s;
mod frecip_d;
mod frsqrt_s;
mod frsqrt_d;
mod fmov_s;
mod fmov_d;
mod movgr2fr_w;
mod movgr2fr_d;
mod movgr2frh_w;
mod movfr2gr_s;
mod movfr2gr_d;
mod movfrh2gr_s;
mod movgr2fcsr;
mod movfcsr2gr;
mod movfr2cf;
mod movcf2fr;
mod movgr2cf;
mod movcf2gr;
mod fcvt_s_d;
mod fcvt_d_s;
mod ftintrm_w_s;
mod ftintrm_w_d;
mod ftintrm_l_s;
mod ftintrm_l_d;
mod ftintrp_w_s;
mod ftintrp_w_d;
mod ftintrp_l_s;
mod ftintrp_l_d;
mod ftintrz_w_s;
mod ftintrz_w_d;
mod ftintrz_l_s;
mod ftintrz_l_d;
mod ftintrne_w_s;
mod ftintrne_w_d;
mod ftintrne_l_s;
mod ftintrne_l_d;
mod ftint_w_s;
mod ftint_w_d;
mod ftint_l_s;
mod ftint_l_d;
mod ffint_s_w;
mod ffint_s_l;
mod ffint_d_w;
mod ffint_d_l;
mod frint_s;
mod frint_d;
mod slti;
mod sltui;
mod addi_w;
mod addi_d;
mod lu52i_d;
mod andi;
mod ori;
mod xori;
mod csrrd;
mod csrwr;
mod csrxchg;
mod cacop;
mod lddir;
mod ldpte;
mod iocsrrd_b;
mod iocsrrd_h;
mod iocsrrd_w;
mod iocsrrd_d;
mod iocsrwr_b;
mod iocsrwr_h;
mod iocsrwr_w;
mod iocsrwr_d;
mod tlbclr;
mod tlbflush;
mod tlbsrch;
mod tlbrd;
mod tlbwr;
mod tlbfill;
mod ertn;
mod idle;
mod invtlb;
mod fmadd_s;
mod fmadd_d;
mod fmsub_s;
mod fmsub_d;
mod fnmadd_s;
mod fnmadd_d;
mod fnmsub_s;
mod fnmsub_d;
mod fcmp_cond_s;
mod fcmp_cond_d;
mod fsel;
mod addu16i_d;
mod lu12i_w;
mod lu32i_d;
mod pcaddi;
mod pcalau12i;
mod pcaddu12i;
mod pcaddu18i;
mod ll_w;
mod sc_w;
mod ll_d;
mod sc_d;
mod ldptr_w;
mod stptr_w;
mod ldptr_d;
mod stptr_d;
mod ld_b;
mod ld_h;
mod ld_w;
mod ld_d;
mod st_b;
mod st_h;
mod st_w;
mod st_d;
mod ld_bu;
mod ld_hu;
mod ld_wu;
mod preld;
mod fld_s;
mod fst_s;
mod fld_d;
mod fst_d;
mod ldx_b;
mod ldx_h;
mod ldx_w;
mod ldx_d;
mod stx_b;
mod stx_h;
mod stx_w;
mod stx_d;
mod ldx_bu;
mod ldx_hu;
mod ldx_wu;
mod preldx;
mod fldx_s;
mod fldx_d;
mod fstx_s;
mod fstx_d;
mod amswap_w;
mod amswap_d;
mod amadd_w;
mod amadd_d;
mod amand_w;
mod amand_d;
mod amor_w;
mod amor_d;
mod amxor_w;
mod amxor_d;
mod ammax_w;
mod ammax_d;
mod ammin_w;
mod ammin_d;
mod ammax_wu;
mod ammax_du;
mod ammin_wu;
mod ammin_du;
mod amswap_db_w;
mod amswap_db_d;
mod amadd_db_w;
mod amadd_db_d;
mod amand_db_w;
mod amand_db_d;
mod amor_db_w;
mod amor_db_d;
mod amxor_db_w;
mod amxor_db_d;
mod ammax_db_w;
mod ammax_db_d;
mod ammin_db_w;
mod ammin_db_d;
mod ammax_db_wu;
mod ammax_db_du;
mod ammin_db_wu;
mod ammin_db_du;
mod dbar;
mod ibar;
mod fldgt_s;
mod fldgt_d;
mod fldle_s;
mod fldle_d;
mod fstgt_s;
mod fstgt_d;
mod fstle_s;
mod fstle_d;
mod ldgt_b;
mod ldgt_h;
mod ldgt_w;
mod ldgt_d;
mod ldle_b;
mod ldle_h;
mod ldle_w;
mod ldle_d;
mod stgt_b;
mod stgt_h;
mod stgt_w;
mod stgt_d;
mod stle_b;
mod stle_h;
mod stle_w;
mod stle_d;
mod beqz;
mod bnez;
mod bceqz;
mod bcnez;
mod jirl;
mod b;
mod bl;
mod beq;
mod bne;
mod blt;
mod bge;
mod bltu;
mod bgeu;
use clo_w::*;
use clz_w::*;
use cto_w::*;
use ctz_w::*;
use clo_d::*;
use clz_d::*;
use cto_d::*;
use ctz_d::*;
use revb_2h::*;
use revb_4h::*;
use revb_2w::*;
use revb_d::*;
use revh_2w::*;
use revh_d::*;
use bitrev_4b::*;
use bitrev_8b::*;
use bitrev_w::*;
use bitrev_d::*;
use ext_w_h::*;
use ext_w_b::*;
use rdtimel_w::*;
use rdtimeh_w::*;
use rdtime_d::*;
use cpucfg::*;
use asrtle_d::*;
use asrtgt_d::*;
use alsl_w::*;
use alsl_wu::*;
use bytepick_w::*;
use bytepick_d::*;
use add_w::*;
use add_d::*;
use sub_w::*;
use sub_d::*;
use slt::*;
use sltu::*;
use maskeqz::*;
use masknez::*;
use nor::*;
use and::*;
use or::*;
use xor::*;
use orn::*;
use andn::*;
use sll_w::*;
use srl_w::*;
use sra_w::*;
use sll_d::*;
use srl_d::*;
use sra_d::*;
use rotr_w::*;
use rotr_d::*;
use mul_w::*;
use mulh_w::*;
use mulh_wu::*;
use mul_d::*;
use mulh_d::*;
use mulh_du::*;
use mulw_d_w::*;
use mulw_d_wu::*;
use div_w::*;
use mod_w::*;
use div_wu::*;
use mod_wu::*;
use div_d::*;
use mod_d::*;
use div_du::*;
use mod_du::*;
use crc_w_b_w::*;
use crc_w_h_w::*;
use crc_w_w_w::*;
use crc_w_d_w::*;
use crcc_w_b_w::*;
use crcc_w_h_w::*;
use crcc_w_w_w::*;
use crcc_w_d_w::*;
use _break::*;
use dbcl::*;
use syscall::*;
use alsl_d::*;
use slli_w::*;
use slli_d::*;
use srli_w::*;
use srli_d::*;
use srai_w::*;
use srai_d::*;
use rotri_w::*;
use rotri_d::*;
use bstrins_w::*;
use bstrpick_w::*;
use bstrins_d::*;
use bstrpick_d::*;
use fadd_s::*;
use fadd_d::*;
use fsub_s::*;
use fsub_d::*;
use fmul_s::*;
use fmul_d::*;
use fdiv_s::*;
use fdiv_d::*;
use fmax_s::*;
use fmax_d::*;
use fmin_s::*;
use fmin_d::*;
use fmaxa_s::*;
use fmaxa_d::*;
use fmina_s::*;
use fmina_d::*;
use fscaleb_s::*;
use fscaleb_d::*;
use fcopysign_s::*;
use fcopysign_d::*;
use fabs_s::*;
use fabs_d::*;
use fneg_s::*;
use fneg_d::*;
use flogb_s::*;
use flogb_d::*;
use fclass_s::*;
use fclass_d::*;
use fsqrt_s::*;
use fsqrt_d::*;
use frecip_s::*;
use frecip_d::*;
use frsqrt_s::*;
use frsqrt_d::*;
use fmov_s::*;
use fmov_d::*;
use movgr2fr_w::*;
use movgr2fr_d::*;
use movgr2frh_w::*;
use movfr2gr_s::*;
use movfr2gr_d::*;
use movfrh2gr_s::*;
use movgr2fcsr::*;
use movfcsr2gr::*;
use movfr2cf::*;
use movcf2fr::*;
use movgr2cf::*;
use movcf2gr::*;
use fcvt_s_d::*;
use fcvt_d_s::*;
use ftintrm_w_s::*;
use ftintrm_w_d::*;
use ftintrm_l_s::*;
use ftintrm_l_d::*;
use ftintrp_w_s::*;
use ftintrp_w_d::*;
use ftintrp_l_s::*;
use ftintrp_l_d::*;
use ftintrz_w_s::*;
use ftintrz_w_d::*;
use ftintrz_l_s::*;
use ftintrz_l_d::*;
use ftintrne_w_s::*;
use ftintrne_w_d::*;
use ftintrne_l_s::*;
use ftintrne_l_d::*;
use ftint_w_s::*;
use ftint_w_d::*;
use ftint_l_s::*;
use ftint_l_d::*;
use ffint_s_w::*;
use ffint_s_l::*;
use ffint_d_w::*;
use ffint_d_l::*;
use frint_s::*;
use frint_d::*;
use slti::*;
use sltui::*;
use addi_w::*;
use addi_d::*;
use lu52i_d::*;
use andi::*;
use ori::*;
use xori::*;
use csrrd::*;
use csrwr::*;
use csrxchg::*;
use cacop::*;
use lddir::*;
use ldpte::*;
use iocsrrd_b::*;
use iocsrrd_h::*;
use iocsrrd_w::*;
use iocsrrd_d::*;
use iocsrwr_b::*;
use iocsrwr_h::*;
use iocsrwr_w::*;
use iocsrwr_d::*;
use tlbclr::*;
use tlbflush::*;
use tlbsrch::*;
use tlbrd::*;
use tlbwr::*;
use tlbfill::*;
use ertn::*;
use idle::*;
use invtlb::*;
use fmadd_s::*;
use fmadd_d::*;
use fmsub_s::*;
use fmsub_d::*;
use fnmadd_s::*;
use fnmadd_d::*;
use fnmsub_s::*;
use fnmsub_d::*;
use fcmp_cond_s::*;
use fcmp_cond_d::*;
use fsel::*;
use addu16i_d::*;
use lu12i_w::*;
use lu32i_d::*;
use pcaddi::*;
use pcalau12i::*;
use pcaddu12i::*;
use pcaddu18i::*;
use ll_w::*;
use sc_w::*;
use ll_d::*;
use sc_d::*;
use ldptr_w::*;
use stptr_w::*;
use ldptr_d::*;
use stptr_d::*;
use ld_b::*;
use ld_h::*;
use ld_w::*;
use ld_d::*;
use st_b::*;
use st_h::*;
use st_w::*;
use st_d::*;
use ld_bu::*;
use ld_hu::*;
use ld_wu::*;
use preld::*;
use fld_s::*;
use fst_s::*;
use fld_d::*;
use fst_d::*;
use ldx_b::*;
use ldx_h::*;
use ldx_w::*;
use ldx_d::*;
use stx_b::*;
use stx_h::*;
use stx_w::*;
use stx_d::*;
use ldx_bu::*;
use ldx_hu::*;
use ldx_wu::*;
use preldx::*;
use fldx_s::*;
use fldx_d::*;
use fstx_s::*;
use fstx_d::*;
use amswap_w::*;
use amswap_d::*;
use amadd_w::*;
use amadd_d::*;
use amand_w::*;
use amand_d::*;
use amor_w::*;
use amor_d::*;
use amxor_w::*;
use amxor_d::*;
use ammax_w::*;
use ammax_d::*;
use ammin_w::*;
use ammin_d::*;
use ammax_wu::*;
use ammax_du::*;
use ammin_wu::*;
use ammin_du::*;
use amswap_db_w::*;
use amswap_db_d::*;
use amadd_db_w::*;
use amadd_db_d::*;
use amand_db_w::*;
use amand_db_d::*;
use amor_db_w::*;
use amor_db_d::*;
use amxor_db_w::*;
use amxor_db_d::*;
use ammax_db_w::*;
use ammax_db_d::*;
use ammin_db_w::*;
use ammin_db_d::*;
use ammax_db_wu::*;
use ammax_db_du::*;
use ammin_db_wu::*;
use ammin_db_du::*;
use dbar::*;
use ibar::*;
use fldgt_s::*;
use fldgt_d::*;
use fldle_s::*;
use fldle_d::*;
use fstgt_s::*;
use fstgt_d::*;
use fstle_s::*;
use fstle_d::*;
use ldgt_b::*;
use ldgt_h::*;
use ldgt_w::*;
use ldgt_d::*;
use ldle_b::*;
use ldle_h::*;
use ldle_w::*;
use ldle_d::*;
use stgt_b::*;
use stgt_h::*;
use stgt_w::*;
use stgt_d::*;
use stle_b::*;
use stle_h::*;
use stle_w::*;
use stle_d::*;
use beqz::*;
use bnez::*;
use bceqz::*;
use bcnez::*;
use jirl::*;
use b::*;
use bl::*;
use beq::*;
use bne::*;
use blt::*;
use bge::*;
use bltu::*;
use bgeu::*;


#[derive(Debug, Clone)]
pub struct AssemblyInstruction {
    pub address: u64,
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

#[derive(Debug, Clone)]
pub struct Operand {
    pub operand_type: OperandType,
    pub value: u64,
    pub symbol: Option<SymbolRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperandType {
    GeneralRegister,
    FloatRegister,
    UnsignedImm,
    SignedImm,
    Offset,
}


impl AssemblyInstruction {
    pub fn new() -> Self {
        AssemblyInstruction {
            address: 0,
            label: None,
            bytes: [0; 4],
            opcode: Opcode::AND,
            operand1: None,
            operand2: None,
            operand3: None,
            operand4: None,
            regs_write: Vec::new(),
            regs_read: Vec::new(),
        }
    }
}


pub fn dissam(bytes: &[u8], base: u64, symbols: &mut HashMap<u64, SymbolRecord>) -> Result<Vec<AssemblyInstruction>, u8> {
    let mut instructions = Vec::<AssemblyInstruction>::new();
    
    let len = bytes.len();
    if len % 4 != 0 || len <= 0 {
        return Err(1);
    }

    let mut i = 0;
    while i < len as u64 {
        let code = u32::from_le_bytes(clone_into_array(&bytes[i as usize ..i as usize + 4]));
        let mut j = 0;    
        let address = base + i;
        match (code >> 26) as usize {
            ADDU16I_D => {
                j += 1; 
                instructions.push(addu16i_d(code, address, symbols));
            }
            JIRL => {
                j += 1; 
                instructions.push(jirl(code, address, symbols));
            }
            BEQ => {
                j += 1; 
                instructions.push(beq(code, address, symbols));
            }
            BNE => {
                j += 1; 
                instructions.push(bne(code, address, symbols));
            }
            BLT => {
                j += 1; 
                instructions.push(blt(code, address, symbols));
            }
            BGE => {
                j += 1; 
                instructions.push(bge(code, address, symbols));
            }
            BLTU => {
                j += 1; 
                instructions.push(bltu(code, address, symbols));
            }
            BGEU => {
                j += 1; 
                instructions.push(bgeu(code, address, symbols));
            }
            BEQZ => {
                j += 1; 
                instructions.push(beqz(code, address, symbols));
            }
            BNEZ => {
                j += 1; 
                instructions.push(bnez(code, address, symbols));
            }
            /*
            BCEQZ => {
                j += 1; 
                instructions.push(bceqz(code, address, symbols));
            }
            BCNEZ => {
                j += 1; 
                instructions.push(bcnez(code, address, symbols));
            }
            */
            BC => {
                if (code as usize >> 8) & 3 == 0 {
                    j += 1;
                    instructions.push(bceqz(code, address, symbols));
                } else if (code as usize >> 8) & 3 == 1 {
                    j += 1;
                    instructions.push(bcnez(code, address, symbols));
                }
            }
            B => {
                j += 1; 
                instructions.push(b(code, address, symbols));
            }
            BL => {
                j += 1; 
                instructions.push(bl(code, address, symbols));
            }
            _ => {}
        }

        match code as usize >> 25 {
            LU12I_W => {
                j += 1; 
                instructions.push(lu12i_w(code, address, symbols));
            }
            LU32I_D => {
                j += 1; 
                instructions.push(lu32i_d(code, address, symbols));
            }
            PCADDI => {
                j += 1; 
                instructions.push(pcaddi(code, address, symbols));
            }
            PCALAU12I => {
                j += 1; 
                instructions.push(pcalau12i(code, address, symbols));
            }
            PCADDU12I => {
                j += 1; 
                instructions.push(pcaddu12i(code, address, symbols));
            }
            PCADDU18I => {
                j += 1; 
                instructions.push(pcaddu18i(code, address, symbols));
            }
            _ => {}
        }

        match code as usize >> 24 {
            /*
            CSRRD => {
                j += 1; 
                instructions.push(csrrd(code, address, symbols));
            }
            CSRWR => {
                j += 1; 
                instructions.push(csrwr(code, address, symbols));
            }
            CSRXCHG => {
                j += 1; 
                instructions.push(csrxchg(code, address, symbols));
            }
            */
            CSR => {
                if (code >> 5) & ((1 << 5) - 1) == 0 {
                    j += 1;
                    instructions.push(csrrd(code, address, symbols));
                } else if (code >> 5) & ((1 << 5) - 1) == 1 {
                    j += 1;
                    instructions.push(csrwr(code, address, symbols));
                } else {
                    j += 1;
                    instructions.push(csrxchg(code, address, symbols));
                }
            }
            LL_W => {
                j += 1; 
                instructions.push(ll_w(code, address, symbols));
            }
            SC_W => {
                j += 1; 
                instructions.push(sc_w(code, address, symbols));
            }
            LL_D => {
                j += 1; 
                instructions.push(ll_d(code, address, symbols));
            }
            SC_D => {
                j += 1; 
                instructions.push(sc_d(code, address, symbols));
            }
            LDPTR_W => {
                j += 1; 
                instructions.push(ldptr_w(code, address, symbols));
            }
            STPTR_W => {
                j += 1; 
                instructions.push(stptr_w(code, address, symbols));
            }
            LDPTR_D => {
                j += 1; 
                instructions.push(ldptr_d(code, address, symbols));
            }
            STPTR_D => {
                j += 1; 
                instructions.push(stptr_d(code, address, symbols));
            }
            _ => {}
        }

        match code as usize >> 22 {
            BSTRINS_BSTRPICK_W => {
                j += 1;
                if ((code >> 15) & 1) == 0 && ((code >> 21) & 1) == 1 {
                    instructions.push(bstrins_w(code, address, symbols));
                   
                } else if((code >> 15) & 1) == 0 && ((code >> 21) & 1) == 1 {
                    instructions.push(bstrpick_w(code, address, symbols)); 
                   
                }
            }
            BSTRINS_D => {
                j += 1; 
                instructions.push(bstrins_d(code, address, symbols));
            }
            BSTRPICK_D => {
                j += 1; 
                instructions.push(bstrpick_d(code, address, symbols));
            }
            SLTI => {
                j += 1; 
                instructions.push(slti(code, address, symbols));
            }
            SLTUI => {
                j += 1; 
                instructions.push(sltui(code, address, symbols));
            }
            ADDI_W => {
                j+= 1; 
                instructions.push(addi_w(code, address, symbols));
            }
            ADDI_D => {
                j += 1; 
                instructions.push(addi_d(code, address, symbols));
            }
            LU52I_D => {
                j += 1; 
                instructions.push(lu52i_d(code, address, symbols));
            }
            ANDI => {
                j += 1; 
                instructions.push(andi(code, address, symbols));
            }
            ORI => {
                j += 1; 
                instructions.push(ori(code, address, symbols));
            }
            XORI => {
                j += 1; 
                instructions.push(xori(code, address, symbols));
            }
            CACOP => {
                j += 1; 
                instructions.push(cacop(code, address, symbols));
            }
            LD_B => {
                j += 1; 
                instructions.push(ld_b(code, address, symbols));
            }
            LD_H => {
                j += 1; 
                instructions.push(ld_h(code, address, symbols));
            }
            LD_W => {
                j += 1; 
                instructions.push(ld_w(code, address, symbols));
            }
            LD_D => {
                j += 1; 
                instructions.push(ld_d(code, address, symbols));
            }
            ST_B => {
                j += 1; 
                instructions.push(st_b(code, address, symbols));
            }
            ST_H => {
                j += 1; 
                instructions.push(st_h(code, address, symbols));
            }
            ST_W => {
                j += 1; 
                instructions.push(st_w(code, address, symbols));
            }
            ST_D => {
                j += 1; 
                instructions.push(st_d(code, address, symbols));
            }
            LD_BU => {
                j += 1; 
                instructions.push(ld_bu(code, address, symbols));
            }
            LD_HU => {
                j += 1; 
                instructions.push(ld_hu(code, address, symbols));
            }
            LD_WU => {
                j += 1; 
                instructions.push(ld_wu(code, address, symbols));
            }
            PRELD => {
                j += 1; 
                instructions.push(preld(code, address, symbols));
            }
            FLD_S => {
                j += 1; 
                instructions.push(fld_s(code, address, symbols));
            }
            FST_S => {
                j += 1; 
                instructions.push(fst_s(code, address, symbols));
            }
            FLD_D => {
                j += 1; 
                instructions.push(fld_d(code, address, symbols));
            }
            FST_D => {
                j += 1; 
                instructions.push(fst_d(code, address, symbols));
            }
            _ => {}
        }

        match code as usize >> 20 {
            FMADD_S => {
                j += 1; 
                instructions.push(fmadd_s(code, address, symbols));
            }
            FMADD_D => {
                j += 1; 
                instructions.push(fmadd_d(code, address, symbols));
            }
            FMSUB_S => {
                j += 1; 
                instructions.push(fmsub_s(code, address, symbols));
            }
            FMSUB_D => {
                j += 1; 
                instructions.push(fmsub_d(code, address, symbols));
            }
            FNMADD_S => {
                j += 1; 
                instructions.push(fnmadd_s(code, address, symbols));
            }
            FNMADD_D => {
                j += 1; 
                instructions.push(fnmadd_d(code, address, symbols));
            }
            FNMSUB_S => {
                j += 1; 
                instructions.push(fnmsub_s(code, address, symbols));
            }
            FNMSUB_D => {
                j += 1; 
                instructions.push(fnmsub_d(code, address, symbols));
            }
            FCMP_cond_S => {
                j += 1; 
                instructions.push(fcmp_cond_s(code, address, symbols));
            }
            FCMP_cond_D => {
                j += 1; 
                instructions.push(fcmp_cond_d(code, address, symbols));
            }
            FSEL => {
                j += 1; 
                instructions.push(fsel(code, address, symbols));
            }
            _ => {}
        }

        match code as usize >> 18 {
            /*
            ALSL_W => {j += 1; instructions.push(alsl_w(code, address, symbols));}
            ALSL_WU => {j += 1; instructions.push(alsl_wu(code, address, symbols));}
            */
            ALSL => {
                if (code & (1 << 16)) == 0 {
                    j += 1;
                    instructions.push(alsl_w(code, address, symbols));
                } else if (code & (1 << 16)) == 1 {
                    j += 1;
                    instructions.push(alsl_wu(code, address, symbols));
                }
            }
            BYTEPICK_W => {j += 1; instructions.push(bytepick_w(code, address, symbols));}
            BYTEPICK_D => {j += 1; instructions.push(bytepick_d(code, address, symbols));}
            ALSL_D => {j += 1; instructions.push(alsl_d(code, address, symbols));}
            /*
            SLLI_W => {j += 1; instructions.push(slli_w(code, address, symbols));}
            SLLI_D => {j += 1; instructions.push(slli_d(code, address, symbols));}
            */
            SLLI => {
                if (code as usize >> 15) & 7 == 1 {
                    j += 1;
                    instructions.push(slli_w(code, address, symbols));
                } else if (code as usize >> 16) & 3 == 1 {
                    j += 1;
                    instructions.push(slli_d(code, address, symbols));
                }
            }
            SRLI => {
                j += 1; 
                if (code >> 15) & ((1 << 3) - 1) == 1 {
                    instructions.push(srli_w(code, address, symbols));
                } else if (code >> 16) & ((1 << 2) - 1) == 1 {
                    instructions.push(srli_d(code, address, symbols));
                } 
               
            }
            /*
            SRAI_W => {j += 1; instructions.push(srai_w(code, address, symbols));}
            SRAI_D => {j += 1; instructions.push(srai_d(code, address, symbols));}
            */
            SRAI => {
                if (code as usize >> 15) & 7 == 1 {
                    j += 1;
                    instructions .push(srai_w(code, address, symbols));
                } else if (code as usize >> 16) & 3 == 1 {
                    j += 1;
                    instructions.push(srai_d(code, address, symbols));
                }
            }
            /*
            ROTRI_W => {j += 1; instructions.push(rotri_w(code, address, symbols));}
            ROTRI_D => {j += 1; instructions.push(rotri_d(code, address, symbols));}
            */
            ROTRI => {
                if (code as usize >> 15) & 7 == 1 {
                    j += 1;
                    instructions.push(rotri_w(code, address, symbols));
                } else if (code as usize >> 16) & 3 == 1 {
                    j += 1;
                    instructions.push(rotri_d(code, address, symbols));
                }
            }
            LDDIR => {j += 1; instructions.push(lddir(code, address, symbols));}
            LDPTE => {j += 1; instructions.push(ldpte(code, address, symbols));}
            _ => {}
        }

        match code as usize >> 15 {
            ASRTLE_D => {j += 1; instructions.push(asrtle_d(code, address, symbols));}
            ASRTGT_D => {j += 1; instructions.push(asrtgt_d(code, address, symbols));}
            ADD_W => {j += 1; instructions.push(add_w(code, address, symbols));}
            ADD_D => {j += 1; instructions.push(add_d(code, address, symbols));}
            SUB_W => {j += 1; instructions.push(sub_w(code, address, symbols));}
            SUB_D => {j += 1; instructions.push(sub_d(code, address, symbols));}
            SLT => {j += 1; instructions.push(slt(code, address, symbols));}
            SLTU => {j += 1; instructions.push(sltu(code, address, symbols));}
            MASKEQZ => {j += 1; instructions.push(maskeqz(code, address, symbols));}
            MASKNEZ => {j += 1; instructions.push(masknez(code, address, symbols));}
            NOR => {j += 1; instructions.push(nor(code, address, symbols));}
            AND => {j += 1; instructions.push(and(code, address, symbols));}
            OR => {j += 1; instructions.push(or(code, address, symbols));}
            XOR => {j += 1; instructions.push(xor(code, address, symbols));}
            ORN => {j += 1; instructions.push(orn(code, address, symbols));}
            ANDN => {j += 1; instructions.push(andn(code, address, symbols));}
            SLL_W => {j += 1; instructions.push(sll_w(code, address, symbols));}
            SRL_W => {j += 1; instructions.push(srl_w(code, address, symbols));}
            SRA_W => {j += 1; instructions.push(sra_w(code, address, symbols));}
            SLL_D => {j += 1; instructions.push(sll_d(code, address, symbols));}
            SRL_D => {j += 1; instructions.push(srl_d(code, address, symbols));}
            SRA_D => {j += 1; instructions.push(sra_d(code, address, symbols));}
            ROTR_W => {j += 1; instructions.push(rotr_w(code, address, symbols));} 
            ROTR_D => {j += 1; instructions.push(rotr_d(code, address, symbols));} 
            MUL_W => {j += 1; instructions.push(mul_w(code, address, symbols));}
            MULH_W => {j += 1; instructions.push(mulh_w(code, address, symbols));} 
            MULH_WU => {j += 1; instructions.push(mulh_wu(code, address, symbols));}
            MUL_D => {j += 1; instructions.push(mul_d(code, address, symbols));}
            MULH_D => {j += 1; instructions.push(mulh_d(code, address, symbols));} 
            MULH_DU => {j += 1; instructions.push(mulh_du(code, address, symbols));}
            MULW_D_W => {j += 1; instructions.push(mulw_d_w(code, address, symbols));}
            MULW_D_WU => {j += 1; instructions.push(mulw_d_wu(code, address, symbols));}
            DIV_W => {j += 1; instructions.push(div_w(code, address, symbols));}
            MOD_W => {j += 1; instructions.push(mod_w(code, address, symbols));}
            DIV_WU => {j += 1; instructions.push(div_wu(code, address, symbols));} 
            MOD_WU => {j += 1; instructions.push(mod_wu(code, address, symbols));} 
            DIV_D => {j += 1; instructions.push(div_d(code, address, symbols));}
            MOD_D => {j += 1; instructions.push(mod_d(code, address, symbols));}
            DIV_DU => {j += 1; instructions.push(div_du(code, address, symbols));} 
            MOD_DU => {j += 1; instructions.push(mod_du(code, address, symbols));} 
            CRC_W_B_W => {j += 1; instructions.push(crc_w_b_w(code, address, symbols));}
            CRC_W_H_W => {j += 1; instructions.push(crc_w_h_w(code, address, symbols));}
            CRC_W_W_W => {j += 1; instructions.push(crc_w_w_w(code, address, symbols));}
            CRC_W_D_W => {j += 1; instructions.push(crc_w_d_w(code, address, symbols));}
            CRCC_W_B_W => {j += 1; instructions.push(crcc_w_b_w(code, address, symbols));}
            CRCC_W_H_W => {j += 1; instructions.push(crcc_w_h_w(code, address, symbols));}
            CRCC_W_W_W => {j += 1; instructions.push(crcc_w_w_w(code, address, symbols));}
            CRCC_W_D_W => {j += 1; instructions.push(crcc_w_d_w(code, address, symbols));}
            BREAK => {j += 1; instructions.push(_break(code, address, symbols));}
            DBCL => {j += 1; instructions.push(dbcl(code, address, symbols));}
            SYSCALL => {j += 1; instructions.push(syscall(code, address, symbols));}
            FADD_S => {j += 1; instructions.push(fadd_s(code, address, symbols));}
            FADD_D => {j += 1; instructions.push(fadd_d(code, address, symbols));}
            FSUB_S => {j += 1; instructions.push(fsub_s(code, address, symbols));}
            FSUB_D => {j += 1; instructions.push(fsub_d(code, address, symbols));}
            FMUL_S => {j += 1; instructions.push(fmul_s(code, address, symbols));}
            FMUL_D => {j += 1; instructions.push(fmul_d(code, address, symbols));}
            FDIV_S => {j += 1; instructions.push(fdiv_s(code, address, symbols));}
            FDIV_D => {j += 1; instructions.push(fdiv_d(code, address, symbols));}
            FMAX_S => {j += 1; instructions.push(fmax_s(code, address, symbols));}
            FMAX_D => {j += 1; instructions.push(fmax_d(code, address, symbols));}
            FMIN_S => {j += 1; instructions.push(fmin_s(code, address, symbols));}
            FMIN_D => {j += 1; instructions.push(fmin_d(code, address, symbols));}
            FMAXA_S => {j += 1; instructions.push(fmaxa_s(code, address, symbols));}
            FMAXA_D => {j += 1; instructions.push(fmaxa_d(code, address, symbols));}
            FMINA_S => {j += 1; instructions.push(fmina_s(code, address, symbols));}
            FMINA_D => {j += 1; instructions.push(fmina_d(code, address, symbols));}
            FSCALEB_S => {j += 1; instructions.push(fscaleb_s(code, address, symbols));}
            FSCALEB_D => {j += 1; instructions.push(fscaleb_d(code, address, symbols));}
            FCOPYSIGN_S => {j += 1; instructions.push(fcopysign_s(code, address, symbols));}
            FCOPYSIGN_D => {j += 1; instructions.push(fcopysign_d(code, address, symbols));}
            IDLE => {j += 1; instructions.push(idle(code, address, symbols));}
            INVTLB => {j += 1; instructions.push(invtlb(code, address, symbols));}
            LDX_B => {j += 1; instructions.push(ldx_b(code, address, symbols));}
            LDX_H => {j += 1; instructions.push(ldx_h(code, address, symbols));}
            LDX_W => {j += 1; instructions.push(ldx_w(code, address, symbols));}
            LDX_D => {j += 1; instructions.push(ldx_d(code, address, symbols));}
            STX_B => {j += 1; instructions.push(stx_b(code, address, symbols));}
            STX_H => {j += 1; instructions.push(stx_h(code, address, symbols));}
            STX_W => {j += 1; instructions.push(stx_w(code, address, symbols));}
            STX_D => {j += 1; instructions.push(stx_d(code, address, symbols));}
            LDX_BU => {j += 1; instructions.push(ldx_bu(code, address, symbols));}
            LDX_HU => {j += 1; instructions.push(ldx_hu(code, address, symbols));}
            LDX_WU => {j += 1; instructions.push(ldx_wu(code, address, symbols));}
            PRELDX => {j += 1; instructions.push(preldx(code, address, symbols));}
            FLDX_S => {j += 1; instructions.push(fldx_s(code, address, symbols));} 
            FLDX_D => {j += 1; instructions.push(fldx_d(code, address, symbols));} 
            FSTX_S => {j += 1; instructions.push(fstx_s(code, address, symbols));} 
            FSTX_D => {j += 1; instructions.push(fstx_d(code, address, symbols));} 
            AMSWAP_W => {j += 1; instructions.push(amswap_w(code, address, symbols));}
            AMSWAP_D => {j += 1; instructions.push(amswap_d(code, address, symbols));}
            AMADD_W => {j += 1; instructions.push(amadd_w(code, address, symbols));} 
            AMADD_D => {j += 1; instructions.push(amadd_d(code, address, symbols));} 
            AMAND_W => {j += 1; instructions.push(amand_w(code, address, symbols));} 
            AMAND_D => {j += 1; instructions.push(amand_d(code, address, symbols));} 
            AMOR_W => {j += 1; instructions.push(amor_w(code, address, symbols));} 
            AMOR_D => {j += 1; instructions.push(amor_d(code, address, symbols));} 
            AMXOR_W => {j += 1; instructions.push(amxor_w(code, address, symbols));} 
            AMXOR_D => {j += 1; instructions.push(amxor_d(code, address, symbols));} 
            AMMAX_W => {j += 1; instructions.push(ammax_w(code, address, symbols));} 
            AMMAX_D => {j += 1; instructions.push(ammax_d(code, address, symbols));} 
            AMMIN_W => {j += 1; instructions.push(ammin_w(code, address, symbols));} 
            AMMIN_D => {j += 1; instructions.push(ammin_d(code, address, symbols));} 
            AMMAX_WU => {j += 1; instructions.push(ammax_wu(code, address, symbols));}
            AMMAX_DU => {j += 1; instructions.push(ammax_du(code, address, symbols));}
            AMMIN_WU => {j += 1; instructions.push(ammin_wu(code, address, symbols));}
            AMMIN_DU => {j += 1; instructions.push(ammin_du(code, address, symbols));}
            AMSWAP_DB_W => {j += 1; instructions.push(amswap_db_w(code, address, symbols));}
            AMSWAP_DB_D => {j += 1; instructions.push(amswap_db_d(code, address, symbols));}
            AMADD_DB_W => {j += 1; instructions.push(amadd_db_w(code, address, symbols));}
            AMADD_DB_D => {j += 1; instructions.push(amadd_db_d(code, address, symbols));}
            AMAND_DB_W => {j += 1; instructions.push(amand_db_w(code, address, symbols));}
            AMAND_DB_D => {j += 1; instructions.push(amand_db_d(code, address, symbols));}
            AMOR_DB_W => {j += 1; instructions.push(amor_db_w(code, address, symbols));}
            AMOR_DB_D => {j += 1; instructions.push(amor_db_d(code, address, symbols));}
            AMXOR_DB_W => {j += 1; instructions.push(amxor_db_w(code, address, symbols));}
            AMXOR_DB_D => {j += 1; instructions.push(amxor_db_d(code, address, symbols));}
            AMMAX_DB_W => {j += 1; instructions.push(ammax_db_w(code, address, symbols));}
            AMMAX_DB_D => {j += 1; instructions.push(ammax_db_d(code, address, symbols));}
            AMMIN_DB_W => {j += 1; instructions.push(ammin_db_w(code, address, symbols));}
            AMMIN_DB_D => {j += 1; instructions.push(ammin_db_d(code, address, symbols));}                  
            AMMAX_DB_WU => {j += 1; instructions.push(ammax_db_wu(code, address, symbols));}                   
            AMMAX_DB_DU => {j += 1; instructions.push(ammax_db_du(code, address, symbols));}                   
            AMMIN_DB_WU => {j += 1; instructions.push(ammin_db_wu(code, address, symbols));}                   
            AMMIN_DB_DU => {j += 1; instructions.push(ammin_db_du(code, address, symbols));}                  
            DBAR => {j += 1; instructions.push(dbar(code, address, symbols));}                
            IBAR => {j += 1; instructions.push(ibar(code, address, symbols));}                
            FLDGT_S => {j += 1; instructions.push(fldgt_s(code, address, symbols));}                  
            FLDGT_D => {j += 1; instructions.push(fldgt_d(code, address, symbols));}                  
            FLDLE_S => {j += 1; instructions.push(fldle_s(code, address, symbols));}                  
            FLDLE_D => {j += 1; instructions.push(fldle_d(code, address, symbols));}                  
            FSTGT_S => {j += 1; instructions.push(fstgt_s(code, address, symbols));}                  
            FSTGT_D => {j += 1; instructions.push(fstgt_d(code, address, symbols));}                  
            FSTLE_S => {j += 1; instructions.push(fstle_s(code, address, symbols));}                  
            FSTLE_D => {j += 1; instructions.push(fstle_d(code, address, symbols));}                  
            LDGT_B => {j += 1; instructions.push(ldgt_b(code, address, symbols));}                  
            LDGT_H => {j += 1; instructions.push(ldgt_h(code, address, symbols));}                  
            LDGT_W => {j += 1; instructions.push(ldgt_w(code, address, symbols));}                  
            LDGT_D => {j += 1; instructions.push(ldgt_d(code, address, symbols));}                  
            LDLE_B => {j += 1; instructions.push(ldle_b(code, address, symbols));}                  
            LDLE_H => {j += 1; instructions.push(ldle_h(code, address, symbols));}                  
            LDLE_W => {j += 1; instructions.push(ldle_w(code, address, symbols));}                  
            LDLE_D => {j += 1; instructions.push(ldle_d(code, address, symbols));}                  
            STGT_B => {j += 1; instructions.push(stgt_b(code, address, symbols));}                  
            STGT_H => {j += 1; instructions.push(stgt_h(code, address, symbols));}                  
            STGT_W => {j += 1; instructions.push(stgt_w(code, address, symbols));}                  
            STGT_D => {j += 1; instructions.push(stgt_d(code, address, symbols));}                  
            STLE_B => {j += 1; instructions.push(stle_b(code, address, symbols));}                  
            STLE_H => {j += 1; instructions.push(stle_h(code, address, symbols));}                  
            STLE_W => {j += 1; instructions.push(stle_w(code, address, symbols));}                  
            STLE_D => {j += 1; instructions.push(stle_d(code, address, symbols));} 
            _ => {}
        }

        match code as usize >> 10 {
            CLO_W => {j += 1; instructions.push(clo_w(code, address, symbols));}
            CLZ_W => {j += 1; instructions.push(clz_w(code, address, symbols));}
            CTO_W => {j += 1; instructions.push(cto_w(code, address, symbols));}
            CTZ_W => {j += 1; instructions.push(ctz_w(code, address, symbols));}
            CLO_D => {j += 1; instructions.push(clo_d(code, address, symbols));}
            CLZ_D => {j += 1; instructions.push(clz_d(code, address, symbols));}
            CTO_D => {j += 1; instructions.push(cto_d(code, address, symbols));}
            CTZ_D => {j += 1; instructions.push(ctz_d(code, address, symbols));}
            REVB_2H => {j += 1; instructions.push(revb_2h(code, address, symbols));}
            REVB_4H => {j += 1; instructions.push(revb_4h(code, address, symbols));}
            REVB_2W => {j += 1; instructions.push(revb_2w(code, address, symbols));}
            REVB_D => {j += 1; instructions.push(revb_d(code, address, symbols));}
            REVH_2W => {j += 1; instructions.push(revh_2w(code, address, symbols));}
            REVH_D => {j += 1; instructions.push(revh_d(code, address, symbols));}
            BITREV_4B => {j += 1; instructions.push(bitrev_4b(code, address, symbols));}
            BITREV_8B => {j += 1; instructions.push(bitrev_8b(code, address, symbols));}
            BITREV_W => {j += 1; instructions.push(bitrev_w(code, address, symbols));}
            BITREV_D => {j += 1; instructions.push(bitrev_d(code, address, symbols));}
            EXT_W_H => {j += 1; instructions.push(ext_w_h(code, address, symbols));}
            EXT_W_B => {j += 1; instructions.push(ext_w_b(code, address, symbols));}
            RDTIMEL_W => {j += 1; instructions.push(rdtimel_w(code, address, symbols));}
            RDTIMEH_W => {j += 1; instructions.push(rdtimeh_w(code, address, symbols));}
            RDTIME_D => {j += 1; instructions.push(rdtime_d(code, address, symbols));}
            CPUCFG => {j += 1; instructions.push(cpucfg(code, address, symbols));}
            FSQRT_S => {j += 1; instructions.push(fsqrt_s(code, address, symbols));}
            FSQRT_D => {j += 1; instructions.push(fsqrt_d(code, address, symbols));}
            FRECIP_S => {j += 1; instructions.push(frecip_s(code, address, symbols));}
            FRECIP_D => {j += 1; instructions.push(frecip_d(code, address, symbols));} 
            FRSQRT_S => {j += 1; instructions.push(frsqrt_s(code, address, symbols));} 
            FRSQRT_D => {j += 1; instructions.push(frsqrt_d(code, address, symbols));} 
            FMOV_S => {j += 1; instructions.push(fmov_s(code, address, symbols));}
            FMOV_D => {j += 1; instructions.push(fmov_d(code, address, symbols));}
            MOVGR2FR_W => {j += 1; instructions.push(movgr2fr_w(code, address, symbols));}
            MOVGR2FR_D => {j += 1; instructions.push(movgr2fr_d(code, address, symbols));}
            MOVGR2FRH_W => {j += 1; instructions.push(movgr2frh_w(code, address, symbols));} 
            MOVFR2GR_S => {j += 1; instructions.push(movfr2gr_s(code, address, symbols));}
            MOVFR2GR_D => {j += 1; instructions.push(movfr2gr_d(code, address, symbols));}
            MOVFRH2GR_S => {j += 1; instructions.push(movfrh2gr_s(code, address, symbols));}
            MOVGR2FCSR => {j += 1; instructions.push(movgr2fcsr(code, address, symbols));}
            MOVFCSR2GR => {j += 1; instructions.push(movfcsr2gr(code, address, symbols));}
            MOVFR2CF => {j += 1; instructions.push(movfr2cf(code, address, symbols));} 
            MOVCF2FR => {j += 1; instructions.push(movcf2fr(code, address, symbols));} 
            MOVGR2CF => {j += 1; instructions.push(movgr2cf(code, address, symbols));} 
            MOVCF2GR => {j += 1; instructions.push(movcf2gr(code, address, symbols));} 
            FCVT_S_D => {j += 1; instructions.push(fcvt_s_d(code, address, symbols));} 
            FCVT_D_S => {j += 1; instructions.push(fcvt_d_s(code, address, symbols));} 
            FTINTRM_W_S => {j += 1; instructions.push(ftintrm_w_s(code, address, symbols));}
            FTINTRM_W_D => {j += 1; instructions.push(ftintrm_w_d(code, address, symbols));}
            FTINTRM_L_S => {j += 1; instructions.push(ftintrm_l_s(code, address, symbols));}
            FTINTRM_L_D => {j += 1; instructions.push(ftintrm_l_d(code, address, symbols));}
            FTINTRP_W_S => {j += 1; instructions.push(ftintrp_w_s(code, address, symbols));}
            FTINTRP_W_D => {j += 1; instructions.push(ftintrp_w_d(code, address, symbols));}
            FTINTRP_L_S => {j += 1; instructions.push(ftintrp_l_s(code, address, symbols));}
            FTINTRP_L_D => {j += 1; instructions.push(ftintrp_l_d(code, address, symbols));}
            FTINTRZ_W_S => {j += 1; instructions.push(ftintrz_w_s(code, address, symbols));}
            FTINTRZ_W_D => {j += 1; instructions.push(ftintrz_w_d(code, address, symbols));}
            FTINTRZ_L_S => {j += 1; instructions.push(ftintrz_l_s(code, address, symbols));}
            FTINTRZ_L_D => {j += 1; instructions.push(ftintrz_l_d(code, address, symbols));}
            FTINTRNE_W_S => {j += 1; instructions.push(ftintrne_w_s(code, address, symbols));}
            FTINTRNE_W_D => {j += 1; instructions.push(ftintrne_w_d(code, address, symbols));}
            FTINTRNE_L_S => {j += 1; instructions.push(ftintrne_l_s(code, address, symbols));}
            FTINTRNE_L_D => {j += 1; instructions.push(ftintrne_l_d(code, address, symbols));}
            FTINT_W_S => {j += 1; instructions.push(ftint_w_s(code, address, symbols));}
            FTINT_W_D => {j += 1; instructions.push(ftint_w_d(code, address, symbols));}
            FTINT_L_S => {j += 1; instructions.push(ftint_l_s(code, address, symbols));}
            FTINT_L_D => {j += 1; instructions.push(ftint_l_d(code, address, symbols));}
            FFINT_S_W => {j += 1; instructions.push(ffint_s_w(code, address, symbols));}
            FFINT_S_L => {j += 1; instructions.push(ffint_s_l(code, address, symbols));}
            FFINT_D_W => {j += 1; instructions.push(ffint_d_w(code, address, symbols));}
            FFINT_D_L => {j += 1; instructions.push(ffint_d_l(code, address, symbols));}
            //FFINT_S => {j += 1; instructions.push(ffint_s(code, address, symbols));}
            FRINT_D => {j += 1; instructions.push(frint_d(code, address, symbols));}
            IOCSRRD_B => {j += 1; instructions.push(iocsrrd_b(code, address, symbols));}
            IOCSRRD_H => {j += 1; instructions.push(iocsrrd_h(code, address, symbols));}
            IOCSRRD_W => {j += 1; instructions.push(iocsrrd_w(code, address, symbols));}
            IOCSRRD_D => {j += 1; instructions.push(iocsrrd_d(code, address, symbols));}
            IOCSRWR_B => {j += 1; instructions.push(iocsrwr_b(code, address, symbols));}
            IOCSRWR_H => {j += 1; instructions.push(iocsrwr_h(code, address, symbols));}
            IOCSRWR_W => {j += 1; instructions.push(iocsrwr_w(code, address, symbols));}
            IOCSRWR_D => {j += 1; instructions.push(iocsrwr_d(code, address, symbols));}
            TLBCLR => {j += 1; instructions.push(tlbclr(code, address, symbols));}
            TLBFLUSH => {j += 1; instructions.push(tlbflush(code, address, symbols));}
            TLBSRCH => {j += 1; instructions.push(tlbsrch(code, address, symbols));} 
            TLBRD => {j += 1; instructions.push(tlbrd(code, address, symbols));} 
            TLBWR => {j += 1; instructions.push(tlbwr(code, address, symbols));} 
            TLBFILL => {j += 1; instructions.push(tlbfill(code, address, symbols));} 
            ERTN => {j += 1; instructions.push(ertn(code, address, symbols));}
            FABS_S => {j += 1; instructions.push(fabs_s(code, address, symbols));} 
            FABS_D => {j += 1; instructions.push(fabs_d(code, address, symbols));} 
            FNEG_S => {j += 1; instructions.push(fneg_s(code, address, symbols));} 
            FNEG_D => {j += 1; instructions.push(fneg_d(code, address, symbols));} 
            FLOGB_S => {j += 1; instructions.push(flogb_s(code, address, symbols));}
            FLOGB_D => {j += 1; instructions.push(flogb_d(code, address, symbols));}
            FCLASS_S => {j += 1; instructions.push(fclass_s(code, address, symbols));}
            FCLASS_D => {j += 1; instructions.push(fclass_d(code, address, symbols));}
            _ => {}
        }
        if j != 1 {
            instructions.reverse();
            //crate::app::log(&format!("instructions parse error"));
            //let mut file = File::create("error").unwrap();
            for k in 0..j {
                
            }
            instructions.reverse();
        }

        if j == 1 {
            let mut insn = instructions.pop().unwrap();
            for k in 0..4 {
                insn.bytes[k] = bytes[i as usize + k];
            }
            instructions.push(insn);
        }

        i += 4
    }
    

    Ok(instructions)
}

fn clone_into_array<A, T>(slice: &[T]) -> A 
where
    A: Default + AsMut<[T]>,
    T: Clone
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

impl fmt::Display for AssemblyInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(label) = &self.label {
            println!("<{}>:", label);
        } 
        write!(f, "0x{:08x}\t", self.address);
        write!(f, "{}\t", self.opcode);

        match &self.operand1 {
            Some(operand) => {
                match operand.operand_type {
                    OperandType::Offset => {
                        if let Some(record) = &operand.symbol {
                            if self.opcode != Opcode::JIRL { 
                                write!(f, "{} <{}> (0x{:08x})", operand.value as isize, record.name, self.address as isize + operand.value as isize); 
                            } else {
                                write!(f, "{} (0x{:08x})", operand.value as isize, self.address as isize + operand.value as isize);
                            }
                        } else if self.opcode != Opcode::JIRL {
                            write!(f, "{} (0x{:08x})", operand.value as isize, self.address as isize + operand.value as isize);
                        } else {
                            write!(f, "{}", operand.value as isize);
                        }
                        
                        write!(f, "")
                    }
                    OperandType::SignedImm => write!(f, "{}", operand.value as isize),
                    OperandType::UnsignedImm => write!(f, "{}", operand.value),
                    OperandType::FloatRegister => write!(f, "$f{}", operand.value),
                    OperandType::GeneralRegister => write!(f, "$r{}", operand.value),
                }
            }
            None => write!(f, "")
        };
        match &self.operand2 {
            Some(operand) => {
                match operand.operand_type {
                    OperandType::Offset => {
                        if let Some(record) = &operand.symbol {
                            if self.opcode != Opcode::JIRL { 
                                write!(f, ", {} <{}> (0x{:08x})", operand.value as isize, record.name, self.address as isize + operand.value as isize); 
                            } else {
                                write!(f, ", {} (0x{:08x})", operand.value as isize, self.address as isize + operand.value as isize);
                            }
                        } else if self.opcode != Opcode::JIRL {
                            write!(f, ", {} (0x{:08x})", operand.value as isize, self.address as isize + operand.value as isize);
                        } else {
                            write!(f, ", {}", operand.value as isize);
                        }
                        
                        write!(f, "")
                    }
                    OperandType::SignedImm => write!(f, ", {}", operand.value as isize),
                    OperandType::UnsignedImm => write!(f, ", {}", operand.value),
                    OperandType::FloatRegister => write!(f, ", $f{}", operand.value),
                    OperandType::GeneralRegister => write!(f, ", $r{}", operand.value),
                }
            }
            None => write!(f, "")
        };
        match &self.operand3 {
            Some(operand) => {
                match operand.operand_type {
                    OperandType::Offset => {
                        if let Some(record) = &operand.symbol {
                            if self.opcode != Opcode::JIRL { 
                                write!(f, ", {} <{}> (0x{:08x})", operand.value as isize, record.name, self.address as isize + operand.value as isize); 
                            } else {
                                write!(f, ", {} (0x{:08x})", operand.value as isize, self.address as isize + operand.value as isize);
                            }
                        } else if self.opcode != Opcode::JIRL {
                            write!(f, ", {} (0x{:08x})", operand.value as isize, self.address as isize + operand.value as isize);
                        } else {
                            write!(f, ", {}", operand.value as isize);
                        }
                        
                        write!(f, "")
                    }
                    OperandType::SignedImm => write!(f, ", {}", operand.value as isize),
                    OperandType::UnsignedImm => write!(f, ", {}", operand.value),
                    OperandType::FloatRegister => write!(f, ", $f{}", operand.value),
                    OperandType::GeneralRegister => write!(f, ", $r{}", operand.value),
                }
            }
            None => write!(f, "")
        };
        match &self.operand4 {
            Some(operand) => {
                match operand.operand_type {
                    OperandType::Offset => {
                        if let Some(record) = &operand.symbol {
                            if self.opcode != Opcode::JIRL { 
                                write!(f, ", {} <{}> (0x{:08x})", operand.value as isize, record.name, self.address as isize + operand.value as isize); 
                            } else {
                                write!(f, ", {} (0x{:08x})", operand.value as isize, self.address as isize + operand.value as isize);
                            }
                        } else if self.opcode != Opcode::JIRL {
                            write!(f, ", {} (0x{:08x})", operand.value as isize, self.address as isize + operand.value as isize);
                        } else {
                            write!(f, ", {}", operand.value as isize);
                        }
                        
                        write!(f, "")
                    }
                    OperandType::SignedImm => write!(f, ", {}", operand.value as isize),
                    OperandType::UnsignedImm => write!(f, ", {}", operand.value),
                    OperandType::FloatRegister => write!(f, ", $f{}", operand.value),
                    OperandType::GeneralRegister => write!(f, ", $r{}", operand.value),
                }
            }
            None => write!(f, "")
        }

    }
}

fn analyse_plt(insns: &mut Vec<AssemblyInstruction>, symbol: &mut HashMap<u64, SymbolRecord>) {
    let mut i = 8;
    while i < insns.len() {
        let mut value: isize = 0;
        if let Some(operand) = &insns[i].operand2 {
            value = (operand.value as isize) << 12; 
        }
        let mut addr = (insns[i].address as isize + value);
        if let Some(operand) = &insns[i + 1].operand3 {
            value = operand.value as isize;
            addr = addr + value;
        } 
        let mut name = String::new();
        if let Some(record) = symbol.get(&(addr as u64)) {
            name = record.name.clone();
            symbol.insert(insns[i].address, SymbolRecord::from_plt(addr as u64, record.name.clone()));
        }
        insns[i].label = Some(name);

        i += 4;
    }
}

pub struct LabelCounter {
    pub count: usize,
}


