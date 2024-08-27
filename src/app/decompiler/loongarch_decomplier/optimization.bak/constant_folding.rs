use crate::loongarch_decomplier::*;

pub fn constant_folding(ir_ast: &mut AbstractSyntaxTree) -> Box<AbstractSyntaxTree> {
    for ast in ir_ast.next.iter_mut() {
        if let ASTType::Assign = ast.ast_type {
            for next in ast.next.iter_mut() {
                constant_folding(next);
            }
        } else if let ASTType::Operator(op) = ast.ast_type.clone() {
            let operand1 = match ast.next[0].ast_type {
                ASTType::Integer(_) => ast.next[0].clone(),
                ASTType::Operator(_) => constant_folding(ast),
                ASTType::Variable(is_ptr) => Box::new(AbstractSyntaxTree::new()),
                _ => panic!(""),
            };
            let operand2 = match ast.next[1].ast_type {
                ASTType::Integer(_) => ast.next[1].clone(), 
                ASTType::Operator(_) => constant_folding(ast),
                ASTType::Variable(is_ptr) => Box::new(AbstractSyntaxTree::new()),
                _ => panic!(""),
            };

            if let ASTType::Integer(s1) = operand1.ast_type {
                if let ASTType::Integer(s2) = operand2.ast_type {
                    let value1 = operand1.value;
                    let value2 = operand2.value;

                    if s1 != s2 {
                        panic!("signed error");
                    }
                    
                    let value = match op {
                        Operator::Not => panic!("not error"),
                        Operator::Xor => {
                            value1 ^ value2
                        }
                        Operator::And => {
                            value1 & value2
                        }
                        Operator::Div => {
                            (value1 as isize / value2 as isize) as usize
                        }
                        Operator::Mul => {
                            (value1 as isize * value2 as isize) as usize
                        }
                        Operator::Sub => {
                            (value1 as isize - value2 as isize) as usize
                        }
                        Operator::Add => {
                            (value1 as isize + value2 as isize) as usize
                        }
                        Operator::Or => {
                            value1 | value2
                        }
                    };

                    *ast = Box::new(AbstractSyntaxTree {
                        ast_type: ASTType::Integer(s1),
                        value,
                        next: Vec::new(),
                    });

                    return Box::new(*ast.clone());
                } else {
                    return Box::new(AbstractSyntaxTree::new());
                }
            } else {
                return Box::new(AbstractSyntaxTree::new());
            }
        }
    } 
    Box::new(AbstractSyntaxTree::new())
}

/*
pub fn constant_folding(ast: &mut AbstractSyntaxTree) {
    for ir_ast in ast.next.iter_mut() {
        if let ASTType::Assign = ir_ast.ast_type {
            if let Some(opcode_ast) = ir_ast.next.first() {
                let mut is_all_constant = true;
                let mut signed = true;
                for operand_ast in opcode_ast.next.iter() {
                    if let ASTType::Integer(s) = operand_ast.ast_type {
                        signed = s;
                    } else {
                        is_all_constant = false;
                        break;
                    }
                }
                if !is_all_constant {
                    continue;
                }
                let mut skip = false;
                let mut res = 0;
                match opcode_ast.ast_type {
                    ASTType::Operator(Operator::Or) => {
                        for (i, operand_ast) in opcode_ast.next.iter().enumerate() {
                            if let ASTType::Integer(s) = operand_ast.ast_type {
                                if s != signed {
                                    skip = true;
                                    break;
                                }
                                if i == 0 {
                                    res = operand_ast.value;
                                } else {
                                    res |= operand_ast.value;
                                } 
                            } else {
                                skip = true;
                                break;
                            }
                        }
                        if skip {
                            continue;
                        }

                        let mut value_ast = AbstractSyntaxTree::new();
                        value_ast.value = res;
                        value_ast.ast_type = ASTType::Integer(signed);

                        ir_ast.next.remove(0);
                        ir_ast.next.insert(0, Box::new(value_ast));
                    }
                    ASTType::Operator(Operator::Add) => {
                        for (i, operand_ast) in opcode_ast.next.iter().enumerate() {
                            if let ASTType::Integer(s) = operand_ast.ast_type {
                                if s != signed {
                                    skip = true;
                                    break;
                                }
                                if i == 0 {
                                    res = operand_ast.value;
                                } else {
                                    res = (res as isize + operand_ast.value as isize) as usize;
                                } 
                            } else {
                                skip = true;
                                break;
                            }
                        }
                        if skip {
                            continue;
                        }

                        let mut value_ast = AbstractSyntaxTree::new();
                        value_ast.value = res;
                        value_ast.ast_type = ASTType::Integer(signed);

                        ir_ast.next.remove(0);
                        ir_ast.next.insert(0, Box::new(value_ast));
                    }
                    ASTType::Operator(Operator::Sub) => {
                        for (i, operand_ast) in opcode_ast.next.iter().enumerate() {
                            if let ASTType::Integer(s) = operand_ast.ast_type {
                                if s != signed {
                                    skip = true;
                                    break;
                                }
                                if i == 0 {
                                    res = operand_ast.value;
                                } else {
                                    res = (res as isize - operand_ast.value as isize) as usize;
                                } 
                            } else {
                                skip = true;
                                break;
                            }
                        }
                        if skip {
                            continue;
                        }

                        let mut value_ast = AbstractSyntaxTree::new();
                        value_ast.value = res;
                        value_ast.ast_type = ASTType::Integer(signed);

                        ir_ast.next.remove(0);
                        ir_ast.next.insert(0, Box::new(value_ast))
                    }
                    ASTType::Operator(Operator::Mul) => {
                        for (i, operand_ast) in opcode_ast.next.iter().enumerate() {
                            if let ASTType::Integer(s) = operand_ast.ast_type {
                                if s != signed {
                                    skip = true;
                                    break;
                                }
                                if i == 0 {
                                    res = operand_ast.value;
                                } else {
                                    res = (res as isize * operand_ast.value as isize) as usize;
                                } 
                            } else {
                                skip = true;
                                break;
                            }
                        }
                        if skip {
                            continue;
                        }

                        let mut value_ast = AbstractSyntaxTree::new();
                        value_ast.value = res;
                        value_ast.ast_type = ASTType::Integer(signed);

                        ir_ast.next.remove(0);
                        ir_ast.next.insert(0, Box::new(value_ast))
                    }
                    ASTType::Operator(Operator::Div) => {
                        for (i, operand_ast) in opcode_ast.next.iter().enumerate() {
                            if let ASTType::Integer(s) = operand_ast.ast_type {
                                if s != signed {
                                    skip = true;
                                    break;
                                }
                                if i == 0 {
                                    res = operand_ast.value;
                                } else {
                                    res = (res as isize / operand_ast.value as isize) as usize;
                                } 
                            } else {
                                skip = true;
                                break;
                            }
                        }
                        if skip {
                            continue;
                        }

                        let mut value_ast = AbstractSyntaxTree::new();
                        value_ast.value = res;
                        value_ast.ast_type = ASTType::Integer(signed);

                        ir_ast.next.remove(0);
                        ir_ast.next.insert(0, Box::new(value_ast))
                    }
                    ASTType::Operator(Operator::And) => {
                        for (i, operand_ast) in opcode_ast.next.iter().enumerate() {
                            if let ASTType::Integer(s) = operand_ast.ast_type {
                                if s != signed {
                                    skip = true;
                                    break;
                                }
                                if i == 0 {
                                    res = operand_ast.value;
                                } else {
                                    res = (res as isize & operand_ast.value as isize) as usize;
                                } 
                            } else {
                                skip = true;
                                break;
                            }
                        }
                        if skip {
                            continue;
                        }

                        let mut value_ast = AbstractSyntaxTree::new();
                        value_ast.value = res;
                        value_ast.ast_type = ASTType::Integer(signed);

                        ir_ast.next.remove(0);
                        ir_ast.next.insert(0, Box::new(value_ast))
                    }
                    ASTType::Operator(Operator::Xor) => {
                        for (i, operand_ast) in opcode_ast.next.iter().enumerate() {
                            if let ASTType::Integer(s) = operand_ast.ast_type {
                                if s != signed {
                                    skip = true;
                                    break;
                                }
                                if i == 0 {
                                    res = operand_ast.value;
                                } else {
                                    res = (res as isize ^ operand_ast.value as isize) as usize;
                                } 
                            } else {
                                skip = true;
                                break;
                            }
                        }
                        if skip {
                            continue;
                        }

                        let mut value_ast = AbstractSyntaxTree::new();
                        value_ast.value = res;
                        value_ast.ast_type = ASTType::Integer(signed);

                        ir_ast.next.remove(0);
                        ir_ast.next.insert(0, Box::new(value_ast))
                    }
                    _ => {}
                }
            }
        }
    }
}
*/
