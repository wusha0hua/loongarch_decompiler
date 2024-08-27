//use crate::loongarch_decomplier::*;
use crate::app::decompiler::loongarch_decomplier::*;

pub fn format_while(ast: &mut AbstractSyntaxTree) {
    let mut insert_map = HashMap::<usize, AbstractSyntaxTree>::new();
    for (i, next) in ast.next.iter_mut().enumerate() {
        match &next.ast_type {
            ASTType::Loop => {
                let mut break_n = 0;
                let mut continue_n = 0;
                for next in next.next.iter() {
                    chech_is_simple_while(next, &mut continue_n, &mut break_n);
                }
                
                if break_n == 1 && continue_n == 1 {
                    let irs = format_simple_while(next);
                    insert_map.insert(i, irs);
                }

                /*
                for (i, next) in next.next.iter_mut().enumerate() {
                    println!("{:?}", next.ast_type);
                    format_while(next);
                }
                */
                format_while(next);
               
            }
            _ => {
                for next in next.next.iter_mut() {
                    format_while(next);
                }
            }
        }

    }

    if insert_map.len() > 0 {
        let mut insert_vec: Vec<(usize, AbstractSyntaxTree)> = insert_map.into_iter().collect();
        insert_vec.sort_by(|a, b| a.0.cmp(&b.0));

        for (index, mut irs) in insert_vec {
            irs.next.reverse();
            for ir in irs.next {
                ast.next.insert(index, ir);
            }
        }
    }
}

/*
pub fn format_while(ast: &mut AbstractSyntaxTree, init_irs: &mut AbstractSyntaxTree) {
    match &ast.ast_type {
        ASTType::Loop => {
            let mut break_n = 0;
            let mut continue_n = 0;
            for next in ast.next.iter() {
                
                chech_is_simple_while(next, &mut continue_n, &mut break_n);
            }
            
            if break_n == 1 && continue_n == 1 {
                let init_block = format_simple_while(ast);
                *init_irs = init_block;
            }

            let mut init_replace_map = HashMap::<usize, AbstractSyntaxTree>::new();
            for (i, next) in ast.next.iter_mut().enumerate() {
                let mut init_irs = AbstractSyntaxTree::new();
                format_while(next, &mut init_irs);
                if init_irs.next.len() != 0 {
                    init_replace_map.insert(i, init_irs);
                }
            }
            if init_replace_map.len() != 0 {
                let mut n = 0;
                let mut insert_vec: Vec<(usize, AbstractSyntaxTree)> = init_replace_map.into_iter().collect();
                insert_vec.sort_by(|a, b| a.0.cmp(&b.0));
                for (i, irs) in insert_vec {
                    for ir in irs.next {
                        let index = i + n;
                        ast.next.insert(index, ir);
                        n += 1;
                    }
                }
            }
        }
        _ => {
            let mut init_replace_map = HashMap::<usize, AbstractSyntaxTree>::new();
            for (i, next) in ast.next.iter_mut().enumerate() {
                let mut init_irs = AbstractSyntaxTree::new();
                format_while(next, &mut init_irs);
                if init_irs.next.len() != 0 {
                    init_replace_map.insert(i, init_irs);
                }
            }
            if init_replace_map.len() != 0 {
                let mut n = 0;
                let mut insert_vec: Vec<(usize, AbstractSyntaxTree)> = init_replace_map.into_iter().collect();
                insert_vec.sort_by(|a, b| a.0.cmp(&b.0));
                for (i, irs) in insert_vec {
                    for ir in irs.next {
                        let index = i + n;
                        ast.next.insert(index, ir);
                        n += 1;
                    }
                }
            }
        }
    }
}
*/

fn chech_is_simple_while(loop_ast: &AbstractSyntaxTree, break_n: &mut usize, continue_n: &mut usize){
    match &loop_ast.ast_type {
        ASTType::Loop => {}
        ASTType::Break => *break_n += 1,
        ASTType::Continue => *continue_n += 1,
        _ => {
            for next in loop_ast.next.iter() {
                chech_is_simple_while(next, break_n, continue_n);
            }
        }
    }
}

fn format_simple_while(loop_ast: &mut AbstractSyntaxTree) -> AbstractSyntaxTree {
    //loop_ast.ast_type = ASTType::While;

    let mut condictions = AbstractSyntaxTree::new();
    let mut break_tf = false;
    let mut while_body = AbstractSyntaxTree::new();
    let mut init_block = AbstractSyntaxTree::new();

    for loop_next in loop_ast.next.iter() {
        let mut found_break = false;
        if let ASTType::If = &loop_next.ast_type {
            for if_next in loop_next.next.iter() {
                //println!("{:#?}\n", if_next);
                match &if_next.ast_type {
                    ASTType::Condictions => {
                        condictions = *if_next.clone(); 
                    }
                    ASTType::True => {
                        if if_next.next.len() == 1 {
                            if let ASTType::Break = if_next.next.first().unwrap().ast_type {
                                break_tf = true;
                                found_break = true;
                            }
                        }
                    }
                    ASTType::False => {
                        if if_next.next.len() == 1 {
                            if let ASTType::Break = if_next.next.first().unwrap().ast_type {
                                break_tf = false;
                                found_break = true;
                            }
                        }
                    }
                    _ => {}
                } 
            }

            if !found_break {
                init_block.next.push(loop_next.clone());
                continue;
            }

            for if_next in loop_next.next.iter() {
                match &if_next.ast_type {
                    ASTType::True => {
                        if !break_tf {
                            while_body = *if_next.clone();
                            break;
                        }
                    }
                    ASTType::False => {
                        if break_tf {
                            while_body = *if_next.clone();
                            break;
                        }
                    }
                    _ => {}
                }
            }
            
        } else {
            init_block.next.push(loop_next.clone());
        }
    }


    if init_block.next.len() != 0 {
        for init_ir in init_block.next.iter() {
            if let ASTType::Assign(_) = &init_ir.ast_type {
                let mut vars = Vec::<AbstractSyntaxTree>::new(); 
                get_right_var(init_ir, &mut vars);
                for var in vars.iter() {
                    let mut is_static = true;
                    check_var_static(&loop_ast, var, &mut is_static, &init_block);
                    if !is_static {
                        return AbstractSyntaxTree::new();
                    }
                }
            } else {
                return AbstractSyntaxTree::new();
            }
        }
    }


    //println!("{:#?}", while_body);

    loop_ast.ast_type = ASTType::While;
    loop_ast.next.clear();
    if break_tf {
        // 多条件取反
    } else {
        loop_ast.next.push(Box::new(condictions)); 
        for ast in while_body.next {
            if ast.ast_type != ASTType::Continue {
                loop_ast.next.push(ast);
            }
        }
    } 

    init_block
    //println!("{:#?}", loop_ast);
}

fn get_right_var(ast: &AbstractSyntaxTree, vars: &mut Vec<AbstractSyntaxTree>) {
    match &ast.ast_type {
        ASTType::Assign(_) | ASTType::Operator(_) => {
            for next in ast.next.iter() {
                get_right_var(next, vars);
            }
        }
        ASTType::Variable(_) => {
            vars.push(ast.clone());
        }
        _ => {}
    }
}

fn check_var_static(ast: &AbstractSyntaxTree, var: &AbstractSyntaxTree, is_static: &mut bool, init_irs: &AbstractSyntaxTree) {
    match &ast.ast_type {
        ASTType::While | ASTType::Loop | ASTType::If | ASTType::True | ASTType::False => {
            for next in ast.next.iter() {
                check_var_static(next, var, is_static, init_irs);
            }
        }
        ASTType::Assign(_) => {
            for ir in init_irs.next.iter() {
                if **ir == *ast {
                    continue; 
                } else {
                    if ast.value == ir.value {
                        *is_static = false;
                    }
                }
            } 
        }
        _ => {}
    } 
}
