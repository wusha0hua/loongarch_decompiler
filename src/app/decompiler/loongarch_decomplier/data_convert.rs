//use crate::loongarch_decomplier::*;
use crate::app::decompiler::loongarch_decomplier::*;
use std::{char, collections::HashMap};

pub fn get_c_string(start: usize, bytes: &Vec<u8>) -> String {
    let mut string = String::new();
    let mut offset = start;
    let mut c = bytes[start];
    while c != 0 {
        string.push(c as char);
        offset += 1;
        c = bytes[offset];
    }
    string
}

pub fn get_c_string_from_data(start: u64, data: &HashMap<u64, Vec<u8>>) -> Option<String> {
    let mut s = String::new();
    let mut flag = false;
    for d in data {
        if start > *d.0 && start < *d.0 + d.1.len() as u64 {
            flag = true;
            let mut offset = start - *d.0;
            let mut c = d.1[offset as usize];
            while c != 0 {
                s.push(c as char);
                offset += 1;
                c = d.1[offset as usize];
            }
        }
    }

    let mut string = String::new();
    for c in s.chars() {
        if c == '\n' {
            string.push('\\');
            string.push('n');
        } else if c == '\t'{
            string.push('\\');
            string.push('t');
        } else {
            string.push(c);
        } 
    }

    if flag {
        return Some(string);
    } else {
        return None;
    }
}


pub fn set_signed(size: Size) -> Size {
    match size {
        Size::Unsigned8 => Size::Signed8,
        Size::Unsigned16 => Size::Signed16,
        Size::Unsigned32 => Size::Signed32,
        Size::Unsigned64 => Size::Signed64,
        _ => size,
    }
}

pub fn set_unsigned(size: Size) -> Size {
    match size {
        Size::Signed8 => Size::Unsigned8,
        Size::Signed16 => Size::Unsigned16,
        Size::Signed32 => Size::Unsigned32,
        Size::Signed64 => Size::Unsigned64,
        _ => size
    }
}

pub fn get_index_at_vec<T: std::cmp::PartialEq>(vec: &Vec<T>, elem: T) -> Option<usize> {
    if vec.iter().any(|e| *e == elem) {
        let mut i = 0;
        for e in vec.iter() {
            if *e == elem {
                break;
            }
            i += 1;
        } 
        Some(i)
    } else {
        None
    }
}

pub fn get_key_from_value<K: std::clone::Clone, V: std::cmp::PartialEq>(map: &HashMap<K, V>, value: V) -> Option<K> {
    for m in map.iter() {
        if *m.1 == value {
            return Some(m.0.clone());
        }
    }
    None
}
