//use crate::loongarch_decomplier::*;
use crate::app::decompiler::loongarch_decomplier::*;

/*
pub fn variable_propagation(irs_ast: &mut AbstractSyntaxTree, ast_symbol_map: &HashMap<usize, ASTSymbol>) {
    let max_operand_n = MAX_OPERAND_IN_ASSIGN.load(std::sync::atomic::Ordering::SeqCst) as usize;
    
    let mut irs_ast_clone = irs_ast.clone();
    let mut delete_ir_map = HashMap::<usize, HashSet<bool>>::new();

    for (i, ir_ast) in irs_ast.next.iter_mut().enumerate() {
        if let ASTType::Assign(_) = &ir_ast.ast_type {
            let mut n = 0;
            get_operand_number(&ir_ast, &mut n);            
            replace(ir_ast, ast_symbol_map, i, &mut irs_ast_clone, max_operand_n, n, &mut delete_ir_map);
        }
    }
}
*/


pub fn variable_propagation(irs_ast: &mut AbstractSyntaxTree, ast_symbol_map: &HashMap<usize, ASTSymbol>) {
    let max_operand_n = MAX_OPERAND_IN_ASSIGN.load(std::sync::atomic::Ordering::SeqCst) as usize;

    let mut irs_ast_clone = irs_ast.clone();
    let mut delete_ir_map = HashMap::<usize, HashSet<bool>>::new();

    for (i, ir_ast) in irs_ast.next.iter_mut().enumerate() {
        if let ASTType::Assign(is_ptr) = ir_ast.ast_type {
            //println!("{}", ir_ast.to_string(ast_symbol_map));
            let mut n = 0;
            get_operand_number(&ir_ast, &mut n);
            replace(ir_ast, ast_symbol_map, i, &mut irs_ast_clone, max_operand_n, n, &mut delete_ir_map);
            for (j, ast) in irs_ast_clone.next.iter_mut().enumerate() {
                if i == j {
                    *ast = Box::new(*ir_ast.clone());
                }
            }
        } 
        //println!("\n{}\n", irs_ast_clone.to_string(ast_symbol_map));
    }

    let mut n = 0;
    let mut vec: Vec<(usize, HashSet<bool>)> = delete_ir_map.into_iter().collect();
    vec.sort_by(|a, b| a.0.cmp(&b.0));

    for (index, set) in vec.iter() {
        if set.len() == 0 {

        } else if set.len() == 2 {
            
        } else if let Some(_) = set.get(&false) {

        } else {
            irs_ast.next.remove(index - n);
            n += 1;
        }
    }

    let mut replace_vec = Vec::<usize>::new();
    let mut replace_map = HashMap::<usize, usize>::new();
    let mut replace_val_map = HashMap::<usize, bool>::new();
    for (i, ir_ast) in irs_ast.next.iter().enumerate() {
        if let ASTType::Assign(is_val) = &ir_ast.ast_type {
            let operand = ir_ast.next.first().unwrap();
            if let ASTType::Variable(var_is_val) = &operand.ast_type {
                let operand_ast = ir_ast.next.first().unwrap();
                let operand_sym = &ast_symbol_map[&(operand_ast.value as usize)];
                let assign_sym = &ast_symbol_map[&(ir_ast.value as usize)];
                if operand_sym.select_type != assign_sym.select_type {
                    continue;
                }
                /*
                if *is_val != *var_is_val || operand_sym.select_type != assign_sym.select_type {
                    continue;
                } 
                */
                if is_assign_static(&irs_ast, i, ir_ast.value as usize) {
                    //println!("{}", ir_ast.to_string(&ast_symbol_map));
                    replace_vec.push(i);
                    replace_map.insert(i, operand_ast.value as usize);
                    replace_val_map.insert(i, *var_is_val); 
                }
            }
        } 
    }

    //replace_vec.reverse();
    //println!("{:?}", replace_vec);
    let mut delete_set = HashSet::<usize>::new();
    while let Some(i) = replace_vec.pop() {
        //println!("{}", i);
        replace_single(irs_ast, i, &mut delete_set, &replace_map, &replace_val_map, ast_symbol_map); 
    }
    //println!("");

    let mut delete_vec: Vec<usize> = delete_set.into_iter().collect();
    delete_vec.sort_by(|a, b| a.cmp(b));
    delete_vec.reverse();

    for index in delete_vec {
        irs_ast.next.remove(index);
    }
}

/*
fn replace(ir_ast: &mut AbstractSyntaxTree, ast_symbol_map: &HashMap<usize, ASTSymbol>, index: usize, irs_ast: &mut AbstractSyntaxTree, max: usize, n: usize, delete_ir_map: &mut HashMap<usize, HashSet<bool>>) {
    match &ir_ast.ast_type {
        ASTType::Assign(_) | ASTType::Operator(_) => {
            for next in ir_ast.next.iter_mut() {
                replace(next, ast_symbol_map, index, irs_ast, max, n, delete_ir_map);
            }
        }
        ASTType::Variable(var_is_ptr) => {
            irs_ast.next.reverse();
        }
    }
}

*/

fn replace(ir_ast: &mut AbstractSyntaxTree, ast_symbol_map: &HashMap<usize, ASTSymbol>, index: usize, irs_ast: &mut AbstractSyntaxTree, max: usize, n: usize, delete_ir_map: &mut HashMap<usize, HashSet<bool>>) {
    match &ir_ast.ast_type {
        ASTType::Assign(_) | ASTType::Operator(_) => {
            for next in ir_ast.next.iter_mut() {
                replace(next, ast_symbol_map, index, irs_ast, max, n, delete_ir_map);
            }
        }
        ASTType::Variable(var_is_ptr) => {
            let var_is_ptr = var_is_ptr.clone();
            irs_ast.next.reverse();
            let len = irs_ast.next.len();
            for (i, ast) in irs_ast.next.iter().enumerate() {
                let now_index = len - index - 1;
                if i <= now_index {
                    continue;
                }
                if let ASTType::Assign(assign_is_ptr) = ast.ast_type {
                    let sym = &ast_symbol_map[&(ast.value as usize)];
                    if sym.scope != abstract_syntax_tree::Scope::Temp {
                        continue;
                    }
                    if ir_ast.value == ast.value && var_is_ptr == assign_is_ptr {
                        let mut n2 = 0;
                        get_operand_number(ast, &mut n2);
                        for next in ast.next.iter() {
                            if let ASTType::Operator(_) = next.ast_type {
                                if n + n2 - 1 <= max {
                                    *ir_ast = *next.clone();
                                    let set = delete_ir_map.entry(len - i - 1).or_default();
                                    set.insert(true);
                                } else {
                                    let set = delete_ir_map.entry(len - i - 1).or_default();
                                    set.insert(false);
                                }
                            }
                            break;
                        }
                    }
                }
            }
            irs_ast.next.reverse();
        }
        _ => {}
    }
}

fn get_operand_number(ast: &AbstractSyntaxTree, n: &mut usize) {
    match &ast.ast_type {
        ASTType::Assign(_) => {
            for next in ast.next.iter() {
                get_operand_number(next, n);
            }
        }
        ASTType::Operator(_) => {
            for next in ast.next.iter() {
                get_operand_number(next, n);
            }
        }
        ASTType::Variable(_) => {
            *n += 1;
        }
        ASTType::Integer(_, _) => {
            *n += 1;
        }
        ASTType::Float(_, _) => {
            *n += 1;
        }
        _ => {}
    }
}


fn is_assign_static(irs_ast: &AbstractSyntaxTree, index: usize, target: usize) -> bool {
    let mut is_static = true;
    for (i, ir_ast) in irs_ast.next.iter().enumerate() {
        if i <= index {
            continue;
        }
        match &ir_ast.ast_type {
            ASTType::Assign(_) => {
                if ir_ast.value as usize == target {
                    is_static = false;
                }
                //is_assign_operand_static(ir_ast, target, &mut is_static);
            }
            _ => {}
        }
    }
    is_static
}

fn is_assign_operand_static(ir_ast: &AbstractSyntaxTree, target: usize, is_static: &mut bool) {
    match &ir_ast.ast_type {
        ASTType::Assign(_) | ASTType::Operator(_) => {
            for next in ir_ast.next.iter() {
                is_assign_operand_static(next, target, is_static);
            }
        }
        ASTType::Variable(_) => {
            if ir_ast.value as usize == target {
                *is_static = false;
            }
        }
        _ => {}
    }
}

fn replace_single(irs_ast: &mut AbstractSyntaxTree, index: usize, delete_set: &mut HashSet<usize>, replace_map: &HashMap<usize, usize>, replace_val_map: &HashMap<usize, bool>, ast_symbol_map: &HashMap<usize, ASTSymbol>) {
    let mut target_is_val = false;
    let mut target_id = 0;
    for (i, ir_ast) in irs_ast.next.iter_mut().enumerate() {
        if i < index {
            continue;
        } else if i == index {
            if let ASTType::Assign(is_val) = &ir_ast.ast_type {
                target_is_val = *is_val;
                target_id = ir_ast.value;
            }
        } else if let ASTType::Assign(_) = &ir_ast.ast_type {
            let mut is_replace = false;
            replace_assign(ir_ast, target_id as usize, target_is_val, &mut is_replace, replace_map[&index], replace_val_map[&index]);  
            if is_replace {
                delete_set.insert(index);
            }
        }

    }
}

fn replace_assign(ir_ast: &mut AbstractSyntaxTree, target_id: usize, target_is_val: bool, is_replace: &mut bool, replace_id: usize, replace_is_val: bool) {
    match &ir_ast.ast_type {
        ASTType::Variable(var_is_val) => {
            if ir_ast.value as usize == target_id && *var_is_val == target_is_val {
                *is_replace = true;
                ir_ast.value = replace_id as u64; 
                ir_ast.ast_type = ASTType::Variable(replace_is_val); 
            }
        }
        _ => {
            for next in ir_ast.next.iter_mut() {
                replace_assign(next, target_id, target_is_val, is_replace, replace_id, replace_is_val);
            }
        }
    }
}
