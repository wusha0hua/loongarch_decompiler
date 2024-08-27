//use crate::loongarch_decomplier::*;
use crate::app::decompiler::loongarch_decomplier::*;

mod variable_propagation;
mod branch_refine;
mod condiction_aggregation;
mod format_while;
mod constant_folding;
mod updata_type;

use variable_propagation::*;
use branch_refine::*;
use condiction_aggregation::*;
use format_while::*;
use constant_folding::*;
pub use updata_type::*;

pub fn optimization(ast: &mut AbstractSyntaxTree, ast_symbol_map: &HashMap<usize, ASTSymbol>) {
    //println!("variable_propagation");
    divide_into_region(ast, ast_symbol_map);
    //println!("branch_refine");
    branch_refine(ast); 
    //println!("format_condictions");
    format_condictions(ast);
    //println!("condiction_aggregation");
    condiction_aggregation(ast);
    //println!("format_while");
    format_while(ast);
    //println!("constant_folding");
    search_assign(ast);

    let mut delete_set = HashSet::<u64>::new();
    //search_redundant_ir(ast, &ast.clone(), &mut delete_set);
    //delete_redundant_ir(ast, &mut delete_set);
}

fn divide_into_region(ast: &mut AbstractSyntaxTree, ast_symbol_map: &HashMap<usize, ASTSymbol>) {
    match &ast.ast_type {
        ASTType::If => {
            for next in ast.next.iter_mut() {
                divide_into_region(next, ast_symbol_map);
            }
        }
        ASTType::Begin(_) | ASTType::Loop | ASTType::True | ASTType::False => {
            variable_propagation(ast, ast_symbol_map);  
            for next in ast.next.iter_mut() {
                divide_into_region(next, ast_symbol_map);
            }
        }
        _ => {}
    }

}

fn search_assign(ast: &mut AbstractSyntaxTree) {
    match &ast.ast_type {
        ASTType::Assign(_) => {
            constant_folding(ast);
        }
        _ => {
            for next in ast.next.iter_mut() {
                search_assign(next);
            }
        }
    }
}


pub fn format_condictions(ast: &mut AbstractSyntaxTree) {
    match &ast.ast_type {
        ASTType::If => {
            let mut condiction = AbstractSyntaxTree::new();
            for next in ast.next.iter_mut() {
                if let ASTType::Condiction(_) = &next.ast_type {
                    condiction = *next.clone();
                    break
                }
            }
            let mut condictions = AbstractSyntaxTree::new();
            condictions.ast_type = ASTType::Condictions;
            condictions.next.push(Box::new(condiction));

            for next in ast.next.iter_mut() {
                if let ASTType::Condiction(_) = &next.ast_type {
                    *next = Box::new(condictions);
                    break;
                }
            }

            for next in ast.next.iter_mut() {
                format_condictions(next);
            }
            
        }
        _ => {
            for next in ast.next.iter_mut() {
                format_condictions(next);
            }
        }
    }
}

fn search_redundant_ir(ir_ast: &mut AbstractSyntaxTree, irs: &AbstractSyntaxTree, delete_set: &mut HashSet<u64>) {
    match &ir_ast.ast_type {
        ASTType::Begin(_) | ASTType::Loop | ASTType::While | ASTType::True | ASTType::False => {
            for next in ir_ast.next.iter_mut() {
                match &next.ast_type {
                    ASTType::Assign(_) => {
                        //println!("{:?}", next.next.first().unwrap().next); 
                        let mut n = 0;
                        search_var(irs, next.value, &mut n);
                        if n == 1 {
                            delete_set.insert(next.value); 
                        }
                    }
                    _ => {
                        for next in next.next.iter_mut() {
                            search_redundant_ir(next, irs, delete_set);
                        }
                    }
                }
            } 
        }
        _ => {
            for next in ir_ast.next.iter_mut() {
                search_redundant_ir(next, irs, delete_set);
            }
        }
    }
}

fn search_var(ir_ast: &AbstractSyntaxTree, id: u64, n: &mut usize) {
    match &ir_ast.ast_type {
        ASTType::Begin(_) | ASTType::While | ASTType::Loop | ASTType::If | ASTType::True | ASTType::False | ASTType::Operator(_) | ASTType::Condictions | ASTType::Condiction(_) | ASTType::Function(_) | ASTType::Parameter => {
            for next in ir_ast.next.iter() {
                search_var(next, id, n);
            }
        }
        ASTType::Assign(_) => {
            if ir_ast.value == id {
                *n += 1
            } 

            /*
            for next in ir_ast.next.iter() {
                search_var(next, id, n);
            }
            */
        }

        ASTType::Variable(_) => {
            if ir_ast.value == id {
                *n += 1;
            }
        }

       _ => {} 
    }
}

fn delete_redundant_ir(ast: &mut AbstractSyntaxTree, delete_set: &mut HashSet<u64>) {
    let mut index_set = HashSet::<usize>::new();
    for (i, ir) in ast.next.iter().enumerate() {
        if let ASTType::Assign(_) = &ir.ast_type {
            if let Some(_) = delete_set.get(&ir.value) {
                index_set.insert(i);
                delete_set.remove(&ir.value);
            }
        }
    }

    let mut index_vec: Vec<usize> = index_set.into_iter().collect();
    index_vec.sort_by(|a, b| a.cmp(b));
    index_vec.reverse();

    for i in index_vec {
        ast.next.remove(i);
    }

    for next in ast.next.iter_mut() {
        match &next.ast_type {
            ASTType::Begin(_) | ASTType::While | ASTType::Loop | ASTType::If | ASTType::True | ASTType::False => {
                delete_redundant_ir(next, delete_set);
            }
            _ => {}
        } 
    }

}
