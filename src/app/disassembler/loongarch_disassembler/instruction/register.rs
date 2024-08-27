use serde_json;
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Register {
    GR(u64),
    FR(u64),
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

const R0: u64 = 0;
const R1: u64 = 1;
const R2: u64 = 2;
const R3: u64 = 3;
const R4: u64 = 4;
const R5: u64 = 5;
const R6: u64 = 6;
const R7: u64 = 7;
const R8: u64 = 8;
const R9: u64 = 9;
const R10: u64 = 10;
const R11: u64 = 11;
const R12: u64 = 12;
const R13: u64 = 13;
const R14: u64 = 14;
const R15: u64 = 15;
const R16: u64 = 16;
const R17: u64 = 17;
const R18: u64 = 18;
const R19: u64 = 19;
const R20: u64 = 20;
const R21: u64 = 21;
const R22: u64 = 22;
const R23: u64 = 23;
const R24: u64 = 24;
const R25: u64 = 25;
const R26: u64 = 26;
const R27: u64 = 27;
const R28: u64 = 28;
const R29: u64 = 29;
const R30: u64 = 30;
const R31: u64 = 31;

const F0: u64 = 0;
const F1: u64 = 1;
const F2: u64 = 2;
const F3: u64 = 3;
const F4: u64 = 4;
const F5: u64 = 5;
const F6: u64 = 6;
const F7: u64 = 7;
const F8: u64 = 8;
const F9: u64 = 9;
const F10: u64 = 10;
const F11: u64 = 11;
const F12: u64 = 12;
const F13: u64 = 13;
const F14: u64 = 14;
const F15: u64 = 15;
const F16: u64 = 16;
const F17: u64 = 17;
const F18: u64 = 18;
const F19: u64 = 19;
const F20: u64 = 20;
const F21: u64 = 21;
const F22: u64 = 22;
const F23: u64 = 23;
const F24: u64 = 24;
const F25: u64 = 25;
const F26: u64 = 26;
const F27: u64 = 27;
const F28: u64 = 28;
const F29: u64 = 29;
const F30: u64 = 30;
const F31: u64 = 31;


pub fn get_gr_from_value(value: u64) -> GR {
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

pub fn get_fr_from_value(value: u64) -> FR {
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
