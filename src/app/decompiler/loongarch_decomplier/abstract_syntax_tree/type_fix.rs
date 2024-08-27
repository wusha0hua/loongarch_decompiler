//use crate::loongarch_decomplier::*;
use crate::app::decompiler::loongarch_decomplier::*;

pub fn updata_ptr(irs_ast: &mut AbstractSyntaxTree, ast_symbol_map: &mut HashMap<usize, ASTSymbol>) {
    irs_ast.next.reverse();
    for ir_ast in irs_ast.next.iter_mut() {
        let sym = &ast_symbol_map[&(ir_ast.value as usize)];
        if is_ptr(&sym.select_type) {
            let mut is_exist = false;
            let mut target = HashSet::<usize>::new();
            is_exist_ptr(&ir_ast, &mut is_exist, &ast_symbol_map, &mut target);
            if !is_exist {
                //search_target();
            }
        } 
    }
    irs_ast.next.reverse();
}

fn is_exist_ptr(ir_ast: &AbstractSyntaxTree, is_exist: &mut bool, ast_symbol_map: &HashMap<usize, ASTSymbol>, target: &mut HashSet<usize>) {
    match &ir_ast.ast_type {
        ASTType::Variable(_) => {
            let sym = &ast_symbol_map[&(ir_ast.value as usize)];
            if is_ptr(&sym.select_type) {
                *is_exist = true;
            } else {
                target.insert(ir_ast.value as usize);
            }
        }
        _ => {
            for next in ir_ast.next.iter() {
                is_exist_ptr(next, is_exist, ast_symbol_map, target);
            }
        }
    }
} 

fn search_target(target: &mut HashSet<usize>, irs_ast: &AbstractSyntaxTree, ast_symbol_map: &HashMap<usize, ASTSymbol>) -> Option<usize> {
    None
}

/*
pub fn updata_type(irs_ast: &mut AbstractSyntaxTree, ast_symbol_map: &mut HashMap<usize, ASTSymbol>) {
    updata_ptr(irs_ast, ast_symbol_map);
}

pub fn updata_ptr(irs_ast: &mut AbstractSyntaxTree, ast_symbol_map: &mut HashMap<usize, ASTSymbol>) {
    irs_ast.next.reverse();
    for ir_ast in irs_ast.next.iter_mut() {
        updata_ptr_recursion(ir_ast, ast_symbol_map, &ASTSymbolValueType::Unknown)
    }
    irs_ast.next.reverse();
}

pub fn updata_ptr_recursion(ir_ast: &mut AbstractSyntaxTree, ast_symbol_map: &mut HashMap<usize, ASTSymbol>, assign_type: &ASTSymbolValueType) {
    match &ir_ast.ast_type {
        ASTType::Assign(_) => {
            let sym = ast_symbol_map.entry(ir_ast.value).or_insert(ASTSymbol::new(ir_ast.value));
            let assign_type = sym.select_type.clone();
            for next in ir_ast.next.iter_mut() {
                updata_ptr_recursion(next, ast_symbol_map, &assign_type);
            }
        }
        ASTType::Operator(_) => {
            for next in ir_ast.next.iter_mut() {
                updata_ptr_recursion(next, ast_symbol_map, assign_type);
            }
        }
        ASTType::Variable(_) => {
            let sym = ast_symbol_map.entry(ir_ast.value).or_insert(ASTSymbol::new(ir_ast.value));
            if sym.select_type == ASTSymbolValueType::Ptr {
                sym.select_type = assign_type.clone();
            }
        }
        ASTType::If | ASTType::Loop | ASTType::True | ASTType::False => {
            ir_ast.next.reverse();
            for next in ir_ast.next.iter_mut() {
                updata_ptr_recursion(next, ast_symbol_map, assign_type);
            }
            ir_ast.next.reverse();
        }
        _ => {}
    } 
}
*/




/*
pub fn updata_type(irs_ast: &mut AbstractSyntaxTree, ast_symbol_map: &mut HashMap<usize, ASTSymbol>) {
    for ir_ast in irs_ast.next.iter_mut() {
        if let ASTType::Assign(_) = &ir_ast.ast_type {
            let _type = ast_symbol_map[&ir_ast.value].select_type.clone(); 
            updata_type_recursion(ir_ast, ast_symbol_map, &_type);
        }
    }
}

fn updata_type_recursion(ir_ast: &mut AbstractSyntaxTree, ast_symbol_map: &mut HashMap<usize, ASTSymbol>, assign_type: &ASTSymbolValueType) {
    match &ir_ast.ast_type {
        ASTType::Assign(_) => {
            for next in ir_ast.next.iter_mut() {
                updata_type_recursion(next, ast_symbol_map, assign_type);
            }
        }
        ASTType::Operator(_) => {
            for next in ir_ast.next.iter_mut() {
                updata_type_recursion(next, ast_symbol_map, assign_type);
            }
        }
        ASTType::Variable(_) => {
            let sym = ast_symbol_map.entry(ir_ast.value).or_insert(ASTSymbol::new(ir_ast.value)); 
            if sym.select_type == ASTSymbolValueType::Unknown {
                sym.select_type = assign_type.clone();
            }
        }

        _ => {}
    }
}
*/
