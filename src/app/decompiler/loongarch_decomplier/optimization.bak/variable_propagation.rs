use crate::loongarch_decomplier::*;

pub fn variable_propagation(irs_ast: &mut AbstractSyntaxTree) {
    let max_operand_n = MAX_OPERAND_IN_ASSIGN.load(std::sync::atomic::Ordering::SeqCst) as usize;

    let mut irs_ast_clone = irs_ast.clone();
    let mut delete_ir_map = HashMap::<usize, HashSet<bool>>::new();
    for (i, ir_ast) in irs_ast.next.iter_mut().enumerate() {
        if let ASTType::Assign = ir_ast.ast_type {
            let mut n = 0;
            get_operand_number(&ir_ast, &mut n);
            repalce(ir_ast, i, &mut irs_ast_clone, max_operand_n, n, &mut delete_ir_map);   
            for (j, ast) in irs_ast_clone.next.iter_mut().enumerate() {
                if i == j {
                    *ast = Box::new(*ir_ast.clone()); 
                }
            }
        }
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
    
}

fn repalce(ir_ast: &mut AbstractSyntaxTree, index: usize, irs_ast: &mut AbstractSyntaxTree, max: usize, n: usize, delete_ir_map: &mut HashMap<usize, HashSet<bool>>) {
    if let ASTType::Assign = ir_ast.ast_type {
        for next in ir_ast.next.iter_mut() {
            repalce(next, index, irs_ast, max, n, delete_ir_map);
        }
    } else if let ASTType::Operator(_) = ir_ast.ast_type {
        for next in ir_ast.next.iter_mut() {
            repalce(next, index, irs_ast, max, n, delete_ir_map);
        } 
    } else if let ASTType::Variable(is_ptr) = ir_ast.ast_type {
        irs_ast.next.reverse();
        let len = irs_ast.next.len();
        for (i, ast) in irs_ast.next.iter().enumerate() {
            let now_index = len - index -1;
            if i <= now_index {
                continue;
            }
            if let ASTType::Assign = ast.ast_type {
                if ir_ast.value == ast.value {
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
        /*
        for (i, ast) in irs_ast.next.iter().enumerate() {
            if i >= index {
                return;
            }
            if let ASTType::Assign = ast.ast_type {
                if ir_ast.value == ast.value {
                    for next in ast.next.iter() {
                        if let ASTType::Operator(_) = next.ast_type {
                            *ir_ast = *next.clone();
                            break;
                        }
                    }
                }
            }
        }
        */
    }
}

fn get_operand_number(ast: &AbstractSyntaxTree, n: &mut usize) {
    if let ASTType::Assign = ast.ast_type {
        for next in ast.next.iter() {
            get_operand_number(next, n);
        }
    } else if let ASTType::Operator(_) = ast.ast_type {
        for next in ast.next.iter() {
            get_operand_number(next, n);
        }
    } else if let ASTType::Variable(is_ptr) = ast.ast_type {
        *n += 1;
    } else if let ASTType::Integer(_) = ast.ast_type {
        *n += 1;
    } else if let ASTType::Float(_) = ast.ast_type {
        *n += 1;
    }
}
