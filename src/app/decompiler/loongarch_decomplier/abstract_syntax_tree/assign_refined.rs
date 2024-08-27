//use crate::loongarch_decomplier::*;
use crate::app::decompiler::loongarch_decomplier::*;

#[derive(Debug)]
struct Bracket {
    left_index: usize,
    right_index: usize,
    left_operator: Option<Operator>,
    middle_operator: Option<Operator>,
    right_operator: Option<Operator>,
}

impl Bracket {
    fn from(left_index: usize, right_index: usize, left_operator: Option<Operator>, middle_operator: Option<Operator>, right_operator: Option<Operator>) -> Self {
        Self {
            left_index,
            right_index,
            left_operator,
            middle_operator,
            right_operator,
        }
    }
}

pub fn refine_assign_str_bracket(assign_str: String) -> String {
    let mut assign_str_vec = assign_str.as_bytes().to_vec();
    //assign_str_vec.push(0);

    let mut left_operator: Option<Operator> = None;
    let mut middle_operator: Option<Operator> = None;
    let mut right_operator: Option<Operator> = None;

    let mut left_index = 0;
    let mut right_index = 0;

    let mut left_bracket_set = std::collections::HashSet::<usize>::new();
    let mut bracket_vec = Vec::<Bracket>::new();

    let mut op_n = 0;

    let mut stack = Vec::<(usize, char)>::new();
    for (i, c) in assign_str_vec.iter().enumerate() {
        let c = *c as char;
        match c {
            c @ ('(' | ')' | '+' | '/') => {
                stack.push((i, c));
                op_n += 1;
            }
            '-' => {
                if assign_str_vec[i + 1] == ' ' as u8 {
                    stack.push((i, c));
                    op_n += 1;
                }
            }
            '*' => {
                if assign_str_vec[i + 1] == ' ' as u8 {
                    stack.push((i, c));
                    op_n += 1;
                }
            }
            _ => {}
        }
    }

    let mut n = 0;
    let mut left_bracket_stack = Vec::<usize>::new();
    for (i, c) in stack.iter() {
        match c {
            c @ ('+' | '-' | '*' | '/') => {
                match c {
                    '+' => middle_operator = Some(Operator::Add),
                    '-' => middle_operator = Some(Operator::Sub),
                    '*' => middle_operator = Some(Operator::Mul),
                    '/' => middle_operator = Some(Operator::Div),
                    _ => {}
                }
                let mut nn = 1;
                let index = i;
                for (i, c) in stack.iter() {
                    if i < index {
                        continue;
                    }
                    match c {
                        '(' => nn += 1,
                        ')' => {
                            nn -= 1;
                            if nn == 0 {
                                bracket_vec.push(Bracket::from(left_bracket_stack.pop().unwrap(), *i, None, middle_operator.clone(), None));
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
            '(' => {
                n += 1;
                left_bracket_stack.push(*i);
            }
            ')' => n -= 1,
            _ => {}
        }
    }

    let len = assign_str_vec.len();
    for b in bracket_vec.iter_mut() {
        let left = b.left_index;
        let right = b.right_index;

        if left as isize - 2 < 0 {
            b.left_operator = None;
        } else if assign_str_vec[left - 1] == '(' as u8 {
            b.left_operator = None;
        } else {
            match assign_str_vec[left - 2] as char {
                '+' => b.left_operator = Some(Operator::Add),
                '-' => b.left_operator = Some(Operator::Sub),
                '*' => b.left_operator = Some(Operator::Mul),
                '/' => b.left_operator = Some(Operator::Div),
                _ => {}
            }
        }

        if right + 2 >= len {
            b.right_operator = None;
        } else if assign_str_vec[right + 1] == ')' as u8 {
            b.right_operator = None;
        } else {
            match assign_str_vec[right + 2] as char {
                '+' => b.right_operator = Some(Operator::Add),
                '-' => b.right_operator = Some(Operator::Sub),
                '*' => b.right_operator = Some(Operator::Mul),
                '/' => b.right_operator = Some(Operator::Div),
                _ => {}
            }
        }
    }

    for v in bracket_vec.iter() {
        //println!("{:?}", v);
    }
    for (i, c) in assign_str_vec.iter().enumerate() {
        //println!("{}: {}", *c as char, i);
    }

    for b in bracket_vec.iter() {
        if is_delete_bracket(b) {
            assign_str_vec[b.left_index] = 0;
            assign_str_vec[b.right_index] = 0;
        }
    }

    let mut index: Option<usize> = None;
    let mut is_found_eq = false;
    for (i, c) in assign_str_vec.iter().enumerate() {
        let c = *c as char;
        if c == '=' {
            is_found_eq = true;
        }
        if c == '*' && assign_str_vec[i + 1] != ' ' as u8 && is_found_eq == true {
            index = Some(i);
            break
        }
    }

    if let Some(index) = index {
        if op_n != 0 {
            assign_str_vec.insert(index + 1, '(' as u8);
            assign_str_vec.push(')' as u8);
        }
    }

    String::from_utf8(assign_str_vec.into_iter().filter(|x| *x != 0).collect()).unwrap()
}

fn is_delete_bracket(b: &Bracket) -> bool {
    let middle_operator = match &b.middle_operator {
        Some(op) => op,
        None => return false,
    };

    if let Some(lop) = &b.left_operator {
        if let Some(rop) = &b.right_operator {
            if *middle_operator == Operator::Add || *middle_operator == Operator::Sub {
                if *lop == Operator::Sub {
                    return false;
                } else if *lop == Operator::Mul || *lop == Operator::Div || *rop == Operator::Mul || *rop == Operator::Div {
                    return false;
                } else {
                    return true;
                }
            } else if *middle_operator == Operator::Mul || *middle_operator == Operator::Div {
                if *lop == Operator::Div {
                    return false;
                } else {
                    return true;
                }
            }
        } else {
            if *lop == Operator::Add {
                return true;
            } else if *lop == Operator::Sub {
                if *middle_operator == Operator::Mul || *middle_operator == Operator::Div {
                    return true;
                }
            } else if *lop == Operator::Mul {
                if *middle_operator == Operator::Mul {
                    return true;
                }
            } else if *lop == Operator::Div {
                return false;
            }
        }
    } else {
        if let Some(rop) = &b.right_operator {
            if *middle_operator == Operator::Add  || *middle_operator == Operator::Sub {
                if *rop == Operator::Add || *rop == Operator::Sub {
                    return true;
                }
            } else if *middle_operator == Operator::Mul || *middle_operator == Operator::Div {
                return true;
            }
        } else {
            return true;
        }
    }
    false
}


