pub use crate::loongarch_decomplier::*;

mod constant_propagation;
mod constant_folding;
mod sign_adjust;
mod variable_propagation;

pub use constant_propagation::*;
pub use constant_folding::*;
pub use sign_adjust::*;
pub use variable_propagation::*;


pub fn optimization(irs_ast: &mut AbstractSyntaxTree) {
    /*
    variable_propagation(irs_ast);
    for ir_ast in irs_ast.next.iter_mut() {
        if let ASTType::Assign = ir_ast.ast_type {
            constant_folding(ir_ast);
            sign_adjust(ir_ast);
        }
    }
    */
}
