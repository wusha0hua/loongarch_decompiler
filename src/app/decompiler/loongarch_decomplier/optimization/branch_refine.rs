//use crate::loongarch_decomplier::*;
use crate::app::decompiler::loongarch_decomplier::*;

pub fn branch_refine(ast: &mut AbstractSyntaxTree) {
    match &ast.ast_type {
        ASTType::If => {
            let mut exist_false = false;
            let mut exist_true = false;
            for next in ast.next.iter_mut() {
                if next.ast_type == ASTType::True {
                    exist_true = true;
                } else if next.ast_type == ASTType::False {
                    exist_false = true;
                }
            }

            if exist_true == true && exist_false == true {
            } else if exist_true == true && exist_false == false {
            } else if exist_true == false && exist_false == true {
                for next in ast.next.iter_mut() {
                    if next.ast_type == ASTType::False {
                        next.ast_type = ASTType::True;
                    } else if let ASTType::Condiction(relation) = &next.ast_type {
                        next.ast_type = ASTType::Condiction(!relation.clone()); 
                    } 
                }
            }

            for next in ast.next.iter_mut() {
                branch_refine(next);
            }
        }
        ASTType::Loop | ASTType::False | ASTType::True | ASTType::Begin(_) => {
            for next in ast.next.iter_mut() {
                branch_refine(next);
            }
        }
        _ => {

        }
    }     
}

fn branch_refine_recursion(ast: &mut AbstractSyntaxTree) {
}
