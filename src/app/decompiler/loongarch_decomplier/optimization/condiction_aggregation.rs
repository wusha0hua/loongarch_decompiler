//use crate::loongarch_decomplier::*;
use crate::app::decompiler::loongarch_decomplier::*;

pub fn condiction_aggregation(ast: &mut AbstractSyntaxTree) {
    match &ast.ast_type {
        ASTType::If => {
            let mut can_aggregation = false;
            let mut exist_true = false;
            let mut exist_false = false;
            for next in ast.next.iter() {
                match &next.ast_type {
                    ASTType::True => exist_true = true,
                    ASTType::False => exist_false = true,
                    _ => {}
                }
            }

            if exist_true && !exist_false {
                for outer_if_next in ast.next.iter() {
                    if outer_if_next.ast_type == ASTType::True {
                        if outer_if_next.next.len() == 1 {
                            let first_ast = outer_if_next.next.first().unwrap();
                            if first_ast.ast_type == ASTType::If {
                                let mut exist_true = false;
                                let mut exist_false = false;

                                for inner_if_next in first_ast.next.iter() {
                                    match &inner_if_next.ast_type {
                                        ASTType::True => exist_true = true,
                                        ASTType::False => exist_false = true,
                                        _ => {}
                                    }
                                }

                                if exist_true && !exist_false {
                                    can_aggregation = true;
                                }
                            }
                        }
                    }
                }
            }

            if can_aggregation {
                let mut outer_true_index = 0;
                let mut outer_conds_index = 0;
                let mut outer_condictions = AbstractSyntaxTree::new();
                for (i, next) in ast.next.iter().enumerate() {
                    match &next.ast_type {
                        ASTType::Condictions => {
                            outer_conds_index = i;
                            outer_condictions = *next.clone(); 
                        }                    
                        ASTType::True => {
                            outer_true_index = i;
                        }
                        _ => {}
                    }
                }

                let mut inner_true = AbstractSyntaxTree::new();
                let mut inner_condictions = AbstractSyntaxTree::new();

                for true_ast in ast.next.iter() {
                    if true_ast.ast_type == ASTType::True {
                        let inner_if_ast = true_ast.next.first().unwrap();
                        for inner_next in inner_if_ast.next.iter() {
                            match &inner_next.ast_type {
                                ASTType::Condictions => {
                                    inner_condictions = *inner_next.clone();
                                }
                                ASTType::True => {
                                    inner_true = *inner_next.clone();
                                }
                                _ => {}
                            }
                        }
                        break;
                    }
                }

                and_condictions(&mut outer_condictions, &inner_condictions);

                ast.next.remove(outer_conds_index);
                ast.next.insert(outer_conds_index, Box::new(outer_condictions));

                ast.next.remove(outer_true_index);
                ast.next.insert(outer_true_index, Box::new(inner_true));

                condiction_aggregation(ast);
            }
            
            for next in ast.next.iter_mut() {
                condiction_aggregation(next);
            }
            //condiction_aggregation(ast);
        }
        _ => {
            for next in ast.next.iter_mut() {
                condiction_aggregation(next);
            }
        }
    }
}

pub fn and_condictions(conds1: &mut AbstractSyntaxTree, conds2: &AbstractSyntaxTree) {
    match &conds1.ast_type {
        ASTType::Condictions | ASTType::Operator(_) => {
            let mut exist_operator = false;
            for next in conds1.next.iter() {
                if let ASTType::Operator(_) = &next.ast_type {
                    exist_operator = true;
                }
            }

            if exist_operator {
                for next in conds1.next.iter_mut() {
                    and_condictions(next, conds2);
                }
            } else {
                let mut and_ast = AbstractSyntaxTree::new();
                and_ast.ast_type = ASTType::Operator(Operator::And);

                let mut last_cond = conds1.next.last().unwrap(); 
                and_ast.next.push(Box::new(*last_cond.clone()));
                let mut first_cond = conds2.next.first().unwrap();
                and_ast.next.push(Box::new(*first_cond.clone()));

                conds1.next.pop();
                conds1.next.push(Box::new(and_ast));
            }
        }
        _ => {}
    }
    /*
    for next in conds1.next.iter_mut() {
        match &next.ast_type {
            ASTType::Operator(_) => {
                let mut exist_operator = false;
                for operator_next in next.next.iter_mut() {
                    if let ASTType::Operator(_) = &operator_next.ast_type {
                        exist_operator = true;
                    }
                }

                if exist_operator {

                }
            }
            _ => {}
        }
    } 
    */
}

/*
pub fn condiction_aggregation(ast: &mut AbstractSyntaxTree) {
    match &ast.ast_type {
        ASTType::If => {
            let mut can_aggregation = false;
            let mut exist_true = false;
            let mut exist_false = false;
            for next in ast.next.iter() {
                if next.ast_type == ASTType::True {
                    exist_true = true;
                } else if next.ast_type == ASTType::False {
                    exist_false = true;
                }
            }
    
            if exist_true == true && exist_false == false {
                for outer_if_next in ast.next.iter() {
                    if outer_if_next.ast_type == ASTType::True {
                        if outer_if_next.next.len() == 1 {
                            let first_ast = outer_if_next.next.first().unwrap();        
                            if first_ast.ast_type == ASTType::If {
                                let mut exist_true = false;
                                let mut exist_false = false;
                                for inner_if_next in first_ast.next.iter() {
                                    if inner_if_next.ast_type == ASTType::True {
                                        exist_true = true;
                                    } else if inner_if_next.ast_type == ASTType::False {
                                        exist_false = true;
                                    }
                                }
                                if exist_true == true && exist_false == false {
                                    can_aggregation = true;
                                }
                            }
                        }
                    }
                }
            }         

            if can_aggregation {

                let mut outer_true_index = 0;
                let mut outer_cond_index = 0;
                let mut outer_condiction = AbstractSyntaxTree::new();
                for (i, next) in ast.next.iter().enumerate() {
                    if let ASTType::Condiction(_) = &next.ast_type {
                        outer_cond_index = i;
                        outer_condiction = *next.clone();
                    } else if let ASTType::True = &next.ast_type {
                        outer_true_index = i;
                    }
                }
                
                let mut inner_true = AbstractSyntaxTree::new();
                let mut inner_condiction = AbstractSyntaxTree::new();

                for true_ast in ast.next.iter() {
                    if true_ast.ast_type == ASTType::True {
                        let inner_if_ast = true_ast.next.first().unwrap();
                        for inner_next in inner_if_ast.next.iter() {
                            if let ASTType::Condiction(_) = inner_next.ast_type {
                                inner_condiction = *inner_next.clone();
                            } else if inner_next.ast_type == ASTType::True {
                                inner_true = *inner_next.clone();
                            }
                        }
                        break;
                    } 
                }

                let condictions = and_condictions(outer_condiction, inner_condiction);
                
                ast.next.remove(outer_cond_index);
                ast.next.insert(outer_cond_index, Box::new(condictions));

                ast.next.remove(outer_true_index);
                ast.next.insert(outer_true_index, Box::new(inner_true));
            }

            //println!("{:?}", ast);
            //println!("exist_true: {}, exist_false: {}", exist_true, exist_false);
            /*
            if exist_true == true && exist_false == false {
                for next in ast.next.iter_mut() {
                    if next.ast_type == ASTType::True {
                        if next.next.len() == 1 {
                            if next.next.first().unwrap().ast_type == ASTType::If {
                                let mut exist_true = false;
                                let mut exist_false = false;
                                for next in next.next.first().unwrap().next.iter() {
                                    if next.ast_type == ASTType::True {
                                        exist_true = true;
                                    } else {
                                        exist_false = true;
                                    }
                                }
                                if exist_true == true && exist_false == false {
                                    println!("ok");
                                }
                            }
                        } 
                        break;
                    }
                }
            }
            */

            for next in ast.next.iter_mut() {
                condiction_aggregation(next);
            }
        }
        _ => {
            for next in ast.next.iter_mut() {
                condiction_aggregation(next);
            } 
        }
    }
}


pub fn and_condictions(condiction1: AbstractSyntaxTree, condiction2: AbstractSyntaxTree) -> AbstractSyntaxTree {
    let mut condictions = AbstractSyntaxTree::new();
    condictions.ast_type = ASTType::Condictions;

    let mut and_ast = AbstractSyntaxTree::new();
    and_ast.ast_type = ASTType::Operator(Operator::And);

    and_ast.next.push(Box::new(condiction1));
    and_ast.next.push(Box::new(condiction2));

    condictions.next.push(Box::new(and_ast));

    condictions
}
*/
