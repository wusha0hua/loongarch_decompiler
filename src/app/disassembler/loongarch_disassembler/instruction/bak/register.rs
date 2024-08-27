#[derive(Debug, Clone)]
pub enum Register {
    GR(usize),
    FR(usize),
}

#[derive(Debug, Clone)]
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
const F17: usize = 17;
const F18: usize = 18;
const F19: usize = 19;
const F20: usize = 20;
const F21: usize = 21;
const F22: usize = 22;
const F23: usize = 23;
const F24: usize = 24;
const F25: usize = 25;
const F26: usize = 26;
const F27: usize = 27;
const F28: usize = 28;
const F29: usize = 29;
const F30: usize = 30;
const F31: usize = 31;


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
    }
}
