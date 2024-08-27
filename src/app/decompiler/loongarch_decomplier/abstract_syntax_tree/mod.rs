mod ast_symbol;
mod type_fix;
mod assign_refined;

pub use ast_symbol::*;
pub use type_fix::*;
pub use assign_refined::*;

//use crate::{loongarch_decomplier::*, NameValue};
//use crate::loongarch_decomplier::data_flow;
use crate::app::decompiler::{loongarch_decomplier::*, NameValue};
use crate::app::decompiler::loongarch_decomplier::data_flow; 

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractSyntaxTree { 
    pub ast_type: ASTType, 
    pub value: u64,
    pub next: Vec<Box<AbstractSyntaxTree>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTType {
    Begin(String),
    FunctionReturn(ASTSymbolValueType, bool),
    EndReturn,
    Function(String),
    Parameter,
    Return,
    Loop,
    Break,
    Continue,
    Assign(bool),
    If,
    Condiction(Relation),
    Condictions,
    True,
    False,
    Operator(Operator),
    Variable(bool),
    Integer(bool, ASTSymbolValueType),
    Float(bool, ASTSymbolValueType),
    While,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
    Not,
}

impl AbstractSyntaxTree {
    pub fn new() -> Self {
        Self {
            ast_type: ASTType::Begin(String::from("None")),
            value: 0,
            next: Vec::new(),
        }
    }

    pub fn from_cfg_tree(cft: &ControlFlowTree, cfg: &ControlFlowGraph, function_name: String, ast_symbol_map: &mut HashMap<usize, ASTSymbol>) -> Self {
        let mut ast = AbstractSyntaxTree::new();
        ast.ast_type = ASTType::Begin(function_name);
        //let mut ast_symbol_map = HashMap::<usize, ASTSymbol>::new();
        
        if SHOW_CONTROL_FLOW_IN_DATA_FLOW_IR.load(std::sync::atomic::Ordering::SeqCst) {
            println!("\n---------- control flow in data flow ir ----------");
        }

        let mut counter = Counter::new();
        let mut address_symbol_map = HashMap::<(Address, usize), usize>::new();
        let mut params = Vec::<AbstractSyntaxTree>::new();
        cft.travel_cft_with_ast(&mut ast, cfg, ast_symbol_map, &mut address_symbol_map, &mut counter, &mut params); 
        //updata_type(&mut ast, ast_symbol_map);
        //params.reverse();
        let mut params_ast = AbstractSyntaxTree::new();
        params_ast.ast_type = ASTType::Parameter;
        for param in params {
            params_ast.next.push(Box::new(param)); 
        }
        ast.next.insert(0, Box::new(params_ast));
        if SHOW_CONTROL_FLOW_IN_DATA_FLOW_IR.load(std::sync::atomic::Ordering::SeqCst) {
            println!("---------------------------------------------------\n");
        }
        
        optimization::updata_type(&mut ast, ast_symbol_map);
        if OPTIMIZATION.load(std::sync::atomic::Ordering::SeqCst) {
            optimization(&mut ast, &ast_symbol_map);
        }

        let mut vec = ast_symbol_map.iter().collect::<Vec<(&usize, &ASTSymbol)>>();
        vec.sort_by(|a,b | a.0.cmp(&b.0));
        for v in vec.iter() {
            //println!("{:?}", v);
        }

       
        ast
    }

    pub fn parse_condiction(condiction: Condiction, address_symbol_map: &mut HashMap<(Address, usize), usize>, ast_symbol_map: &mut HashMap<usize, ASTSymbol>, counter: &mut Counter) -> AbstractSyntaxTree {
        let mut ast = AbstractSyntaxTree::new();
        ast.ast_type = ASTType::Condiction(condiction.relation);

        let mut ast1 = AbstractSyntaxTree::new();
        let mut ast2 = AbstractSyntaxTree::new();

        match &condiction.operand1 {
            DFIOperand::Number(number) => {
                ast1.value = number.value as u64;
                ast1.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
            }
            DFIOperand::Symbol(symbol) => {
                let ast_sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                ast1.value = ast_sym.id as u64;
                ast1.ast_type = ASTType::Variable(symbol.value);
            }
            _ => panic!("error"),
        }

        ast.next.push(Box::new(ast1));

        match &condiction.operand2 {
            DFIOperand::Number(number) => {
                ast2.value = number.value as u64;
                ast2.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
            }
            DFIOperand::Symbol(symbol) => {
                let ast_sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                ast2.value = ast_sym.id as u64;
                ast2.ast_type = ASTType::Variable(symbol.value);
            }
            _ => panic!("error"),
        }
        
        ast.next.push(Box::new(ast2));

        ast
    }

    pub fn parse_access_condiction(condiction: Vec<Vec<Condiction>>, address_symbol_map: &mut HashMap<(Address, usize), usize>, ast_symbol_map: &mut HashMap<usize, ASTSymbol>, counter: &mut Counter) -> AbstractSyntaxTree {
        let mut ast = AbstractSyntaxTree::new();
        ast.ast_type = ASTType::Condictions;
        let mut or_ast = AbstractSyntaxTree::new();
        or_ast.ast_type = ASTType::Operator(Operator::Or);
        for andcv in condiction {
            let mut and_ast = AbstractSyntaxTree::new();
            and_ast.ast_type = ASTType::Operator(Operator::And);
            for c in andcv {
                let cond_ast = AbstractSyntaxTree::parse_condiction(c, address_symbol_map, ast_symbol_map, counter); 
                and_ast.next.push(Box::new(cond_ast));
            }
            or_ast.next.push(Box::new(and_ast));
        }        
        ast.next.push(Box::new(or_ast));
        ast
    }


    pub fn parse_dfi(ir: &data_flow::DataFlowIr, ast_symbol_map: &mut HashMap<usize, ASTSymbol>, address_symbol_map: &mut HashMap<(Address, usize), usize>, counter: &mut Counter) -> (Option<AbstractSyntaxTree>, Option<AbstractSyntaxTree>) {
        let mut ast = AbstractSyntaxTree::new();
        let mut ast_operand1 = AbstractSyntaxTree::new();
        let mut ast_operand2 = AbstractSyntaxTree::new();
        let mut ast_operator = AbstractSyntaxTree::new();
        
        match &ir.opcode {
            DataFlowIrOpcode::Add => {
                let mut ast = AbstractSyntaxTree::new();
                let mut operator = AbstractSyntaxTree::new();
                let mut ast1 = AbstractSyntaxTree::new();
                let mut ast2 = AbstractSyntaxTree::new();

                if let Some(DFIOperand::Symbol(symbol1)) = &ir.operand1 {
                    let mut sym = create_symbol(symbol1, address_symbol_map, ast_symbol_map, counter);
                    ast.ast_type = ASTType::Assign(symbol1.value);
                    ast.value = sym.id as u64;
                } else {
                    panic!("error");
                }

                match &ir.operand2 {
                    Some(DFIOperand::Number(number)) => {
                        ast1.value = number.value as u64;
                        ast1.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast1.value = sym.id as u64;
                        ast1.ast_type = ASTType::Variable(symbol.value);
                    }
                    _ => panic!("error"),
                }

                match &ir.operand3 {
                    Some(DFIOperand::Number(number)) => {
                        ast2.value = number.value as u64;
                        ast2.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast2.value = sym.id as u64;
                        ast2.ast_type = ASTType::Variable(symbol.value);
                    }
                    _ => panic!("error"),
                }

                operator.ast_type = ASTType::Operator(Operator::Add);
                operator.next.push(Box::new(ast1));
                operator.next.push(Box::new(ast2));
                ast.next.push(Box::new(operator));


                (Some(ast), None)
            }
            DataFlowIrOpcode::Sub => {
                let mut ast = AbstractSyntaxTree::new();
                let mut operator = AbstractSyntaxTree::new();
                let mut ast1 = AbstractSyntaxTree::new();
                let mut ast2 = AbstractSyntaxTree::new();

                if let Some(DFIOperand::Symbol(symbol1)) = &ir.operand1 {
                    let mut sym = create_symbol(symbol1, address_symbol_map, ast_symbol_map, counter);
                    ast.ast_type = ASTType::Assign(symbol1.value);
                    ast.value = sym.id as u64;
                } else {
                    panic!("error");
                }

                match &ir.operand2 {
                    Some(DFIOperand::Number(number)) => {
                        ast1.value = number.value as u64;
                        ast1.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast1.value = sym.id as u64;
                        ast1.ast_type = ASTType::Variable(symbol.value);
                    }
                    _ => panic!("error"),
                }

                match &ir.operand3 {
                    Some(DFIOperand::Number(number)) => {
                        ast2.value = number.value as u64;
                        ast2.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast2.value = sym.id as u64;
                        ast2.ast_type = ASTType::Variable(symbol.value);
                    }
                    _ => panic!("error"),
                }

                operator.ast_type = ASTType::Operator(Operator::Sub);
                operator.next.push(Box::new(ast1));
                operator.next.push(Box::new(ast2));
                ast.next.push(Box::new(operator));

                (Some(ast), None)
            }
            DataFlowIrOpcode::Mul => {
                let mut ast = AbstractSyntaxTree::new();
                let mut operator = AbstractSyntaxTree::new();
                let mut ast1 = AbstractSyntaxTree::new();
                let mut ast2 = AbstractSyntaxTree::new();

                if let Some(DFIOperand::Symbol(symbol1)) = &ir.operand1 {
                    let mut sym = create_symbol(symbol1, address_symbol_map, ast_symbol_map, counter);
                    ast.ast_type = ASTType::Assign(symbol1.value);
                    ast.value = sym.id as u64;
                } else {
                    panic!("error");
                }

                match &ir.operand2 {
                    Some(DFIOperand::Number(number)) => {
                        ast1.value = number.value as u64;
                        ast1.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast1.value = sym.id as u64;
                        ast1.ast_type = ASTType::Variable(symbol.value);
                    }
                    _ => panic!("error"),
                }

                match &ir.operand3 {
                    Some(DFIOperand::Number(number)) => {
                        ast2.value = number.value as u64;
                        ast2.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast2.value = sym.id as u64;
                        ast2.ast_type = ASTType::Variable(symbol.value);
                    }
                    _ => panic!("error"),
                }

                operator.ast_type = ASTType::Operator(Operator::Mul);
                operator.next.push(Box::new(ast1));
                operator.next.push(Box::new(ast2));
                ast.next.push(Box::new(operator));

                (Some(ast), None)
            }
            DataFlowIrOpcode::Div => {
                let mut ast = AbstractSyntaxTree::new();
                let mut operator = AbstractSyntaxTree::new();
                let mut ast1 = AbstractSyntaxTree::new();
                let mut ast2 = AbstractSyntaxTree::new();

                if let Some(DFIOperand::Symbol(symbol1)) = &ir.operand1 {
                    let mut sym = create_symbol(symbol1, address_symbol_map, ast_symbol_map, counter);
                    ast.ast_type = ASTType::Assign(symbol1.value);
                    ast.value = sym.id as u64;
                } else {
                    panic!("error");
                }

                match &ir.operand2 {
                    Some(DFIOperand::Number(number)) => {
                        ast1.value = number.value as u64;
                        ast1.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast1.value = sym.id as u64;
                        ast1.ast_type = ASTType::Variable(symbol.value);
                    }
                    _ => panic!("error"),
                }

                match &ir.operand3 {
                    Some(DFIOperand::Number(number)) => {
                        ast2.value = number.value as u64;
                        ast2.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast2.value = sym.id as u64;
                        ast2.ast_type = ASTType::Variable(symbol.value);
                    }
                    _ => panic!("error"),
                }

                operator.ast_type = ASTType::Operator(Operator::Div);
                operator.next.push(Box::new(ast1));
                operator.next.push(Box::new(ast2));
                ast.next.push(Box::new(operator));

                (Some(ast), None)
            }
            DataFlowIrOpcode::And => {                
                let mut ast = AbstractSyntaxTree::new();
                let mut operator = AbstractSyntaxTree::new();
                let mut ast1 = AbstractSyntaxTree::new();
                let mut ast2 = AbstractSyntaxTree::new();

                if let Some(DFIOperand::Symbol(symbol1)) = &ir.operand1 {
                    let mut sym = create_symbol(symbol1, address_symbol_map, ast_symbol_map, counter);
                    ast.ast_type = ASTType::Assign(symbol1.value);
                    ast.value = sym.id as u64;
                } else {
                    panic!("error");
                }

                match &ir.operand2 {
                    Some(DFIOperand::Number(number)) => {
                        ast1.value = number.value as u64;
                        ast1.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast1.value = sym.id as u64;
                        ast1.ast_type = ASTType::Variable(symbol.value);
                    }
                    _ => panic!("error"),
                }

                match &ir.operand3 {
                    Some(DFIOperand::Number(number)) => {
                        ast2.value = number.value as u64;
                        ast2.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast2.value = sym.id as u64;
                        ast2.ast_type = ASTType::Variable(symbol.value);
                    }
                    _ => panic!("error"),
                }

                operator.ast_type = ASTType::Operator(Operator::And);
                operator.next.push(Box::new(ast1));
                operator.next.push(Box::new(ast2));
                ast.next.push(Box::new(operator));


                (Some(ast), None)
            }
            DataFlowIrOpcode::Or => {
                let mut ast = AbstractSyntaxTree::new();
                let mut operator = AbstractSyntaxTree::new();
                let mut ast1 = AbstractSyntaxTree::new();
                let mut ast2 = AbstractSyntaxTree::new();

                if let Some(DFIOperand::Symbol(symbol1)) = &ir.operand1 {
                    let mut sym = create_symbol(symbol1, address_symbol_map, ast_symbol_map, counter);
                    ast.ast_type = ASTType::Assign(symbol1.value);
                    ast.value = sym.id as u64;
                } else {
                    panic!("error");
                }

                match &ir.operand2 {
                    Some(DFIOperand::Number(number)) => {
                        ast1.value = number.value as u64;
                        ast1.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast1.value = sym.id as u64;
                        ast1.ast_type = ASTType::Variable(symbol.value);
                    }
                    _ => panic!("error"),
                }

                match &ir.operand3 {
                    Some(DFIOperand::Number(number)) => {
                        ast2.value = number.value as u64;
                        ast2.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast2.value = sym.id as u64;
                        ast2.ast_type = ASTType::Variable(symbol.value);
                    }
                    _ => panic!("error"),
                }

                operator.ast_type = ASTType::Operator(Operator::Or);
                operator.next.push(Box::new(ast1));
                operator.next.push(Box::new(ast2));
                ast.next.push(Box::new(operator));

                (Some(ast), None)
            }
            DataFlowIrOpcode::Xor => {
                let mut ast = AbstractSyntaxTree::new();
                let mut operator = AbstractSyntaxTree::new();
                let mut ast1 = AbstractSyntaxTree::new();
                let mut ast2 = AbstractSyntaxTree::new();

                if let Some(DFIOperand::Symbol(symbol1)) = &ir.operand1 {
                    let mut sym = create_symbol(symbol1, address_symbol_map, ast_symbol_map, counter);
                    ast.ast_type = ASTType::Assign(symbol1.value);
                    ast.value = sym.id as u64;
                } else {
                    panic!("error");
                }

                match &ir.operand2 {
                    Some(DFIOperand::Number(number)) => {
                        ast1.value = number.value as u64;
                        ast1.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast1.value = sym.id as u64;
                        ast1.ast_type = ASTType::Variable(symbol.value);
                    }
                    _ => panic!("error"),
                }

                match &ir.operand3 {
                    Some(DFIOperand::Number(number)) => {
                        ast2.value = number.value as u64;
                        ast2.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast2.value = sym.id as u64;
                        ast2.ast_type = ASTType::Variable(symbol.value);
                    }
                    _ => panic!("error"),
                }

                operator.ast_type = ASTType::Operator(Operator::Xor);
                operator.next.push(Box::new(ast1));
                operator.next.push(Box::new(ast2));
                ast.next.push(Box::new(operator));

                (Some(ast), None)
            }
            DataFlowIrOpcode::Not => {
                (None, None)
            }
            DataFlowIrOpcode::Call => {
                let functions = FUNCTIONS.lock().unwrap();     
                if let Some(DFIOperand::Number(number)) = &ir.operand1 {
                    if let Some(name) = functions.get(&(number.value as u64)) {
                        match name {
                            NameValue::Name(name) => ast.ast_type = ASTType::Function(name.clone()), 
                            NameValue::Value(value) => ast.ast_type = ASTType::Function(format!("funcion@{:x}", number.value)),
                        }
                    } else {
                        ast.ast_type = ASTType::Function(format!("func@{:x}", number.value));
                    }
                } else {
                    panic!("operand error");
                }

                ast_operand1.ast_type = ASTType::Return;
                match &ir.operand3 {
                    Some(DFIOperand::Symbol(symbol)) => {
                        let mut ast_symbol = match address_symbol_map.get(&(symbol.address.clone(), symbol.id)) {
                            Some(symbol) => {
                                ast_symbol_map[symbol].clone()
                            }
                            None => {
                                let sid = counter.get();
                                address_symbol_map.insert((symbol.address.clone(), symbol.id), sid);
                                let mut ast_symbol = ASTSymbol::new(sid);
                                ast_symbol.select_type = get_ast_type_from_size(&symbol.size);
                                ast_symbol
                            }
                        };

                        if let ASTType::Function(name) = &ast.ast_type {
                            match name.as_str() {
                                "printf" => {
                                    ast_symbol.select_type = ASTSymbolValueType::Int;
                                }
                                _ => {}
                            }
                        }
                        
                        
                        match &symbol.address {
                            Address::Stack(stack) => {
                                ast_symbol.scope = Scope::Local;
                            }
                            Address::Memory(memory) => {
                                ast_symbol.scope = Scope::Global;
                            }
                            Address::GR(gr) => {
                                ast_symbol.scope = Scope::Temp;   
                            }
                            Address::FR(fr) => {
                                ast_symbol.scope = Scope::Temp;
                            }
                        }

                        ast_operand1.value = ast_symbol.id as u64;
                        ast_symbol_map.insert(ast_symbol.id, ast_symbol);
                        
                    }


                    Some(DFIOperand::Number(number)) => {
                        ast_operand1.value = number.value as u64;
                        ast_operand1.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    _ => {
                        println!("{:?}", ir.operand2);
                        println!("{:?}", ir);
                        panic!("operand error");
                    }
                }

                ast.next.push(Box::new(ast_operand1));

                if let Some(DFIOperand::Parameter(parameters)) = &ir.operand2 {
                   for parameter in parameters.iter() {
                        let mut parameter_ast = AbstractSyntaxTree::new();
                        match parameter {
                            RegisterRecord::Number(number) => {
                                parameter_ast.value = number.value as u64; 
                                parameter_ast.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                            }
                            RegisterRecord::Symbol(symbol) => {
                                let mut ast_sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                                parameter_ast.ast_type = ASTType::Variable(!symbol.value);
                                parameter_ast.value = ast_sym.id as u64;
                            }
                        }
                        ast.next.push(Box::new(parameter_ast));
                   } 

                } else {
                    panic!("error");
                }

                (Some(ast), None)
            }
            DataFlowIrOpcode::Load => {
                let mut size = ASTSymbolValueType::Unknown;
                let mut symbol1_is_value = false;
                if let Some(DFIOperand::Symbol(symbol1)) = &ir.operand1 {
                    symbol1_is_value = symbol1.value;
                    let mut ast_symbol = match address_symbol_map.get(&(symbol1.address.clone(), symbol1.id)) {
                        Some(id) => {
                            ast_symbol_map[id].clone()
                        }
                        None => {
                            let sid = counter.get();
                            address_symbol_map.insert((symbol1.address.clone(), symbol1.id), sid);
                            let mut ast_symbol = ASTSymbol::new(sid);
                            ast_symbol.select_type = get_ast_type_from_size(&symbol1.size);
                            ast_symbol
                        }
                    };
        
                    size = ast_symbol.select_type.clone();
                    ast_symbol.scope = match &symbol1.address {
                        Address::Memory(_) => Scope::Global,
                        Address::Stack(_) => Scope::Local,
                        _ => Scope::Temp,
                    };

                    ast.ast_type = ASTType::Assign(symbol1.value);
                    ast.value = ast_symbol.id as u64;
                    ast_symbol_map.insert(ast_symbol.id, ast_symbol);

                }

                match &ir.operand2 {
                    Some(DFIOperand::Number(number)) => {
                        ast_operand2.value = number.value as u64;
                        ast_operand2.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                        panic!("operand error");
                    }
                    Some(DFIOperand::Symbol(symbol2)) => {
                        let mut ast_symbol = match address_symbol_map.get(&(symbol2.address.clone(), symbol2.id)) {
                            Some(id) => {
                                ast_symbol_map[id].clone()
                            }
                            None => {
                                let sid = counter.get();
                                address_symbol_map.insert((symbol2.address.clone(), symbol2.id), sid);
                                let mut ast_symbol = ASTSymbol::new(sid);
                                ast_symbol.select_type = get_ast_type_from_size(&symbol2.size);
                                ast_symbol
                            }
                        };

                        ast_symbol.scope = match &symbol2.address {
                            Address::Memory(_) => Scope::Global,
                            Address::Stack(_) => Scope::Local,
                            _ => Scope::Temp,
                        };

                        if ast_symbol.scope == Scope::Temp && (ast_symbol.select_type == ASTSymbolValueType::UnsignedLong || ast_symbol.select_type == ASTSymbolValueType::Long){
                            if symbol2.value == true && symbol1_is_value == false {
                                ast_symbol.select_type = type_change_to_ptr(&size);
                            } 
                        }

                        ast_operand2.value = ast_symbol.id as u64;
                        ast_operand2.ast_type = ASTType::Variable(symbol2.value);
                        ast_symbol_map.insert(ast_symbol.id, ast_symbol);
                    }
                    _ => {
                        println!("{:?}", ir);
                        println!("{}", ir);
                        panic!("operand error");
                    }
                }
                ast.next.push(Box::new(ast_operand2));
                    
                (Some(ast), None)
            }
            DataFlowIrOpcode::Store => {
                let mut param: Option<AbstractSyntaxTree> = None;
                let mut assign_sym_id = 0;
                let mut operand_sym_id = 0;

                let mut assign_ast_symbol = ASTSymbol::new(0);
                let mut assign_is_value = false;
                if let Some(DFIOperand::Symbol(symbol2)) = &ir.operand2 {
                    assign_is_value = symbol2.value;
                    let mut ast_symbol = match address_symbol_map.get(&(symbol2.address.clone(), symbol2.id))  {
                        Some(id) => {
                            ast_symbol_map[id].clone()
                        }
                        None => {
                            let sid = counter.get();
                            address_symbol_map.insert((symbol2.address.clone(), symbol2.id), sid);
                            let mut ast_symbol = ASTSymbol::new(sid);
                            ast_symbol.select_type = get_ast_type_from_size(&symbol2.size);
                            ast_symbol
                        }
                    };

                    ast_symbol.scope = match &symbol2.address {
                        Address::Stack(_) => Scope::Local,
                        Address::Memory(_) => Scope::Global,
                        _ => Scope::Temp,
                    };

                    assign_sym_id = ast_symbol.id as u64;
                    ast.value = ast_symbol.id as u64;
                    ast.ast_type = ASTType::Assign(symbol2.value);
                    assign_ast_symbol = ast_symbol.clone();
                    ast_symbol_map.insert(ast_symbol.id, ast_symbol);
                } else {
                    println!("{}", ir);
                    println!("{:?}", ir);
                    panic!("operand error");
                }

                match &ir.operand1 {
                    Some(DFIOperand::Number(number)) => {
                        ast_operand2.value = number.value as u64;
                        ast_operand2.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol1)) => {
                        let mut ast_symbol = match address_symbol_map.get(&(symbol1.address.clone(), symbol1.id)) {
                            Some(id) => {
                                ast_symbol_map[id].clone() 
                            }  
                            None => {
                                let sid = counter.get();
                                address_symbol_map.insert((symbol1.address.clone(), symbol1.id), sid);
                                let mut ast_symbol = ASTSymbol::new(sid);
                                ast_symbol.select_type = get_ast_type_from_size(&symbol1.size);
                                if symbol1.sym_type == DFISymbolType::Param {
                                    let mut param_ast = AbstractSyntaxTree::new();
                                    param_ast.ast_type = ASTType::Variable(symbol1.value);
                                    param_ast.value = sid as u64;
                                    param = Some(param_ast); 
                                }
                                /*
                                if ASTSymbolValueType::Long == ast_symbol.select_type || ASTSymbolValueType::UnsignedLong == ast_symbol.select_type {
                                    let assign_sym = &ast_symbol_map[&assign_sym_id];
                                    if assign_sym.select_type == ast_symbol.select_type {
                                        ast_symbol.select_type = ASTSymbolValueType::Unknown;
                                        let mut assign_sym = assign_sym.clone();
                                        assign_sym.select_type = ASTSymbolValueType::Unknown;
                                        ast_symbol_map.insert(assign_sym_id, assign_sym);
                                    }
                                }
                                */
                                ast_symbol
                            }
                        };

                        ast_symbol.scope = match &symbol1.address {
                            Address::Memory(_) => Scope::Global,
                            Address::Stack(_) => Scope::Local,
                            _ => Scope::Temp,
                        };
    
                        if ast_symbol.scope == Scope::Temp && symbol1.value == true {
                            if assign_ast_symbol.scope == Scope::Temp && assign_ast_symbol.select_type == ASTSymbolValueType::Long && assign_is_value == true {
                                assign_ast_symbol.select_type = ast_symbol.select_type.clone();
                                ast_symbol_map.insert(assign_ast_symbol.id, assign_ast_symbol);
                            }
                        } else if ast_symbol.scope == Scope::Local && symbol1.value == true {
                            if assign_ast_symbol.scope == Scope::Temp && assign_ast_symbol.select_type == ASTSymbolValueType::Long && assign_is_value == true {
                                assign_ast_symbol.select_type = type_change_to_ptr(&ast_symbol.select_type);
                                ast_symbol_map.insert(assign_ast_symbol.id, assign_ast_symbol);
                            }

                        }
                        //operand_sym_id = ast_symbol.id;
                        ast_operand2.value = ast_symbol.id as u64;
                        ast_operand2.ast_type = ASTType::Variable(symbol1.value);

                        ast_symbol_map.insert(ast_symbol.id, ast_symbol);
                    }
                    _ => {
                        println!("{}", ir);
                        println!("{:?}", ir);
                        panic!("operand error");
                    }
                }

                ast.next.push(Box::new(ast_operand2));


                (Some(ast), param)
            }
            DataFlowIrOpcode::Ret => {
                let mut ast = AbstractSyntaxTree::new();
                ast.ast_type = ASTType::EndReturn;
                let mut return_ast = AbstractSyntaxTree::new();
                match &ir.operand1 {
                    Some(DFIOperand::Symbol(symbol)) => {
                        let return_sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        return_ast.value = return_sym.id as u64;
                        return_ast.ast_type = ASTType::Variable(!symbol.value);
                    }
                    Some(DFIOperand::Number(number)) => {
                        return_ast.value = number.value as u64;
                        return_ast.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    _ => panic!("error"),
                }
                ast.next.push(Box::new(return_ast));
                (Some(ast), None)
            }
            DataFlowIrOpcode::Jmp => {
                (None, None)
            }
            DataFlowIrOpcode::Jcc(relation) => {
                (None, None)
            }
            DataFlowIrOpcode::Function => {
                (None, None)
            }
            DataFlowIrOpcode::Nop => (None, None),
        }
    }

    pub fn _parse_dfi(ir: &data_flow::DataFlowIr, ast_symbol_map: &mut HashMap<usize, ASTSymbol>, address_symbol_map: &mut HashMap<(Address, usize), usize>, counter: &mut Counter) -> Option<AbstractSyntaxTree> {
        let mut ast = AbstractSyntaxTree::new();
        let mut ast_operand1 = AbstractSyntaxTree::new();
        let mut ast_operand2 = AbstractSyntaxTree::new();
        let mut ast_operator = AbstractSyntaxTree::new();
        match &ir.opcode {
            DataFlowIrOpcode::Function => {
                ast.ast_type = ASTType::Parameter;
                if let Some(DFIOperand::Parameter(paramters)) = &ir.operand1 {
                    //println!("{:?}", paramters);
                    for paramter in paramters.iter() {
                        if let RegisterRecord::Symbol(symbol) = paramter {
                            let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter); 
                            let mut sym = ast_symbol_map.entry(sym.id).or_insert(ASTSymbol::new(usize::MAX));
                            sym.select_type = ASTSymbolValueType::Unknown;
                            let mut param_ast = AbstractSyntaxTree::new();
                            param_ast.ast_type = ASTType::Variable(!symbol.value);
                            param_ast.value = sym.id as u64;
                            ast.next.push(Box::new(param_ast));
                        }
                    }
                }  
                Some(ast)
            }
            DataFlowIrOpcode::Or => {
                let mut ast = AbstractSyntaxTree::new();
                let mut operator = AbstractSyntaxTree::new();
                let mut ast1 = AbstractSyntaxTree::new();
                let mut ast2 = AbstractSyntaxTree::new();
                    
                if let Some(DFIOperand::Symbol(symbol)) = &ir.operand1 {
                    let mut sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                    ast.ast_type = ASTType::Assign(false);
                    ast.value = sym.id as u64;
                } else {
                    panic!("error");
                }

                match &ir.operand2 {
                    Some(DFIOperand::Number(number)) => {
                        ast1.value = number.value as u64;
                        ast1.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast1.value = sym.id as u64;
                        ast1.ast_type = ASTType::Variable(!symbol.value);
                    }
                    _ => panic!("error"),
                }

                match &ir.operand3 {
                    Some(DFIOperand::Number(number)) => {
                        ast2.value = number.value as u64;
                        ast2.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast2.value = sym.id as u64;
                        ast2.ast_type = ASTType::Variable(!symbol.value);
                    }
                    _ => panic!("error"),
                }

                operator.ast_type = ASTType::Operator(Operator::Or);
                operator.next.push(Box::new(ast1));
                operator.next.push(Box::new(ast2));
                ast.next.push(Box::new(operator));

                Some(ast)
            }
            DataFlowIrOpcode::Add => {
                let mut ast = AbstractSyntaxTree::new();
                let mut operator = AbstractSyntaxTree::new();
                let mut ast1 = AbstractSyntaxTree::new();
                let mut ast2 = AbstractSyntaxTree::new();
                    
                if let Some(DFIOperand::Symbol(symbol)) = &ir.operand1 {
                    let mut sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                    ast.ast_type = ASTType::Assign(false);
                    ast.value = sym.id as u64;
                } else {
                    panic!("error");
                }

                match &ir.operand2 {
                    Some(DFIOperand::Number(number)) => {
                        ast1.value = number.value as u64;
                        ast1.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast1.value = sym.id as u64;
                        ast1.ast_type = match &symbol.sym_type {
                            DFISymbolType::Temp => ASTType::Variable(false),
                            DFISymbolType::Local => ASTType::Variable(!symbol.value),
                            _ => panic!(),
                        };
                    }
                    _ => panic!("error"),
                }

                match &ir.operand3 {
                    Some(DFIOperand::Number(number)) => {
                        ast2.value = number.value as u64;
                        ast2.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast2.value = sym.id as u64;
                        ast2.ast_type = match &symbol.sym_type {
                            DFISymbolType::Temp => ASTType::Variable(false),
                            DFISymbolType::Local => ASTType::Variable(!symbol.value),
                            _ => panic!(),
                        };
                    }
                    _ => panic!("error"),
                }

                operator.ast_type = ASTType::Operator(Operator::Add);
                operator.next.push(Box::new(ast1));
                operator.next.push(Box::new(ast2));
                ast.next.push(Box::new(operator));


                Some(ast)
            }
            DataFlowIrOpcode::Sub => {
                let mut ast = AbstractSyntaxTree::new();
                let mut operator = AbstractSyntaxTree::new();
                let mut ast1 = AbstractSyntaxTree::new();
                let mut ast2 = AbstractSyntaxTree::new();
                    
                if let Some(DFIOperand::Symbol(symbol)) = &ir.operand1 {
                    let mut sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                    ast.ast_type = ASTType::Assign(false);
                    ast.value = sym.id as u64;
                } else {
                    panic!("error");
                }

                match &ir.operand2 {
                    Some(DFIOperand::Number(number)) => {
                        ast1.value = number.value as u64;
                        ast1.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast1.value = sym.id as u64;
                        ast1.ast_type = ASTType::Variable(!symbol.value);
                    }
                    _ => panic!("error"),
                }

                match &ir.operand3 {
                    Some(DFIOperand::Number(number)) => {
                        ast2.value = number.value as u64;
                        ast2.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast2.value = sym.id as u64;
                        ast2.ast_type = ASTType::Variable(!symbol.value);
                    }
                    _ => panic!("error"),
                }

                operator.ast_type = ASTType::Operator(Operator::Sub);
                operator.next.push(Box::new(ast1));
                operator.next.push(Box::new(ast2));
                ast.next.push(Box::new(operator));

                Some(ast)
                //panic!("sub");
                //Some(ast)
            }
            DataFlowIrOpcode::Store => {
                let mut assign_type = ASTSymbolValueType::Unknown;
                let mut dfi_symbol = DFISymbolRecord {
                    address: Address::GR(0),
                    sym_type: DFISymbolType::Temp,
                    id: 0,
                    size: Size::Signed8,
                    value: false,
                };
                if let Some(DFIOperand::Symbol(symbol)) = &ir.operand2 {
                    dfi_symbol = symbol.clone();
                    let mut ast_symbol = match address_symbol_map.get(&(symbol.address.clone(), symbol.id)) {
                        Some(symbol) => {
                            let mut ast_symbol = ast_symbol_map[symbol].clone();
                            ast_symbol
                        }
                        None => {
                            let sid = counter.get();
                            address_symbol_map.insert((symbol.address.clone(), symbol.id), sid);
                            let mut ast_symbol = ASTSymbol::new(sid);
                            ast_symbol.select_type = get_ast_type_from_size(&symbol.size);
                            ast_symbol
                        }
                    };
                    assign_type = ast_symbol.select_type.clone();

                    
                    match &symbol.address {
                        Address::Stack(stack) => {
                            ast_symbol.scope = Scope::Local;
                        }
                        Address::Memory(memory) => {
                            ast_symbol.scope = Scope::Global;
                        }
                        Address::GR(gr) => {
                            ast_symbol.scope = Scope::Local;   
                        }
                        Address::FR(fr) => {
                            ast_symbol.scope = Scope::Local;
                        }
                    }

                    ast.value = ast_symbol.id as u64;
                    ast.ast_type = ASTType::Assign(false);
                    ast_symbol_map.insert(ast_symbol.id, ast_symbol);

                } else {
                    println!("{}", ir);
                    println!("{:?}", ir);
                    panic!("operand error");
                }

        

                let mut size = ASTSymbolValueType::Unknown;
                match &ir.operand1 {
                    Some(DFIOperand::Number(number)) => {
                        ast_operand2.value = number.value as u64;
                        ast_operand2.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let mut ast_symbol = match address_symbol_map.get(&(symbol.address.clone(), symbol.id)) {
                            Some(symbol) => {
                                ast_symbol_map[symbol].clone()
                            }
                            None => {
                                let sid = counter.get();
                                address_symbol_map.insert((symbol.address.clone(), symbol.id), sid);
                                let mut ast_symbol = ASTSymbol::new(sid);
                                ast_symbol.select_type = get_ast_type_from_size(&symbol.size);
                                ast_symbol
                            }
                        };

                        size = ast_symbol.select_type.clone();
                        
                        if ast_symbol.select_type == ASTSymbolValueType::Unknown {
                            if assign_type != ASTSymbolValueType::Long {
                                ast_symbol.select_type = assign_type.clone();
                            } else {
                                ast_symbol.select_type = ASTSymbolValueType::Ptr;
                                let assign_sym = ast_symbol_map.entry(ast.value as usize).or_insert(ASTSymbol::new(ast.value as usize));   
                                assign_sym.select_type = ASTSymbolValueType::Ptr;
                            }
                        }


                        /*
                        let symtype = &symbol.sym_type;
                        let sid = symbol.id;
                        let size = &symbol.size;
                        let is_value = symbol.value;
    
                        if is_value {
                            if let Some(sym) = ast_symbol_map.get(&sid) {
                                ast_symbol.select_type = sym.select_type.clone();
                            } else {
                                ast_symbol.select_type = ASTSymbolValueType::Unknown;
                            }
                        } else {
                            ast_symbol.select_type = get_ast_type_from_size(size);
                        }
                        */
                        
                        
                        match &symbol.address {
                            Address::Stack(stack) => {
                                ast_symbol.scope = Scope::Local;
                            }
                            Address::Memory(memory) => {
                                ast_symbol.scope = Scope::Global;
                            }
                            Address::GR(gr) => {
                                ast_symbol.scope = Scope::Temp;   
                            }
                            Address::FR(fr) => {
                                ast_symbol.scope = Scope::Temp;
                            }
                        }


                        ast_operand2.value = ast_symbol.id as u64;
                        if symbol.sym_type != DFISymbolType::Temp && symbol.sym_type != DFISymbolType::Param {
                            ast_operand2.ast_type = ASTType::Variable(!symbol.value);
                            ast.ast_type = ASTType::Assign(!symbol.value);
                            if !symbol.value {
                                let ast_sym = ast_symbol_map.entry(ast.value as usize).or_insert(ASTSymbol::new(ast.value as usize));
                                //ast.select_type = ast_symbol.select_type.clone();
                                //ast.select_type = type_change_to_ptr(&ast_symbol.select_type);
                                ast_sym.select_type = ASTSymbolValueType::Ptr;
                                let ast_sym = ast_symbol_map.entry(ast_symbol.id).or_insert(ASTSymbol::new(ast_symbol.id));
                                ast_sym.select_type = ASTSymbolValueType::Ptr;
                                ast_symbol.select_type = ASTSymbolValueType::Ptr;

                                let assign_ast = ast_symbol_map.entry(ast.value as usize).or_insert(ASTSymbol::new(ast.value as usize));
                                if ast_symbol.select_type != assign_ast.select_type {
                                    ast_symbol.select_type = assign_ast.select_type.clone(); 
                                }
                            } else {
                                let assign_ast = ast_symbol_map.entry(ast.value as usize).or_insert(ASTSymbol::new(ast.value as usize));
                                assign_ast.select_type = type_change_to_ptr(&size);
                            }
                        } else {
                            let sym = ast_symbol_map.entry(ast.value as usize).or_insert(ASTSymbol::new(usize::MAX));
                            if dfi_symbol.sym_type == DFISymbolType::Temp {
                                sym.select_type = type_change_to_ptr(&size);
                            }
                            ast_operand2.ast_type = ASTType::Variable(false);
                        }


                        ast_symbol_map.insert(ast_symbol.id, ast_symbol);
    
                    }
                    _ => panic!("operand error"),
                }

                ast.next.push(Box::new(ast_operand2));

                /*
                if ir.address == 0x1200006e8 {
                    println!("{:?}", ast_symbol_map[&1]);
                    println!("{:?}", ast_symbol_map[&2]);
                    panic!("{:?}", ast);
                }
                */

                /*
                if ir.address == 0x1200007b8 {
                    println!("{}", ast.to_string(&ast_symbol_map));
                    panic!();
                }
                */
                Some(ast)
            }
            // DataFlowIrOpcode::Store
            DataFlowIrOpcode::Load => {
                let mut size = ASTSymbolValueType::Unknown;
                if let Some(DFIOperand::Symbol(symbol)) = &ir.operand1 {
                    let mut ast_symbol = match address_symbol_map.get(&(symbol.address.clone(), symbol.id)) {
                        Some(symbol) => {
                            ast_symbol_map[symbol].clone()
                        }
                        None => {
                            let sid = counter.get();
                            address_symbol_map.insert((symbol.address.clone(), symbol.id), sid);
                            let mut ast_symbol = ASTSymbol::new(sid);
                            ast_symbol.select_type = get_ast_type_from_size(&symbol.size);
                            ast_symbol
                        }
                    };
                    size = ast_symbol.select_type.clone();


                    /*
                    let symtype = &symbol.sym_type;
                    let sid = symbol.id;
                    let size = &symbol.size;
                    let is_value = symbol.value;

                    if is_value {
                        if let Some(sym) = ast_symbol_map.get(&sid) {
                            ast_symbol.select_type = sym.select_type.clone();
                        } else {
                            ast_symbol.select_type = ASTSymbolValueType::Unknown;
                        }
                    } else {
                        ast_symbol.select_type = get_ast_type_from_size(size);
                    }
                    */
                    
                    
                    match &symbol.address {
                        Address::Stack(stack) => {
                            ast_symbol.scope = Scope::Local;
                        }
                        Address::Memory(memory) => {
                            ast_symbol.scope = Scope::Global;
                        }
                        Address::GR(gr) => {
                            ast_symbol.scope = Scope::Temp;   
                        }
                        Address::FR(fr) => {
                            ast_symbol.scope = Scope::Temp;
                        }
                    }


                    ast.value = ast_symbol.id as u64;
                    ast.ast_type = ASTType::Assign(false);
                    ast_symbol_map.insert(ast_symbol.id, ast_symbol);

                } else {
                    println!("{:?}", ir);
                    panic!("operand error");
                }
                

                match &ir.operand2 {
                    Some(DFIOperand::Number(number)) => {
                        ast_operand2.value = number.value as u64;
                        ast_operand2.ast_type = ASTType::Integer(true, get_number_type(&number.size, number.signed));
                        panic!("operand error");
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let mut ast_symbol = match address_symbol_map.get(&(symbol.address.clone(), symbol.id)) {
                            Some(symbol) => {
                                let ast_symbol = ast_symbol_map.entry(*symbol).or_insert(ASTSymbol::new(usize::MAX));
                                //ast_symbol.select_type = type_change_to_ptr(&size);
                                ast_symbol.clone()
                            }
                            None => {
                                let sid = counter.get();
                                address_symbol_map.insert((symbol.address.clone(), symbol.id), sid);
                                let mut ast_symbol = ASTSymbol::new(sid);
                                ast_symbol.select_type = get_ast_type_from_size(&symbol.size);
                                //ast_symbol.select_type = type_change_to_ptr(&size);
                                ast_symbol
                            }
                        };
                        

                        if ir.address == 0x1200007e0 {
                            //println!("{:?}", ir.operand2);
                            //println!("{:?}", size);
                            //println!("{:?}", type_change_to_ptr(&size));
                            //panic!("{:?}", ast_symbol.select_type);
                            //panic!("{}", symbol.id);
                        }
                        /*
                        let symtype = &symbol.sym_type;
                        let sid = symbol.id;
                        let size = &symbol.size;
                        let is_value = symbol.value;
    
                        if is_value {
                            if let Some(sym) = ast_symbol_map.get(&sid) {
                                ast_symbol.select_type = sym.select_type.clone();
                            } else {
                                ast_symbol.select_type = ASTSymbolValueType::Unknown;
                            }
                        } else {
                            ast_symbol.select_type = get_ast_type_from_size(size);
                        }
                        */
                        
                        
                        match &symbol.address {
                            Address::Stack(stack) => {
                                ast_symbol.scope = Scope::Local;
                            }
                            Address::Memory(memory) => {
                                ast_symbol.scope = Scope::Global;
                            }
                            Address::GR(gr) => {
                                ast_symbol.scope = Scope::Temp;   
                                ast_symbol.select_type = type_change_to_ptr(&size);
                            }
                            Address::FR(fr) => {
                                ast_symbol.scope = Scope::Temp;
                            }
                        }
    
                        ast_operand2.value = ast_symbol.id as u64;
                        ast_operand2.ast_type = ASTType::Variable(!symbol.value);
                        /*
                        let ast_sym = ast_symbol_map.entry(ast_operand2.value).or_insert(ASTSymbol::new(ast_operand2.value));
                        ast_sym.select_type = type_change_to_ptr(&size);
                        */
                        //ast_symbol.select_type = type_change_to_ptr(&size);
                        


                        /*
                        if ir.address == 0x1200006d8 {
                            println!("{:?}", ast_operand2);
                            println!("-{}", ast_operand2.to_string(&ast_symbol_map));
                            panic!();
                        }
                        */
                        ast_symbol_map.insert(ast_symbol.id, ast_symbol);
    
                    }
                    _ => panic!("operand error"),
                }

                ast.next.push(Box::new(ast_operand2));

                /*
                if ir.address == 0x1200007e0 {
                    println!("{:#?}", ast);
                    panic!("");
                }
                */
                /*
                if ir.address == 0x1200007b8 {
                    panic!("{}", ast.to_string(&ast_symbol_map));
                }
                */

                //panic!("load");
                Some(ast)
            }
            DataFlowIrOpcode::Call => {
                let functions = FUNCTIONS.lock().unwrap();     
                if let Some(DFIOperand::Number(number)) = &ir.operand1 {
                    if let Some(name) = functions.get(&(number.value as u64)) {
                        match name {
                            NameValue::Name(name) => ast.ast_type = ASTType::Function(name.clone()), 
                            NameValue::Value(value) => ast.ast_type = ASTType::Function(format!("funcion@{:x}", number.value)),
                        }
                    } else {
                        ast.ast_type = ASTType::Function(format!("func@{:x}", number.value));
                    }
                } else {
                    panic!("operand error");
                }

                ast_operand1.ast_type = ASTType::Return;
                match &ir.operand3 {
                    Some(DFIOperand::Symbol(symbol)) => {
                        let mut ast_symbol = match address_symbol_map.get(&(symbol.address.clone(), symbol.id)) {
                            Some(symbol) => {
                                ast_symbol_map[symbol].clone()
                            }
                            None => {
                                let sid = counter.get();
                                address_symbol_map.insert((symbol.address.clone(), symbol.id), sid);
                                let mut ast_symbol = ASTSymbol::new(sid);
                                ast_symbol.select_type = get_ast_type_from_size(&symbol.size);
                                ast_symbol
                            }
                        };

                        if let ASTType::Function(name) = &ast.ast_type {
                            match name.as_str() {
                                "printf" => {
                                    ast_symbol.select_type = ASTSymbolValueType::Int;
                                }
                                _ => {}
                            }
                        }
                        
                        /*
                        let symtype = &symbol.sym_type;
                        let sid = symbol.id;
                        let size = &symbol.size;
                        let is_value = symbol.value;
    
                        if is_value {
                            if let Some(sym) = ast_symbol_map.get(&sid) {
                                ast_symbol.select_type = sym.select_type.clone();
                            } else {
                                ast_symbol.select_type = ASTSymbolValueType::Unknown;
                            }
                        } else {
                            ast_symbol.select_type = get_ast_type_from_size(size);
                        }
                        */
                        
                        
                        match &symbol.address {
                            Address::Stack(stack) => {
                                ast_symbol.scope = Scope::Local;
                            }
                            Address::Memory(memory) => {
                                ast_symbol.scope = Scope::Global;
                            }
                            Address::GR(gr) => {
                                ast_symbol.scope = Scope::Temp;   
                            }
                            Address::FR(fr) => {
                                ast_symbol.scope = Scope::Temp;
                            }
                        }

                        ast_operand1.value = ast_symbol.id as u64;
                        ast_symbol_map.insert(ast_symbol.id, ast_symbol);
                        
                    }


                    Some(DFIOperand::Number(number)) => {
                        ast_operand1.value = number.value as u64;
                        ast_operand1.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    _ => {
                        println!("{:?}", ir.operand2);
                        println!("{:?}", ir);
                        panic!("operand error");
                    }
                }

                ast.next.push(Box::new(ast_operand1));

                if let Some(DFIOperand::Parameter(parameters)) = &ir.operand2 {
                   for parameter in parameters.iter() {
                        let mut parameter_ast = AbstractSyntaxTree::new();
                        match parameter {
                            RegisterRecord::Number(number) => {
                                parameter_ast.value = number.value as u64; 
                                parameter_ast.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                            }
                            RegisterRecord::Symbol(symbol) => {
                                let mut ast_sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                                parameter_ast.ast_type = ASTType::Variable(!symbol.value);
                                parameter_ast.value = ast_sym.id as u64;
                            }
                        }
                        ast.next.push(Box::new(parameter_ast));
                   } 

                } else {
                    panic!("error");
                }

                //println!("{:#?}", ast);
                Some(ast)
            }
            // DataFlowIrOpcode::Call
            DataFlowIrOpcode::Ret => {
                let mut ast = AbstractSyntaxTree::new();
                ast.ast_type = ASTType::EndReturn;
                let mut return_ast = AbstractSyntaxTree::new();
                match &ir.operand1 {
                    Some(DFIOperand::Symbol(symbol)) => {
                        let return_sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        return_ast.value = return_sym.id as u64;
                        return_ast.ast_type = ASTType::Variable(!symbol.value);
                    }
                    Some(DFIOperand::Number(number)) => {
                        return_ast.value = number.value as u64;
                        return_ast.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    _ => panic!("error"),
                }
                ast.next.push(Box::new(return_ast));
                Some(ast)
            }
            DataFlowIrOpcode::Not => {
                panic!("not");
                Some(ast)
            }
            DataFlowIrOpcode::Xor => {
                panic!("xor");
                Some(ast)
            }
            DataFlowIrOpcode::And => {
                panic!("and");
                Some(ast)
            }
            DataFlowIrOpcode::Div => {
                panic!("div");
                Some(ast)
            }
            DataFlowIrOpcode::Mul => {
                let mut ast = AbstractSyntaxTree::new();
                let mut operator = AbstractSyntaxTree::new();
                let mut ast1 = AbstractSyntaxTree::new();
                let mut ast2 = AbstractSyntaxTree::new();
                    
                if let Some(DFIOperand::Symbol(symbol)) = &ir.operand1 {
                    let mut sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                    ast.ast_type = ASTType::Assign(false);
                    ast.value = sym.id as u64;
                } else {
                    panic!("error");
                }


                match &ir.operand2 {
                    Some(DFIOperand::Number(number)) => {
                        ast1.value = number.value as u64;
                        ast1.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast1.value = sym.id as u64;
                        ast1.ast_type = match &symbol.sym_type {
                            DFISymbolType::Temp => ASTType::Variable(false),
                            DFISymbolType::Local => ASTType::Variable(!symbol.value),
                            _ => panic!(),
                        }
                    }
                    _ => panic!("error"),
                }

                match &ir.operand3 {
                    Some(DFIOperand::Number(number)) => {
                        ast2.value = number.value as u64;
                        ast2.ast_type = ASTType::Integer(number.signed, get_number_type(&number.size, number.signed));
                    }
                    Some(DFIOperand::Symbol(symbol)) => {
                        let sym = create_symbol(symbol, address_symbol_map, ast_symbol_map, counter);
                        ast2.value = sym.id as u64;
                        ast1.ast_type = match &symbol.sym_type {
                            DFISymbolType::Temp => ASTType::Variable(false),
                            DFISymbolType::Local => ASTType::Variable(!symbol.value),
                            _ => panic!(),
                        }
                    }
                    _ => panic!("error"),
                }

                operator.ast_type = ASTType::Operator(Operator::Mul);
                operator.next.push(Box::new(ast1));
                operator.next.push(Box::new(ast2));
                ast.next.push(Box::new(operator));

                Some(ast)
            }
            _ => None,
        }
    }


    pub fn to_string(&self, ast_symbol_map: &HashMap<usize, ASTSymbol>) -> String {
        let mut ast_str = String::new();
        let mut indent: usize = 0;
        //self.list_var(&mut ast_str, ast_symbol_map);
        let mut type_set = HashMap::<usize, bool>::new();
        self.to_string_recursion(&mut ast_str, &mut indent, ast_symbol_map, &mut type_set);
        ast_str
    }

    fn list_var(&self, ast_str: &mut String, ast_symbol_map: &HashMap<usize, ASTSymbol>) {
        let mut var_vec: Vec<(&usize, &ASTSymbol)> = ast_symbol_map.iter().collect();
        var_vec.sort_by(|a, b| a.0.cmp(b.0));
        for (id, sym) in var_vec {
            *ast_str += &format!("{} var{};\n", astvaluetype_to_string(&sym.select_type), id);
        }
    }

    /*
    fn list_variable(&self, ast_str: &mut String, ast_symbol_map: &HashMap<usize, ASTSymbol>) {
        match &self.ast_type {
            ASTType::Assign(is_ptr) => {
                let symbol = &ast_symbol_map[&self.value];
                *ast_str += &format!("is_ptr: {}, size: {:?}\t", is_ptr, symbol.select_type);
                *ast_str += &format!("{} var{};\n", type_to_string(self, ast_symbol_map, *is_ptr, &ASTSymbol::new(0)), self.value);
            }
            _ => {
                for next in self.next.iter() {
                    next.list_variable(ast_str, ast_symbol_map);
                }
            }
        } 
    }
    */

    fn to_string_recursion(&self, ast_str: &mut String, indent: &mut usize, ast_symbol_map: &HashMap<usize, ASTSymbol>, type_set: &mut HashMap<usize, bool>) {
        match &self.ast_type {
            ASTType::Begin(name) => {
                for next in self.next.iter() {
                    if let ASTType::FunctionReturn(return_type, is_ptr) = &next.ast_type {
                        *ast_str += &format!("{} ", return_type_to_string(return_type, *is_ptr));
                    }
                }

                *ast_str += name;
                *ast_str += "(";
                for next in self.next.iter() {
                    if let ASTType::Parameter = &next.ast_type {
                        let len = next.next.len();
                        for (i, p) in next.next.iter().enumerate() {
                            let symbol = &ast_symbol_map[&(p.value as usize)];
                            if i == len - 1 {
                                *ast_str += &format!("{} var{}", astvaluetype_to_string(&symbol.select_type), p.value);
                            } else {
                                *ast_str += &format!("{} var{}, ", astvaluetype_to_string(&symbol.select_type), p.value);
                            }
                        }
                        break;
                    }
                } 
                *ast_str += ")\n";
                *ast_str += "{\n";
                *indent += 1;
                for next in self.next.iter() {
                    next.to_string_recursion(ast_str, indent, ast_symbol_map, type_set);
                }
                *indent -=1;
                *ast_str += "}\n";
            }
            ASTType::FunctionReturn(return_type, is_ptr) => {}
            ASTType::EndReturn => {
                *ast_str += &format!("{}retrun ", get_indent(&indent));
                match &self.next.first().unwrap().ast_type {
                    ASTType::Integer(is_signed, _) => *ast_str += &integer_to_string(&self.next.first().unwrap()),
                    ASTType::Variable(_) => *ast_str += &format!("var{}", self.next.first().unwrap().value),
                    _ => panic!("error"),
                }
                *ast_str += ";\n"
            }
            ASTType::Function(name) => {
                *ast_str += &get_indent(&indent);
                for next in self.next.iter() {
                    if let ASTType::Return = &next.ast_type {
                        let sym = &ast_symbol_map[&(next.value as usize)];
                        *ast_str += &format!("{} var{} = ", return_type_to_string(&sym.select_type, false), next.value);
                        break;
                    }
                }
                *ast_str += &format!("{}", name);
                *ast_str += "(";
                let data = DATA.lock().unwrap();
                match name.as_str() {
                    "printf" | "puts" => {
                        let mut is_string = false;
                        for i in 0..self.next.len() {
                            let next = &self.next[i];
                            if let ASTType::Return = next.ast_type {
                                is_string = true;
                                continue;
                            }

                            if i == self.next.len() - 1 {
                                if is_string {
                                    let address = match &next.ast_type {
                                        ASTType::Integer(_, _) => next.value,
                                        _ => panic!("error"),
                                    };
                                    let string = format!("\"{}\"", get_c_string_from_data(address, &data).unwrap());
                                    *ast_str += &string;
                                    is_string = false;
                                } else {
                                    match &next.ast_type {
                                        ASTType::Variable(_) => *ast_str += &format!("var{}", next.value),
                                        ASTType::Integer(is_signed, _) => *ast_str += &number_to_string(next.value, *is_signed),
                                        _ => panic!("error"),
                                    };
                                    break;
                                }
                            } else {
                                if is_string {
                                    let address = match &next.ast_type {
                                        ASTType::Integer(_, _) => next.value,
                                        _ => panic!("error"),
                                    };
                                    let string = format!("\"{}\"", get_c_string_from_data(address, &data).unwrap());
                                    *ast_str += &string;
                                    *ast_str += ", ";
                                    is_string = false;
                                } else {
                                    match &next.ast_type {
                                        ASTType::Variable(_) => *ast_str += &format!("var{}", next.value),
                                        ASTType::Integer(is_signed, _) => *ast_str += &number_to_string(next.value, *is_signed),
                                        _ => panic!("error"),
                                    }
                                    *ast_str += ", ";
                                }
                            }
                        }
                    }
                    _ => {
                        for next in self.next.iter() {
                            if let ASTType::Return = &next.ast_type {
                                continue;
                            }
                            if *next == *self.next.last().unwrap() {
                                match &next.ast_type {
                                    ASTType::Variable(_) => *ast_str += &format!("var{}", next.value),
                                    ASTType::Integer(is_signed, _) => *ast_str += &number_to_string(next.value, *is_signed),
                                    _ => panic!("error"),
                                }
                                break;
                            } else {
                                match &next.ast_type {
                                    ASTType::Variable(_) => *ast_str += &format!("var{}", next.value),
                                    ASTType::Integer(is_signed, _) => *ast_str += &number_to_string(next.value, *is_signed),
                                    _ => panic!("error"),
                                }
                                *ast_str += ", ";
                            }
                        }
                    }
                }
                *ast_str += ");\n";
            }
            ASTType::Parameter => {}
            ASTType::Return => {

            }
            ASTType::Loop => {
                *ast_str += &format!("{}loop\n", get_indent(&indent));
                *ast_str += &format!("{}{}\n", get_indent(&indent), "{");
                *indent += 1;
                for next in self.next.iter() {
                    next.to_string_recursion(ast_str, indent, ast_symbol_map, type_set);
                }
                *indent -= 1;
                *ast_str += &format!("{}{}\n", get_indent(&indent), "}");
            }
            ASTType::Break => {
                *ast_str += &format!("{}break;\n", get_indent(&indent));
            }
            ASTType::Continue => {
                *ast_str += &format!("{}continue;\n", get_indent(&indent));
            }
            ASTType::Assign(is_ptr) => {
                *ast_str += &format!("{}", get_indent(&indent));
                let mut assign_str = String::new();
                assign_to_string(self, &mut assign_str, ast_symbol_map, type_set);
                *ast_str += &assign_refined::refine_assign_str_bracket(assign_str);
                *ast_str += ";\n";
            }
            ASTType::If => {
                *ast_str += &format!("{}if(", get_indent(&indent));
                for next in self.next.iter() {
                    match &next.ast_type {
                        ASTType::Condictions => {
                            let mut conds_str = String::new();
                            get_condictions_str(next, &mut conds_str, ast_symbol_map);
                            *ast_str += &conds_str;
                            break;
                        }
                        ASTType::Condiction(relation) => {
                            if let Some(operand1) = next.next.first() {
                                match &operand1.ast_type {
                                    ASTType::Integer(is_signed, _) => *ast_str += &format!("{}", number_to_string(operand1.value, *is_signed)),
                                    ASTType::Variable(_) => *ast_str += &format!("var{}", operand1.value),
                                    _ => panic!("error"),
                                }
                            }
                            *ast_str += &format!(" {} ", relation);
                            if let Some(operand2) = next.next.last() {
                                match &operand2.ast_type {
                                    ASTType::Integer(is_signed, _) => *ast_str += &format!("{}", number_to_string(operand2.value, *is_signed)),
                                    ASTType::Variable(_) => *ast_str += &format!("var{}", operand2.value),
                                    _ => panic!("error"),
                                }
                            }

                            break;
                        }
                        _ => {}
                    }
                }
                *ast_str += ")\n";

                *ast_str += &format!("{}{}\n", get_indent(&indent), "{");
                *indent += 1;
                for next in self.next.iter() {
                    if let ASTType::True = &next.ast_type {
                        next.to_string_recursion(ast_str, indent, ast_symbol_map, type_set);
                        break;
                    }
                }
                *indent -= 1;
                *ast_str += &format!("{}{}\n", get_indent(&indent), "}");
                for next in self.next.iter() {
                    if let ASTType::False = &next.ast_type {
                        *ast_str += &format!("{}else\n", get_indent(&indent));
                        *ast_str += &format!("{}{}\n", get_indent(&indent), "{");
                        *indent += 1;
                        next.to_string_recursion(ast_str, indent, ast_symbol_map, type_set);
                        *indent -= 1;
                        *ast_str += &format!("{}{}\n", get_indent(&indent), "}");
                    }
                }
            }
            ASTType::Condiction(relation) => {}
            ASTType::Condictions => {}
            ASTType::True => {
                for next in self.next.iter() {
                    next.to_string_recursion(ast_str, indent, ast_symbol_map, type_set);
                }
            }
            ASTType::False => {
                for next in self.next.iter() {
                    next.to_string_recursion(ast_str, indent, ast_symbol_map, type_set);
                }
            }
            ASTType::Operator(op) => {}
            ASTType::Variable(is_ptr) => {}
            ASTType::Integer(is_signed, int_type) => {}
            ASTType::Float(is_signed, flt_type) => {}
            ASTType::While => {
                *ast_str += &format!("{}while", get_indent(&indent));
                *ast_str += "(";
                for next in self.next.iter() {
                    if next.ast_type == ASTType::Condictions {
                        get_condictions_str(next, ast_str, ast_symbol_map);
                        break;
                    }
                }
                *ast_str += ")\n";
                *ast_str += &format!("{}{}\n", get_indent(&indent), "{");
                *indent += 1;
                for next in self.next.iter() {
                    if next.ast_type != ASTType::Condictions {
                        next.to_string_recursion(ast_str, indent, ast_symbol_map, type_set);
                    }
                }
                *indent -= 1;
                *ast_str += &format!("{}{}\n", get_indent(&indent), "}");
            }
        } 
    }




    fn _to_string_recursion(&self, ast_str: &mut String, indent: &mut usize, ast_symbol_map: &HashMap<usize, ASTSymbol>) {
        match &self.ast_type {
            ASTType::Begin(name) => {
                for next in self.next.iter() {
                    if let ASTType::FunctionReturn(return_type, is_ptr) = &next.ast_type {
                        *ast_str += &format!("{} ", return_type_to_string(return_type, *is_ptr)); 
                        break;
                    }
                }
                *ast_str += &format!("{}(", name);
                for next in self.next.iter() {
                    if let ASTType::Parameter = &next.ast_type {
                        for p in next.next.iter() {
                            if p == next.next.last().unwrap() {
                                if let ASTType::Variable(is_ptr) = &p.ast_type {
                                    let sym = &ast_symbol_map[&(p.value as usize)];
                                    *ast_str += &format!("{} var{}", parameter_to_string(&sym.select_type), sym.id);
                                }
                            } else {
                                if let ASTType::Variable(is_ptr) = &p.ast_type {
                                    let sym = &ast_symbol_map[&(p.value as usize)];
                                    *ast_str += &format!("{} var{}, , ", parameter_to_string(&sym.select_type), sym.id);
                                }
                            }
                        }
                        break;
                    }
                }
                *ast_str += ")\n";
                *ast_str += "{\n";
                *indent += 1;
                for next in self.next.iter() {
                    if let ASTType::Parameter = &next.ast_type {
                        continue;
                    }
                    next._to_string_recursion(ast_str, indent, ast_symbol_map);
                }
                *indent -= 1;
                *ast_str += "}\n";
            }
            ASTType::Function(f) => {
                *ast_str += &get_indent(&indent);
                for next in self.next.iter() {
                    if let ASTType::Return = &next.ast_type {
                        //*ast_str += &format!("var{} = ", next.value); 
                        *ast_str += &variable_to_string(next, ast_symbol_map, false);
                        *ast_str += " = ";
                        break;
                    }
                }
                *ast_str += &format!("{}", f);
                *ast_str += "(";
                let data = DATA.lock().unwrap();
                match f.as_str() {
                    "printf" | "puts" => {
                        let mut is_string = false;
                        for i in 0..self.next.len() {
                            let next = &self.next[i];
                            if let ASTType::Return = next.ast_type {
                                is_string = true;
                                continue;
                            }

                            if i == self.next.len() - 1 {
                                if is_string {
                                    let address = match &next.ast_type {
                                        ASTType::Integer(_, _) => next.value,
                                        _ => panic!("error"),
                                    };
                                    let string = format!("\"{}\"", get_c_string_from_data(address, &data).unwrap());  
                                    *ast_str += &string;
                                    is_string = false;
                                } else {
                                    match &next.ast_type {
                                        ASTType::Variable(is_ptr) => *ast_str += &variable_to_string(next, ast_symbol_map, false),
                                        ASTType::Integer(_, _) => *ast_str += &integer_to_string(next),
                                        _ => panic!("error"),
                                    }
                                    break;
                                }
                            } else {
                                if is_string {
                                    let address = match &next.ast_type {
                                        ASTType::Integer(_, _) => next.value,
                                        _ => panic!("error"),
                                    };
                                    let string = format!("\"{}\"", get_c_string_from_data(address, &data).unwrap());  
                                    *ast_str += &string;
                                    *ast_str += ", ";
                                    is_string = false;
                                } else {
                                    match &next.ast_type {
                                        ASTType::Variable(is_ptr) => *ast_str += &variable_to_string(next, ast_symbol_map, false),
                                        ASTType::Integer(_, _) => *ast_str += &integer_to_string(next),
                                        _ => panic!("error"),
                                    }
                                    *ast_str += ", ";
                                }
                            }

                        }
                    }
                    _ => {
                        for next in self.next.iter() {
                            if let ASTType::Return = &next.ast_type {
                                continue;
                            }
                            if *next == *self.next.last().unwrap() {
                                //*ast_str += &format!("var{}", next.value); 
                                match &next.ast_type {
                                    ASTType::Variable(is_ptr) => *ast_str += &variable_to_string(next, ast_symbol_map, false),
                                    ASTType::Integer(_, _) => *ast_str += &integer_to_string(next),
                                    _ => panic!("error"),
                                }
                                break;
                            } else {
                                //*ast_str += &format!("var{}, ", next.value);
                                match &next.ast_type {
                                    ASTType::Variable(is_ptr) => *ast_str += &variable_to_string(next, ast_symbol_map, false),
                                    ASTType::Integer(_, _) => *ast_str += &integer_to_string(next),
                                    _ => panic!("error"),
                                }
                                *ast_str += ", ";
                            }
                        }
                    }
                }
                *ast_str += ");\n";
            }
            ASTType::Parameter => {panic!("error");}
            ASTType::Return => {panic!("error");}
            ASTType::EndReturn => {
                //*ast_str += &format!("{}return var{};\n", get_indent(&indent), self.value);
                //*ast_str += &variable_to_string(self, ast_symbol_map); 
                *ast_str += &format!("{}return ", get_indent(&indent));
                match &self.next.first().unwrap().ast_type {
                    ASTType::Integer(_, _) => *ast_str += &integer_to_string(&self.next.first().unwrap()),
                    ASTType::Variable(is_ptr) => *ast_str += &variable_to_string(&self.next.first().unwrap(), ast_symbol_map, false),
                    _ => panic!("error"),
                }
                *ast_str += ";\n";
            }
            ASTType::Assign(is_ptr) => {
                let mut assign_str = String::new();
                let mut flag = 0;
                _assign_to_string(self, &mut assign_str, ast_symbol_map, &mut flag, &ASTSymbol::new(usize::MAX));
                *ast_str += &format!("{}{}\n", get_indent(&indent), assign_str);

                /*
                *ast_str += &format!("{}var{} = ", get_indent(&indent), self.value);
                for next in self.next.iter() {
                    if *next == *self.next.last().unwrap() {
                        *ast_str += &format!("var{};\n", next.value);
                        break;
                    } else {
                        if let ASTType::Variable = &next.ast_type {
                            *ast_str += &format!("var{}", next.value);
                        } else if let ASTType::Operator(operator) = &next.ast_type {
                            *ast_str += &format!(" {} ", convert_operator_to_string(operator));
                        }
                    }
                }
                */
            }
            ASTType::If => {
                *ast_str += &get_indent(&indent);
                *ast_str += "if";
                *ast_str += "(";
                for next in self.next.iter() {
                    if let ASTType::Condictions = &next.ast_type {
                        let mut conds_str = String::new();
                        get_condictions_str(next, &mut conds_str, ast_symbol_map); 
                        //conds_str.remove(0);
                        //conds_str.pop();
                        *ast_str += &conds_str;
                    } else if let ASTType::Condiction(relation) = &next.ast_type {
                        if let Some(operand1) = next.next.first() {
                            //*ast_str += &format!("var{}", operand1.value);
                            match &operand1.ast_type {
                                ASTType::Integer(_, _) => *ast_str += &integer_to_string(&operand1),
                                ASTType::Variable(is_ptr) => *ast_str += &variable_to_string(&operand1, ast_symbol_map, true),
                                _ => panic!("error"),
                            }
                        }

                        //println!("@@@@@@@@@@@@@@{:?}", self.next.iter().filter(|t| t.ast_type == ASTType::True));
                        /*
                        for next in self.next.iter() {
                            println!("{:?}: {}", next.ast_type, next.next.len());
                        }
                        for next in self.next.iter() {
                            if let ASTType::True = &next.ast_type {
                                if next.next.len() == 0 {
                                    true_exist = false;

                                }
                            }
                            if let ASTType::False = &next.ast_type {
                                if next.next.len() == 0 {
                                    false_exist = false;
                                }
                            }
                        }
                        */


                        *ast_str += &format!(" {} ", relation);

                        if let Some(operand2) = next.next.last() {
                            //*ast_str += &format!("var{}", operand2.value);
                            match &operand2.ast_type {
                                ASTType::Integer(_, _) => *ast_str += &integer_to_string(&operand2),
                                ASTType::Variable(is_ptr) => *ast_str += &variable_to_string(&operand2, ast_symbol_map, true),
                                _ => panic!("error"),
                            }
                        }
                        break;
                    }
                } 
                *ast_str += ")\n";
                *ast_str += &format!("{}{}", get_indent(&indent), "{\n");
                *indent += 1;
                for next in self.next.iter() {
                    if let ASTType::True = &next.ast_type {
                        //*ast_str += &format!("{}{}", get_indent(&indent), "{\n");
                        //*indent += 1;
                        next._to_string_recursion(ast_str, indent, ast_symbol_map);
                        //*indent -= 1;
                        //*ast_str += &format!("{}{}", get_indent(&indent), "}\n");
                        break;
                    }
                }
                *indent -= 1;
                *ast_str += &format!("{}{}", get_indent(&indent), "}\n");
                for next in self.next.iter() {
                    if let ASTType::False = &next.ast_type {
                        *ast_str += &format!("{}else\n", get_indent(&indent));
                        *ast_str += &format!("{}{}", get_indent(&indent), "{\n");
                        *indent += 1;
                        next._to_string_recursion(ast_str, indent, ast_symbol_map);
                        *indent -= 1;
                        *ast_str += &format!("{}{}", get_indent(&indent), "}\n");
                        break;
                    }
                }
                
            }
            ASTType::Condiction(Relation) => {}
            ASTType::Condictions => {}
            ASTType::True => {
                for next in self.next.iter() {
                    next._to_string_recursion(ast_str, indent, ast_symbol_map);
                }
            }
            ASTType::False => {
                for next in self.next.iter() {
                    next._to_string_recursion(ast_str, indent, ast_symbol_map);
                }
            }
            ASTType::Operator(Operator) => {}
            ASTType::Variable(_) => {}
            ASTType::Integer(signed, size) => {}
            ASTType::Float(signed, size) => {}
            ASTType::Loop => {
                *ast_str += &format!("{}loop\n", get_indent(&indent));
                *ast_str += &format!("{}{}\n", get_indent(&indent), "{");
                *indent += 1;
                for next in self.next.iter() {
                    next._to_string_recursion(ast_str, indent, ast_symbol_map);
                }
                *indent -= 1;
                *ast_str += &format!("{}{}\n", get_indent(&indent), "}");
            }
            ASTType::Break => {
                *ast_str += &format!("{}break;\n", get_indent(&indent));
            }
            ASTType::Continue => {
                *ast_str += &format!("{}continue;\n", get_indent(&indent));
            }
            ASTType::While => {
                *ast_str += &format!("{}while", get_indent(&indent));
                *ast_str += "(";
                for next in self.next.iter() {
                    if next.ast_type == ASTType::Condictions {
                        get_condictions_str(next, ast_str, ast_symbol_map);
                        break;
                    }
                }  
                *ast_str += ")\n";
                *ast_str += &format!("{}{}\n", get_indent(&indent), "{");
                *indent += 1;
                for next in self.next.iter() {
                    if next.ast_type != ASTType::Condictions {
                        next._to_string_recursion(ast_str, indent, ast_symbol_map); 
                    }
                }
                *indent -= 1;
                *ast_str += &format!("{}{}\n", get_indent(&indent), "}");
            }
            _ => {}
        } 
    }

    
    pub fn search_type(&self, ast_type: &ASTType, target: &mut AbstractSyntaxTree) {
        if &self.ast_type == ast_type {
            *target = self.clone();
            return;
        } else {
            for next in self.next.iter() {
                next.search_type(ast_type, target);
            }
        }
    }

    pub fn update_return_var_type(&mut self, function_return_type_map: &HashMap<String, (ASTSymbolValueType, bool)>, ast_symbol_map: &mut HashMap<usize, ASTSymbol>) {
        if let ASTType::Function(name) = &self.ast_type {
            if let Some((var_type, is_ptr)) = function_return_type_map.get(name) {
                for next in self.next.iter() {
                    if let ASTType::Return = &next.ast_type {
                        let id = next.value as usize;
                        let symbol = ast_symbol_map.entry(id).or_insert(ASTSymbol::new(id));
                        symbol.select_type = var_type.clone();
                        /*
                        println!("{:?}", self);
                        println!("{:?}", symbol);
                        */
                        break;
                    }
                }
            } else {

            }
        } else if let ASTType::Begin(_) = &self.ast_type {
            for next in self.next.iter_mut() {
                next.update_return_var_type(function_return_type_map, ast_symbol_map); 
            }
        }
    }

}

fn get_indent(indent: &usize) -> String {
    let mut res = String::new();
    for _ in 0..*indent {
        res += "\t";
    }
    res
}

fn operator_to_string(op: &Operator) -> String {
    match op {
        Operator::Add => String::from("+"),
        Operator::Sub => String::from("-"),
        Operator::Mul => String::from("*"),
        Operator::Div => String::from("/"),
        Operator::And => String::from("&"),
        Operator::Or => String::from("|"),
        Operator::Xor => String::from("^"),
        Operator::Not => String::from("!"),
    }
}

fn convert_operator_to_string(operator: &Operator) -> String {
    match operator {
        Operator::Add => String::from("+"),
        Operator::Sub => String::from("-"),
        Operator::Mul => String::from("*"),
        Operator::Div => String::from("/"),
        Operator::And => String::from("&"),
        Operator::Or => String::from("|"),
        Operator::Xor => String::from("^"),
        Operator::Not => String::from("!"),
    }
}


fn convert_operator_to_string_in_condiction(operator: &Operator) -> String {
    let mut res = String::new();
    if let Operator::Or = operator {
        res += "||";
    } else if let Operator::And = operator {
        res += "&&";
    }
    res
}

fn astsymbolvaluetype_to_string(_type: &ASTSymbolValueType) -> String {
    match _type {
        ASTSymbolValueType::UnsignedChar => String::from("unsigned char"),
        ASTSymbolValueType::Char => String::from("char"),
        ASTSymbolValueType::UnsignedShort => String::from("unsigned short"),
        ASTSymbolValueType::Short => String::from("short"),
        ASTSymbolValueType::UnsignedInt => String::from("unsigned int"),
        ASTSymbolValueType::Int => String::from("int"),
        ASTSymbolValueType::UnsignedLong => String::from("unsigned long"),
        ASTSymbolValueType::Long => String::from("long"),
        ASTSymbolValueType::PtrUnsignedChar => String::from("unsigned char*"),
        ASTSymbolValueType::PtrChar => String::from("char*"),
        ASTSymbolValueType::PtrUnsignedShort => String::from("unsigned short*"),
        ASTSymbolValueType::PtrShort => String::from("short*"),
        ASTSymbolValueType::PtrUnsignedInt => String::from("unsigned int*"),
        ASTSymbolValueType::PtrInt => String::from("int*"),
        ASTSymbolValueType::PtrUnsignedLong => String::from("unsigned long*"),
        ASTSymbolValueType::PtrLong => String::from("long*"),
        ASTSymbolValueType::Unknown => String::from("unkown"),
        ASTSymbolValueType::Ptr => String::from("void*"),
    }
}

fn convert_astsymbolvaluetype_to_string(ast_symbol: &ASTSymbol, is_left_mode: bool, is_ptr: bool, left_sym: &ASTSymbol) -> String {
    let size = &ast_symbol.select_type;
    if is_left_mode {
        if is_ptr {
            match size {
                ASTSymbolValueType::Char => String::from("char*"),
                ASTSymbolValueType::UnsignedChar => String::from("unsigned char*"),
                ASTSymbolValueType::Short => String::from("short*"),
                ASTSymbolValueType::UnsignedShort => String::from("unsigned short*"),
                ASTSymbolValueType::Int => String::from("int*"),
                ASTSymbolValueType::UnsignedInt => String::from("unsigned int*"),                
                ASTSymbolValueType::Long => String::from("long*"),
                ASTSymbolValueType::UnsignedLong => String::from("unsigned long*"),

                ASTSymbolValueType::PtrChar => String::from("char*"),
                ASTSymbolValueType::PtrUnsignedChar => String::from("unsigned char*"),
                ASTSymbolValueType::PtrShort => String::from("short*"),
                ASTSymbolValueType::PtrUnsignedShort => String::from("unsigned short*"),
                ASTSymbolValueType::PtrInt => String::from("int*"),
                ASTSymbolValueType::PtrUnsignedInt => String::from("unsigned int*"),                
                ASTSymbolValueType::PtrLong => String::from("long*"),
                ASTSymbolValueType::PtrUnsignedLong => String::from("unsigned long*"),
                ASTSymbolValueType::Unknown => String::from("unkown"),
                ASTSymbolValueType::Ptr => String::from("ptr"),
                _ => String::new(),
            }
        } else {
            match size {
                ASTSymbolValueType::Char => String::from("char"),
                ASTSymbolValueType::UnsignedChar => String::from("unsigned char"),
                ASTSymbolValueType::Short => String::from("short"),
                ASTSymbolValueType::UnsignedShort => String::from("unsigned short"),
                ASTSymbolValueType::Int => String::from("int"),
                ASTSymbolValueType::UnsignedInt => String::from("unsigned int"),
                ASTSymbolValueType::Long => String::from("long"),
                ASTSymbolValueType::UnsignedLong => String::from("unsigned long"),
                ASTSymbolValueType::PtrChar => String::from("*(char*)"),
                ASTSymbolValueType::PtrUnsignedChar => String::from("*(unsigned char*)"),
                ASTSymbolValueType::PtrShort => String::from("*(short*)"),
                ASTSymbolValueType::PtrUnsignedShort => String::from("*(unsigned short*)"),
                ASTSymbolValueType::PtrInt => String::from("*(int*)"),
                ASTSymbolValueType::PtrUnsignedInt => String::from("*(unsigned int*)"),
                ASTSymbolValueType::PtrLong => String::from("*(long*)"),
                ASTSymbolValueType::PtrUnsignedLong => String::from("*(unsigned long*)"),
                //ASTSymbolValueType::PtrInt => String::from("*(int*)"),
                ASTSymbolValueType::Unknown => String::from("unkown"),
                ASTSymbolValueType::Ptr => String::from("ptr"),
               
                _ => String::new(),
            }  
        }  
    } else {
        if is_ptr {
            match size {
                //ASTSymbolValueType::Int => String::from("(int*)&"),  
                ASTSymbolValueType::Char => String::from("(char*)&"),
                ASTSymbolValueType::UnsignedChar => String::from("(unsigned char*)&"),
                ASTSymbolValueType::Short => String::from("(short*)&"),
                ASTSymbolValueType::UnsignedShort => String::from("(unsigned short*)&"),
                ASTSymbolValueType::Int => String::from("(int*)&"),
                ASTSymbolValueType::UnsignedInt => String::from("(unsigned int*)&"),
                ASTSymbolValueType::Long => String::from("(long*)&"),
                ASTSymbolValueType::UnsignedLong => String::from("(unsigned long*)&"),
                ASTSymbolValueType::PtrChar => String::from("(char*)"),
                ASTSymbolValueType::PtrUnsignedChar => String::from("(unsigned char*)"),
                ASTSymbolValueType::PtrShort => String::from("(short*)"),
                ASTSymbolValueType::PtrUnsignedShort => String::from("(unsigned short*)"),
                ASTSymbolValueType::PtrInt => String::from("(int*)"),
                ASTSymbolValueType::PtrUnsignedInt => String::from("(unsigned int*)"),
                ASTSymbolValueType::PtrLong => String::from("(long*)"),
                ASTSymbolValueType::PtrUnsignedLong => String::from("(unsigned long*)"),
                ASTSymbolValueType::Unknown => String::from("unkown"),
                ASTSymbolValueType::Ptr => String::from("ptr"),

                _ => String::new(),
            }
        } else {
            match size {
                ASTSymbolValueType::Char => String::from("char"),
                ASTSymbolValueType::UnsignedChar => String::from("unsigned char"),
                ASTSymbolValueType::Short => String::from("short"),
                ASTSymbolValueType::UnsignedShort => String::from("unsigned short"),
                ASTSymbolValueType::Int => String::from("int"),
                ASTSymbolValueType::UnsignedInt => String::from("unsigned int"),
                ASTSymbolValueType::Long => String::from("long"),
                ASTSymbolValueType::UnsignedLong => String::from("unsigned long"),
                ASTSymbolValueType::PtrChar => String::from("*(char*)"),
                ASTSymbolValueType::PtrUnsignedChar => String::from("*(unsigned char*)"),
                ASTSymbolValueType::PtrShort => String::from("*(short*)"),
                ASTSymbolValueType::PtrUnsignedShort => String::from("*(unsigned short*)"),
                ASTSymbolValueType::PtrInt => String::from("*(int*)"),
                ASTSymbolValueType::PtrUnsignedInt => String::from("*(unsigned int*)"),
                ASTSymbolValueType::PtrLong => String::from("*(long*)"),
                ASTSymbolValueType::PtrUnsignedLong => String::from("*(unsigned long*)"),
                //ASTSymbolValueType::Int => String::from("int"),
                //ASTSymbolValueType::PtrInt => String::from("*(int*)"), 
                ASTSymbolValueType::Unknown => String::from("unkown"),
                ASTSymbolValueType::Ptr => String::from("ptr"),

                _ => String::new(),
            }
        }
    }
}

fn return_type_to_string(return_type: &ASTSymbolValueType, is_ptr: bool) -> String {
    if is_ptr {
        match return_type {
            ASTSymbolValueType::Char => String::from("char*"),
            ASTSymbolValueType::UnsignedChar => String::from("unsigned char*"),
            ASTSymbolValueType::Short => String::from("short*"),
            ASTSymbolValueType::UnsignedShort => String::from("unsigned short*"),
            ASTSymbolValueType::Int => String::from("int*"),
            ASTSymbolValueType::UnsignedInt => String::from("unsigned int*"),
            ASTSymbolValueType::Long => String::from("long*"),
            ASTSymbolValueType::UnsignedLong => String::from("unsigned long*"),
            ASTSymbolValueType::PtrChar => String::from("*(char*)"),
            ASTSymbolValueType::PtrUnsignedChar => String::from("*(unsigned char*)"),
            ASTSymbolValueType::PtrShort => String::from("*(short*)"),
            ASTSymbolValueType::PtrUnsignedShort => String::from("*(unsigned short*)"),
            ASTSymbolValueType::PtrInt => String::from("*(int*)"),
            ASTSymbolValueType::PtrUnsignedInt => String::from("*(unsigned int*)"),
            ASTSymbolValueType::PtrLong => String::from("*(long*)"),
            ASTSymbolValueType::PtrUnsignedLong => String::from("*(unsigned long*)"),
            ASTSymbolValueType::Unknown => String::from("void"),
            ASTSymbolValueType::Ptr => String::from("ptr"),
        }
    } else {
        match return_type {
            ASTSymbolValueType::Char => String::from("char"),
            ASTSymbolValueType::UnsignedChar => String::from("unsigned char"),
            ASTSymbolValueType::Short => String::from("short"),
            ASTSymbolValueType::UnsignedShort => String::from("unsigned short"),
            ASTSymbolValueType::Int => String::from("int"),
            ASTSymbolValueType::UnsignedInt => String::from("unsigned int"),
            ASTSymbolValueType::Long => String::from("long"),
            ASTSymbolValueType::UnsignedLong => String::from("unsigned long"),
            ASTSymbolValueType::PtrChar => String::from("char*"),
            ASTSymbolValueType::PtrUnsignedChar => String::from("unsigned char*"),
            ASTSymbolValueType::PtrShort => String::from("short*"),
            ASTSymbolValueType::PtrUnsignedShort => String::from("unsigned short*"),
            ASTSymbolValueType::PtrInt => String::from("int*"),
            ASTSymbolValueType::PtrUnsignedInt => String::from("unsigned int*"),
            ASTSymbolValueType::PtrLong => String::from("long*"),
            ASTSymbolValueType::PtrUnsignedLong => String::from("unsigned long*"),
            ASTSymbolValueType::Unknown => String::from("void"),
            ASTSymbolValueType::Ptr => String::from("ptr"),
        }
    }
}


fn get_ast_type_from_size(size: &Size) -> ASTSymbolValueType {
    match size {
        Size::Signed8 => ASTSymbolValueType::Char,
        Size::Unsigned8 => ASTSymbolValueType::UnsignedChar,
        Size::Signed16 => ASTSymbolValueType::Short,
        Size::Unsigned16 => ASTSymbolValueType::UnsignedShort,
        Size::Signed32 => ASTSymbolValueType::Int,
        Size::Unsigned32 => ASTSymbolValueType::UnsignedInt,
        Size::Signed64 => ASTSymbolValueType::Long,
        Size::Unsigned64 => ASTSymbolValueType::UnsignedLong,           
    }
}

fn get_condictions_str(tree: &Box<AbstractSyntaxTree>, conds_str: &mut String, ast_symbol_map: &HashMap<usize, ASTSymbol>) {
    if let ASTType::Condictions = &tree.ast_type {
        for next in tree.next.iter() {
            get_condictions_str(next, conds_str, ast_symbol_map);
        }
    } else if let ASTType::Operator(o) = &tree.ast_type {
        //*conds_str += "(";
        for next in tree.next.iter() {
            get_condictions_str(next, conds_str, ast_symbol_map); 
            if *next != *tree.next.last().unwrap() {
                *conds_str += &format!(" {} ", convert_operator_to_string_in_condiction(o));
            }
        }
        //*conds_str += ")";
    } else if let ASTType::Condiction(relation) = &tree.ast_type {
        if let Some(operand1) = tree.next.first() {
			//*conds_str += &format!("var{}", operand1.value);
            match &operand1.ast_type {
                ASTType::Variable(is_ptr) => *conds_str += &variable_to_string(&operand1, ast_symbol_map, true),
                ASTType::Integer(_, _) => {
                    *conds_str += &integer_to_string(&operand1);
                }
                _ => panic!("error"),
            }
        }

		*conds_str += &format!(" {} ", relation);

		if let Some(operand2) = tree.next.last() {
            match &operand2.ast_type {
                ASTType::Variable(is_ptr) => *conds_str += &variable_to_string(&operand2, ast_symbol_map, true),
                ASTType::Integer(_, _) => *conds_str += &integer_to_string(&operand2),
                _ => panic!("error"),
            }
		}
    }
    /*
    for next in tree.next.iter() {
        if let ASTType::Operator(o) = &next.ast_type {
            println!("{:?}", o);
            *conds_str += "(";
            //println!("{}", "(");
            for n in next.next.iter() {
                //println!("{:#?}", n);
                get_condictions_str(next, conds_str);
                if *n != *next.next.last().unwrap() {
                    *conds_str += &format!(" {:?} ", o);
                    //println!(" {:?} ", o);
                    //panic!("{:#?}", n);
                }
            }
            *conds_str += ")";
            //println!("{}", ")");
        } else if let ASTType::Condiction(relation) = &next.ast_type {
            if let Some(operand1) = next.next.first() {
			    *conds_str += &format!("var{}", operand1.value);
			}

			*conds_str += &format!(" {} ", relation);

			if let Some(operand2) = next.next.last() {
			    *conds_str += &format!("var{}", operand2.value);
			}
			break;

        }
    }
    */
    /*
    if let ASTType::Condictions = &tree.ast_type {
        for next in tree.next.iter() {
            get_condictions_str(next, conds_str);
        } 
    } else if let ASTType::Operator(o) = &tree.ast_type {
        
    } else {

    }
    */
}
fn assign_to_string(tree: &AbstractSyntaxTree, assign_str: &mut String, ast_symbol_map: &HashMap<usize, ASTSymbol>, type_set: &mut HashMap<usize, bool>) {
    match &tree.ast_type {
        ASTType::Assign(is_ptr) => {
            
            let sym = &ast_symbol_map[&(tree.value as usize)];
            //*assign_str += &format!("{} {:?} ", is_ptr, sym.scope);
            let mut is_value = false;
            if let None = type_set.get(&(tree.value as usize)) {
                let sym = &ast_symbol_map[&(tree.value as usize)];
                *assign_str += &format!("{} ", astvaluetype_to_string(&sym.select_type));
                type_set.insert(tree.value as usize, *is_ptr);
                let mut exist_ptr = false;
                test_is_value(tree, ast_symbol_map, &mut exist_ptr);
                if exist_ptr == true {
                    is_value = true;
                }

                match &sym.scope {
                    Scope::Temp => {
                        if is_type_ptr(&sym.select_type) {
                            if *is_ptr == false {
                                is_value = false;
                            }
                        } 
                    }
                    Scope::Local => {
                        if is_type_ptr(&sym.select_type) {
                            if *is_ptr == true {
                                is_value = false;
                            }
                        }
                    }
                    Scope::Global => {}
                } 
            } else {
                let type_is_ptr = type_set[&(tree.value as usize)];
                let sym = &ast_symbol_map[&(tree.value as usize)];
                if type_is_ptr != *is_ptr {
                    match &sym.scope {
                        Scope::Temp => {
                            if *is_ptr {
                                *assign_str += "*";
                                is_value = true;
                            }
                        }
                        Scope::Local => {
                        }
                        Scope::Global => {}
                    }
                }
            }
            *assign_str += &format!("var{} = ", tree.value);            
            if is_value {
                let mut is_exist_ptr = false;
                test_is_value(tree, ast_symbol_map, &mut is_exist_ptr);
                if is_exist_ptr {
                    *assign_str += "*";
                }
            }
            for next in tree.next.iter() {
                assign_to_string(next, assign_str, ast_symbol_map, type_set);
            }
        }
        ASTType::Operator(op) => {
            *assign_str += "(";
            assign_to_string(tree.next.first().unwrap(), assign_str, ast_symbol_map, type_set);
            *assign_str += &format!(" {} ", operator_to_string(op));
            assign_to_string(tree.next.last().unwrap(), assign_str, ast_symbol_map, type_set);
            *assign_str += ")";
        }
        ASTType::Variable(is_ptr) => {
            let sym = &ast_symbol_map[&(tree.value as usize)];
            match &sym.scope {
                Scope::Global => {
                    if !*is_ptr {
                        *assign_str += "&";
                    }
                    *assign_str += &format!("g_var{}", tree.value);
                }
                Scope::Local => {
                    //*assign_str += &format!("{} {:?} var{}", is_ptr, sym.scope, tree.value);
                    if !*is_ptr {
                        *assign_str += "&";
                    }
                    *assign_str += &format!("var{}", tree.value);
                }
                Scope::Temp => {
                    /*
                    if *is_ptr {
                        *assign_str += &format!("{} {:?} *var{}", is_ptr, sym.scope, tree.value);
                    } else {
                        *assign_str += &format!("{} {:?} var{}", is_ptr, sym.scope, tree.value);
                    }
                    */
                    if *is_ptr {
                        *assign_str += "*";
                    }
                    *assign_str += &format!("var{}", tree.value);
                }
            }
        }
        ASTType::Integer(is_signed, _) => {
            if *is_signed {
                *assign_str += &format!("{}", tree.value as isize);
            } else {
                *assign_str += &format!("{}", tree.value);
            }
        }
        _ => {}
    }
}

fn _assign_to_string(tree: &AbstractSyntaxTree, assign_str: &mut String, ast_symbol_map: &HashMap<usize, ASTSymbol>, flag: &mut usize, left_sym: &ASTSymbol) {
    if let ASTType::Assign(is_ptr) = &tree.ast_type {
        let symbol = match ast_symbol_map.get(&(tree.value as usize)) {
            Some(sym) => sym,
            None => {
                for sym in ast_symbol_map.iter() {
                    println!("{:?}", sym);
                }
                println!("\n{:?}", tree.value);
                panic!("error");
            }
        };
        *assign_str += &format!("{} var{} = ", type_to_string(tree, ast_symbol_map, *is_ptr, left_sym), symbol.id);
        for next in tree.next.iter() {
            _assign_to_string(next, assign_str, ast_symbol_map, flag, symbol);
        }
        *assign_str += ";";
    } else if let ASTType::Operator(o) = &tree.ast_type {
        /*
        if let o = Operator::Add {
            println!("+");
        }
        */
        *assign_str += "(";
        for (i, next) in tree.next.iter().enumerate() {
            _assign_to_string(next, assign_str, ast_symbol_map, flag, left_sym);
            if i != tree.next.len() - 1 {
                *assign_str += &format!(" {} ", convert_operator_to_string(o));
            }
        }
        *assign_str += ")";
    } else {
        match &tree.ast_type {
            ASTType::Integer(signed, _) => {
                if *signed {
                    *assign_str += &format!("{}", tree.value as isize);
                } else {
                    *assign_str += &format!("{}", tree.value as usize);
                }
            }
            ASTType::Variable(is_ptr) => {
                let symbol = &ast_symbol_map[&(tree.value as usize)];
                match &symbol.scope {
                    Scope::Temp => {
                        *assign_str += &format!("{} var{}", type_to_string(tree, ast_symbol_map, *is_ptr, left_sym), symbol.id);
                    }
                    Scope::Local => {
                        *assign_str += &format!("{} var{}", type_to_string(tree, ast_symbol_map, *is_ptr, left_sym), symbol.id); 
                    }
                    Scope::Global => {
                        *assign_str += &format!("var{}", symbol.id);
                    }
                }
            }
            _ => {}//panic!("error: {:?}", tree),
        }
    }
}

fn integer_to_string(tree: &AbstractSyntaxTree) -> String {
    if let ASTType::Integer(signed, _) = tree.ast_type {
        if signed {
            format!("{}", tree.value as isize)
        } else {
            format!("{}", tree.value as usize)
        }
    } else {
        panic!("error");
    }
}

fn variable_to_string(tree: &AbstractSyntaxTree, ast_symbol_map: &HashMap<usize, ASTSymbol>, is_in_condiction: bool) -> String {
    if is_in_condiction {
        let symbol = &ast_symbol_map[&(tree.value as usize)];
        return format!("var{}", symbol.id);
        /*
        return match symbol.scope {
            Scope::Temp => format!("{} var{}", type_to_string(tree, ast_symbol_map, false, &ASTSymbol::new(usize::MAX)), symbol.id),
            Scope::Local => format!("{} var{}", type_to_string(tree, ast_symbol_map, false, &ASTSymbol::new(usize::MAX)), symbol.id),
            Scope::Global => format!("var{}", symbol.id),
        };
        */
    }
    match &tree.ast_type {
        ASTType::Variable(is_ptr) => {
            let symbol = &ast_symbol_map[&(tree.value as usize)];
            match symbol.scope {
                Scope::Temp => format!("{} var{}", type_to_string(tree, ast_symbol_map, *is_ptr, &ASTSymbol::new(usize::MAX)), symbol.id),
                Scope::Local => format!("{} var{}", type_to_string(tree, ast_symbol_map, *is_ptr, &ASTSymbol::new(usize::MAX)), symbol.id),
                Scope::Global => format!("var{}", symbol.id),
            }
        }

        ASTType::Return => {
            let symbol = &ast_symbol_map[&(tree.value as usize)];
            match symbol.scope {
                Scope::Temp => format!("{} var{}", type_to_string(tree, ast_symbol_map, false, &ASTSymbol::new(usize::MAX)), symbol.id), 
                Scope::Local => format!("var{}", symbol.id),
                Scope::Global => format!("var{}", symbol.id),
            }
        }

        _ => panic!("error"),
    }
    /*
    if ASTType::Variable == tree.ast_type || ASTType::Return == tree.ast_type {
        let symbol = &ast_symbol_map[&tree.value];
        match symbol.scope {
            Scope::Temp => format!("{} var{}", convert_astsymbolvaluetype_to_string(&ast_symbol_map[&symbol.id].select_type), symbol.id),
            Scope::Local => format!("var{}", symbol.id),
            Scope::Global => format!("var{}", symbol.id),
        }
    } else {
        panic!("error");
    }
    */
}

fn type_to_string(tree: &AbstractSyntaxTree, ast_symbol_map: &HashMap<usize, ASTSymbol>, is_ptr: bool, left_sym: &ASTSymbol) -> String {
    let sym = &ast_symbol_map[&(tree.value as usize)];
    if let ASTType::Assign(_) = tree.ast_type {
        format!("{}", convert_astsymbolvaluetype_to_string(sym, true, is_ptr, left_sym))
    } else if let ASTType::Return = tree.ast_type {
        format!("{}", convert_astsymbolvaluetype_to_string(sym, true, false, left_sym))
    } else {
        format!("{}", convert_astsymbolvaluetype_to_string(sym, false, is_ptr, left_sym))
    } 
}


fn create_symbol(symbol: &DFISymbolRecord, address_symbol_map: &mut HashMap<(Address, usize), usize>, ast_symbol_map: &mut HashMap<usize, ASTSymbol>, counter: &mut Counter) -> ASTSymbol {
	let mut ast_symbol = match address_symbol_map.get(&(symbol.address.clone(), symbol.id)) {
	    Some(symbol) => {
            ast_symbol_map[symbol].clone()
        }
	    None => {
	        let sid = counter.get();
	        address_symbol_map.insert((symbol.address.clone(), symbol.id), sid);
	        let mut ast_symbol = ASTSymbol::new(sid);
            ast_symbol.select_type = get_ast_type_from_size(&symbol.size);

            ast_symbol
	    }
	};
    let sid = ast_symbol.id;
	let symtype = &symbol.sym_type;
	//let sid = symbol.id;
	let size = &symbol.size;
	let is_value = symbol.value;


    /*
    if !is_value {
        if let Some(sym) = ast_symbol_map.get(&sid) {
            ast_symbol.select_type = sym.select_type.clone();
        } else {
            ast_symbol.select_type = ASTSymbolValueType::Unknown;
        }
    } else {
	    ast_symbol.select_type = get_ast_type_from_size(size);
    }
    */
	
	match &symbol.address {
	    Address::Stack(stack) => {
	        ast_symbol.scope = Scope::Local;
	    }
	    Address::Memory(memory) => {
	        ast_symbol.scope = Scope::Global;
	    }
	    Address::GR(gr) => {
	        ast_symbol.scope = Scope::Temp;   
	    }
	    Address::FR(fr) => {
	        ast_symbol.scope = Scope::Temp;
	    }
	}
        

    ast_symbol_map.insert(ast_symbol.id, ast_symbol.clone());
    ast_symbol
}

fn type_change_to_ptr(_type: &ASTSymbolValueType) -> ASTSymbolValueType {
    match _type {
        ASTSymbolValueType::UnsignedChar => ASTSymbolValueType::PtrUnsignedChar,
        ASTSymbolValueType::Char => ASTSymbolValueType::PtrChar,
        ASTSymbolValueType::UnsignedShort => ASTSymbolValueType::PtrUnsignedShort,
        ASTSymbolValueType::Short => ASTSymbolValueType::PtrShort,
        ASTSymbolValueType::UnsignedInt => ASTSymbolValueType::PtrUnsignedInt,
        ASTSymbolValueType::Int => ASTSymbolValueType::PtrInt,
        ASTSymbolValueType::UnsignedLong => ASTSymbolValueType::PtrUnsignedLong,
        ASTSymbolValueType::Long => ASTSymbolValueType::PtrLong,
        ASTSymbolValueType::Unknown => ASTSymbolValueType::Ptr,
        _ => panic!("type error"),
    }
}

fn get_number_type(size: &Size, signed: bool) -> ASTSymbolValueType {
    if signed {
        match size {
            Size::Signed8 => ASTSymbolValueType::Char,
            Size::Signed16 => ASTSymbolValueType::Short,
            Size::Signed32 => ASTSymbolValueType::Int,
            Size::Signed64 => ASTSymbolValueType::Long,
            _ => {
                panic!("{:?}", size);
            }
        }
    } else {
        match size {
            Size::Unsigned8 => ASTSymbolValueType::UnsignedChar,
            Size::Unsigned16 => ASTSymbolValueType::UnsignedShort,
            Size::Unsigned32 => ASTSymbolValueType::UnsignedInt,
            Size::Unsigned64 => ASTSymbolValueType::UnsignedLong,
            _ => panic!("{:?}", size),
        }
    }
}

pub fn parameter_to_string(_type: &ASTSymbolValueType) -> String {
    match _type {
        ASTSymbolValueType::UnsignedChar => String::from("unsigned char"),
        ASTSymbolValueType::Char => String::from("char"),
        ASTSymbolValueType::UnsignedShort => String::from("unsigned short"),
        ASTSymbolValueType::Short => String::from("short"),
        ASTSymbolValueType::UnsignedInt => String::from("unsigned short"),
        ASTSymbolValueType::Int => String::from("int"),
        ASTSymbolValueType::UnsignedLong => String::from("unsigned long"),
        ASTSymbolValueType::Long => String::from("long"),
        ASTSymbolValueType::PtrUnsignedChar => String::from("unsigned char*"),
        ASTSymbolValueType::PtrChar => String::from("char*"),
        ASTSymbolValueType::PtrUnsignedShort => String::from("unsigned short*"),
        ASTSymbolValueType::PtrShort => String::from("short*"),
        ASTSymbolValueType::PtrUnsignedInt => String::from("unsigned int*"),
        ASTSymbolValueType::PtrInt => String::from("int*"),
        ASTSymbolValueType::PtrUnsignedLong => String::from("unsigned long*"),
        ASTSymbolValueType::PtrLong => String::from("long*"),
        ASTSymbolValueType::Unknown => String::from("unkown"),
        ASTSymbolValueType::Ptr => String::from("ptr"),

    }
}


fn astvaluetype_to_string(ast_type: &ASTSymbolValueType) -> String {
    match ast_type {
        ASTSymbolValueType::UnsignedChar => String::from("unsigned char"),
        ASTSymbolValueType::Char => String::from("char"),
        ASTSymbolValueType::UnsignedShort => String::from("unsigned short"),
        ASTSymbolValueType::Short => String::from("short"),
        ASTSymbolValueType::UnsignedInt => String::from("unsigned int"),
        ASTSymbolValueType::Int => String::from("int"),
        ASTSymbolValueType::UnsignedLong => String::from("unsigned long"),
        ASTSymbolValueType::Long => String::from("long"),
        ASTSymbolValueType::PtrUnsignedChar => String::from("unsigned char*"),
        ASTSymbolValueType::PtrChar => String::from("char*"),
        ASTSymbolValueType::PtrUnsignedShort => String::from("unsigned short*"),
        ASTSymbolValueType::PtrShort => String::from("short*"),
        ASTSymbolValueType::PtrUnsignedInt => String::from("unsigned int*"),
        ASTSymbolValueType::PtrInt => String::from("int*"),
        ASTSymbolValueType::PtrUnsignedLong => String::from("unsigned long*"),
        ASTSymbolValueType::PtrLong => String::from("long*"),
        ASTSymbolValueType::Unknown => String::from("unkown"),
        ASTSymbolValueType::Ptr => String::from("void*"),
    }
}


fn number_to_string(num: u64, is_signed: bool) -> String {
    if is_signed {
        format!("{}", num as i64)
    } else {
        format!("{}", num)
    }
}

pub fn is_ptr(ast_type: &ASTSymbolValueType) -> bool {
    match ast_type {
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
        ASTSymbolValueType::Ptr => panic!("error"),

    }
}

pub fn is_type_ptr(ast_type: &ASTSymbolValueType) -> bool {
    match ast_type {
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
        ASTSymbolValueType::Ptr => panic!("error"),

    }
}

fn test_is_value(ast: &AbstractSyntaxTree, ast_symbol_map: &HashMap<usize, ASTSymbol>, is_exist_ptr: &mut bool) {
    match &ast.ast_type {
        ASTType::Variable(is_val) => {
            let sym = &ast_symbol_map[&(ast.value as usize)];
            if (sym.scope == Scope::Temp && *is_val == false && is_ptr(&sym.select_type) == true) || (sym.scope == Scope::Local && *is_val == true && is_ptr(&sym.select_type) == true) {
                *is_exist_ptr = true;
            }
        }
        _ => {
            for next in ast.next.iter() {
                test_is_value(next, ast_symbol_map, is_exist_ptr);
            }
        }
    }
}


