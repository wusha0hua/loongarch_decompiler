use crate::loongarch_decomplier::*;

pub fn sign_adjust(ast: &mut AbstractSyntaxTree) {
    if let ASTType::Assign = ast.ast_type {
        for next in ast.next.iter_mut() {
            sign_adjust(next);
        }
    } else if let ASTType::Operator(op) = ast.ast_type.clone() {
        let mut operand1 = ast.next[0].clone();
        let mut operand2 = ast.next[1].clone();

        match &operand1.ast_type {
            ASTType::Integer(sign1) => {
                match &operand2.ast_type {
                    ASTType::Integer(sign2) => {
                        if (*sign1 == true) && ((operand1.value as isize) < 0) && (*sign2 == true) && ((operand2.value as isize) >= 0) {
                            match op {
                                Operator::Add => {
                                    ast.next.clear();
                                    ast.ast_type = ASTType::Operator(Operator::Sub);
                                    ast.next.push(operand2);
                                    operand1.value = isize::abs(operand1.value as isize) as usize;
                                    ast.next.push(operand1);
                                }
                                Operator::Sub => {
                                    ast.next.clear();
                                    ast.ast_type = ASTType::Operator(Operator::Add);
                                    ast.next.push(operand2);
                                    operand1.value = isize::abs(operand1.value as isize) as usize;
                                    ast.next.push(operand1);
                                }
                                _ => {}
                            }
                           
                        }
                    }
                    ASTType::Variable(is_ptr) => {
                        if (*sign1 == true) && ((operand1.value as isize) < 0) {
                            ast.next.clear();
                            operand1.value = isize::abs(operand1.value as isize) as usize;
                            ast.next.push(operand2); 
                            ast.next.push(operand1);
                            match op {
                                Operator::Add => {

                                    ast.ast_type = ASTType::Operator(Operator::Sub);
                                }
                                Operator::Sub => {

                                    ast.ast_type = ASTType::Operator(Operator::Add);
                                }
                                _ => {}
                            }
                        }
                    }
                    ASTType::Operator(op2) => {
                        if (*sign1 == true) && ((operand1.value as isize) < 0) {
                            ast.next.clear();
                            operand1.value = isize::abs(operand1.value as isize) as usize;
                            ast.next.push(operand2); 
                            ast.next.push(operand1);

                            match op {
                                Operator::Add => {
                                    ast.ast_type = ASTType::Operator(Operator::Sub);
                                }
                                Operator::Sub => {
                                    ast.ast_type = ASTType::Operator(Operator::Add);
                                }
                                _ => {}
                            }
                            
                            sign_adjust(ast.next[0].as_mut());
                        } else {
                            sign_adjust(ast.next[0].as_mut());
                        }
                    }
                    _ => panic!("error"),
                }
            }
            ASTType::Variable(is_ptr) => {
                match &operand2.ast_type {
                    ASTType::Integer(sign2) => {
                        if (*sign2 == true) && ((operand2.value as isize) < 0) {
                            match op {
                                Operator::Add => {
                                    ast.next[1].value = isize::abs(operand2.value as isize) as usize;
                                    ast.ast_type = ASTType::Operator(Operator::Sub);
                                }
                                Operator::Sub => {
                                    ast.next[1].value = isize::abs(operand2.value as isize) as usize;
                                    ast.ast_type = ASTType::Operator(Operator::Add);
                                }
                                _ => {}
                            }
                        }
                    }
                    ASTType::Variable(is_ptr) => {
                    }
                    ASTType::Operator(op2) => {
                        sign_adjust(ast.next[1].as_mut());
                    }
                    _ => panic!("error"),
                }
            }
            ASTType::Operator(op1) => {
                match &operand2.ast_type {
                    ASTType::Integer(sign2) => {
                        if (*sign2 == true) && ((operand2.value as isize) < 0) {
                            match op {
                                Operator::Add => {
                                    ast.ast_type = ASTType::Operator(Operator::Sub);
                                    ast.next[1].value = isize::abs(operand2.value as isize) as usize;
                                }
                                Operator::Sub => {
                                    ast.ast_type = ASTType::Operator(Operator::Add);
                                    ast.next[1].value = isize::abs(operand2.value as isize) as usize;
                                }
                                _ => {}
                            }
                        } else {
                            sign_adjust(ast.next[0].as_mut());
                        }
                    }
                    ASTType::Variable(is_ptr) => {
                        sign_adjust(ast.next[0].as_mut());
                    }
                    ASTType::Operator(op2) => {
                        panic!("operand error");
                    }
                    _ => panic!("error"),
                }
            }
            _ => panic!("error"),
        }
    }


}

/*
pub fn sign_adjust(ast: &mut AbstractSyntaxTree) {
    if let ASTType::Assign = ast.ast_type {
        for next in ast.next.iter_mut() {
            sign_adjust(next);
        }
    } else if let ASTType::Operator(op) = ast.ast_type.clone() {
        let mut opreand1 = ast.next[0].clone();
        let mut opreand2 = ast.next[1].clone();

        if let ASTType::Integer(sign1) = &opreand1.ast_type {
            if let ASTType::Integer(sign2) = &opreand2.ast_type {
                if (*sign1 == true) && ((opreand1.value as isize) < 0) && (*sign2 == true) && ((opreand2.value as isize) >= 0) {
                    match op {
                        Operator::Add => {
                            ast.next.clear();
                            ast.ast_type = ASTType::Operator(Operator::Sub);
                            ast.next.push(opreand2);
                            opreand1.value = isize::abs(opreand1.value as isize) as usize;
                            ast.next.push(opreand1);
                        }
                        Operator::Sub => {
                            ast.next.clear();
                            ast.ast_type = ASTType::Operator(Operator::Add);
                            ast.next.push(opreand2);
                            opreand1.value = isize::abs(opreand1.value as isize) as usize;
                            ast.next.push(opreand1);
                        }
                        _ => {}
                        /*
                        Operator::Mul => {}
                        Operator::Div => {}
                        Operator::And => {}
                        Operator::Or => {}
                        Operator::Xor => {}
                        Operator::Not => {panic!("not error");}
                        */
                    }
                }
            } else if let ASTType::Operator(op) = &opreand2.ast_type {
                if (*sign1 == true) && ((opreand1.value as isize) < 0) {
                    if let Operator::Add = op {
                        ast.next.clear();
                        ast.ast_type = ASTType::Operator(Operator::Sub);
                        opreand1.value = isize::abs(opreand1.value as isize) as usize;
                        ast.next.push(opreand2);
                        ast.next.push(opreand1);
                    } else if let Operator::Sub = op {
                        ast.next.clear();
                        ast.ast_type = ASTType::Operator(Operator::Add);
                        opreand1.value = isize::abs(opreand1.value as isize) as usize;
                        ast.next.push(opreand2);
                        ast.next.push(opreand1);
                    } else {
                        ast.next.clear();
                        ast.next.push(opreand2);
                        ast.next.push(opreand1);
                    }
                } 
            } else {
                panic!("opreand error");
            }
        } else if let ASTType::Operator(op) = &opreand1.ast_type {
            if let ASTType::Integer(sign2) = &opreand2.ast_type {
                if (*sign2 == true) && ((opreand2.value as isize) < 0) {
                    if let Operator::Add = op {
                        ast.next[1].value = isize::abs(opreand2.value as isize) as usize; 
                        ast.ast_type = ASTType::Operator(Operator::Sub);
                    } else if let Operator::Sub = op {
                        ast.next[1].value = isize::abs(opreand2.value as isize) as usize; 
                        ast.ast_type = ASTType::Operator(Operator::Add);
                    } 
                } 
            } else if let ASTType::Operator(op) = &opreand2.ast_type {
            } else {
                panic!("opreand error");
            }
        } else {
            panic!("opreand error");
        }
    } 
}
*/
