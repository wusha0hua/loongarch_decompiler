//use crate::loongarch_decomplier::*;
use crate::app::decompiler::loongarch_decomplier::*;

pub fn updata_type(ast: &mut AbstractSyntaxTree, ast_symbol_map: &mut HashMap<usize, ASTSymbol>) {
    let mut replace_map = HashMap::<usize, ASTSymbolValueType>::new();
    let mut target = HashSet::<usize>::new(); 
    reverse_ast(ast, ast_symbol_map, &mut replace_map, &mut target);
    //println!("target: {:?}", target); 
    //println!("replace_map: {:?}", replace_map);
    //println!("{:?}\n", ast_symbol_map);
    for (id, size) in replace_map {
        let sym = ast_symbol_map.entry(id).or_insert(ASTSymbol::new(usize::MAX));
        sym.select_type = size;
    }
}

fn reverse_ast(ast: &mut AbstractSyntaxTree, ast_symbol_map: &mut HashMap<usize, ASTSymbol>, replace_map: &mut HashMap<usize, ASTSymbolValueType>, target: &mut HashSet<usize>) {
    ast.next.reverse();
    for next in ast.next.iter_mut() {
        match &next.ast_type {
            ASTType::Assign(is_ptr) => {
                let assign_sym = ast_symbol_map[&(next.value as usize)].clone();
                if is_type_ptr(&assign_sym.select_type) {
                    let mut is_exist = false;
                    is_exist_ptr(&next, &mut is_exist, &ast_symbol_map);
                    if !is_exist && assign_sym.scope == Scope::Temp && *is_ptr == false {
                        //println!("{}", next.to_string(&ast_symbol_map));
                        search_target(next, ast_symbol_map, target,  replace_map, &assign_sym.select_type);
                    }
                } 

                if let Some(id) = target.get(&(next.value as usize)) {
                    replace(next, ast_symbol_map, target, replace_map); 
                }
            }
            ASTType::If | ASTType::Loop | ASTType::True | ASTType::False  => {
                reverse_ast(next, ast_symbol_map, replace_map, target);
            }
            _ => {}
        }
    }
    ast.next.reverse();
}

fn is_type_ptr(_type: &ASTSymbolValueType) -> bool {
    match _type {
        ASTSymbolValueType::UnsignedChar => false,
        ASTSymbolValueType::Char => false,
        ASTSymbolValueType::UnsignedShort => false,
        ASTSymbolValueType::Short => false,
        ASTSymbolValueType::UnsignedInt => false,
        ASTSymbolValueType::Int => false,
        ASTSymbolValueType::UnsignedLong => false,
        ASTSymbolValueType::Long => false,
        ASTSymbolValueType::PtrUnsignedChar => true,
        ASTSymbolValueType::PtrChar => true,
        ASTSymbolValueType::PtrUnsignedShort => true,
        ASTSymbolValueType::PtrShort => true,
        ASTSymbolValueType::PtrUnsignedInt => true,
        ASTSymbolValueType::PtrInt => true,
        ASTSymbolValueType::PtrUnsignedLong => true,
        ASTSymbolValueType::PtrLong => true,
        ASTSymbolValueType::Unknown => panic!("error"),
        ASTSymbolValueType::Ptr => true,
    }
}

fn is_exist_ptr(ast: &AbstractSyntaxTree, is_exist: &mut bool, ast_symbol_map: &HashMap<usize, ASTSymbol>) {
    match &ast.ast_type {
        ASTType::Variable(_) => {
            let sym = &ast_symbol_map[&(ast.value as usize)];
            if is_type_ptr(&sym.select_type) {
                *is_exist = true;
            } 
        }
        _ => {
            for next in ast.next.iter() {
                is_exist_ptr(next, is_exist, ast_symbol_map);
            }
        }
    }
}

fn search_target(ast: &AbstractSyntaxTree, ast_symbol_map: &mut HashMap<usize, ASTSymbol>, target: &mut HashSet<usize>, replace_map: &mut HashMap<usize, ASTSymbolValueType>, ptr_type: &ASTSymbolValueType) {
    match &ast.ast_type {
        ASTType::Assign(_) => {
            let first_ast = ast.next.first().unwrap();
            if let ASTType::Operator(op) = &first_ast.ast_type {
                let op_ast = &ast.next.first().unwrap();
                let mut long_n = 0;
                let mut id = 0;
                for operand_ast in op_ast.next.iter() {
                    if let ASTType::Integer(_, _) = &operand_ast.ast_type {
                        continue; 
                    }
                    let sym = &ast_symbol_map[&(operand_ast.value as usize)]; 
                    if sym.select_type == ASTSymbolValueType::Long {
                        long_n += 1;
                        id = operand_ast.value;
                    }
                }

                if long_n == 1 {
                    let mut sym = ast_symbol_map[&(id as usize)].clone();
                    sym.select_type = ptr_type.clone();
                    ast_symbol_map.insert(sym.id, sym);
                } else {
                    for operand_ast in op_ast.next.iter() {
                        if let ASTType::Variable(_) = &operand_ast.ast_type {
                            let sym = &ast_symbol_map[&(operand_ast.value as usize)];
                            if sym.select_type == ASTSymbolValueType::Long {
                                target.insert((operand_ast.value as usize));
                                replace_map.insert((operand_ast.value as usize), ptr_type.clone());
                            }
                        }
                    }
                }
            } else {
                if let ASTType::Variable(_) = &first_ast.ast_type {
                    let mut sym = ast_symbol_map[&(first_ast.value as usize)].clone();
                    sym.select_type = ptr_type.clone();
                    ast_symbol_map.insert((first_ast.value as usize), sym);
                }
            }
        }
        _ => {
            for next in ast.next.iter() {
                search_target(next, ast_symbol_map, target, replace_map, ptr_type);
            }
        }
    }
}

fn replace(ast: &AbstractSyntaxTree, ast_symbol_map: &HashMap<usize, ASTSymbol>, target: &mut HashSet<usize>, replace_map: &mut HashMap<usize, ASTSymbolValueType>) {
    let assign_id = ast.value;
    if let None = target.get(&(assign_id as usize)) {
        return;
    }
    match &ast.ast_type {
        ASTType::Assign(_) => {
            let first_ast = ast.next.first().unwrap();
            if let ASTType::Operator(_) = &first_ast.ast_type {
                for operand_ast in first_ast.next.iter() {
                    if let ASTType::Integer(_, _) = &operand_ast.ast_type {
                        continue;
                    }
                    let operand_sym = &ast_symbol_map[&(operand_ast.value as usize)];
                    if operand_sym.select_type != ASTSymbolValueType::Long {
                        target.remove(&(ast.value as usize));
                        replace_map.remove(&(ast.value as usize));
                        return;
                    } else {
                        target.remove(&(ast.value as usize));
                        let size = replace_map[&(ast.value as usize)].clone();
                        replace_map.remove(&(ast.value as usize));
                        replace_map.insert(operand_ast.value as usize, size);
                        target.insert(operand_ast.value as usize);
                    } 
                }
            } else if let ASTType::Variable(_) = &first_ast.ast_type {
                let size = replace_map[&(ast.value as usize)].clone();
                //replace_map.remove(&ast.value);
                //target.remove(&ast.value);
                replace_map.insert((first_ast.value as usize), size);
                target.insert(first_ast.value as usize);
            }
        } 
        _ => {}
    }
}
