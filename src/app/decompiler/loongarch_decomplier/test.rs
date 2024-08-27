//use crate::loongarch_decomplier::*;
use crate::app::decompiler::loongarch_decomplier::*;

#[test]
fn test_stack_eq() {
    let s1 = DFISymbolRecord {
        address: Address::Stack(-64),
        sym_type: DFISymbolType::Local,
        id: 0,
        size: Size::Unsigned64,
        value: true,
    };

    let mut s2 = s1.clone();
    s2.value = false;

    assert_eq!(s1.clone(), s2.clone());

    let mut s3 = s1.clone();
    s3.address = Address::Stack(20);

    assert_ne!(s1.clone(), s3);

    let g1 = DFISymbolRecord {
        address: Address::Memory(100000),
        sym_type: DFISymbolType::Global,
        id: 1,
        size: Size::Signed16,
        value: true,
    };

    assert_ne!(s1.clone(), g1.clone());
}


fn test_control_flow_tree() {
    
}

#[test]
fn test_relation() {
    let relation = Relation::L;
    let relation = !relation;
    assert_eq!(relation, Relation::GE);
}

/*
#[test]
//#[ignore = ""]
fn test_sign_adjust() {
    let mut ir_ast = AbstractSyntaxTree::new();
    ir_ast.ast_type = ASTType::Assign;
    ir_ast.value = 0;

    let mut op_ast = AbstractSyntaxTree::new();
    op_ast.ast_type = ASTType::Operator(Operator::Add);

    let mut var_ast = AbstractSyntaxTree::new();
    var_ast.value = 1;
    var_ast.ast_type = ASTType::Variable(false);

    let mut num_ast = AbstractSyntaxTree::new();
    num_ast.ast_type = ASTType::Integer(true);
    num_ast.value = (-1isize) as usize;

    op_ast.next.push(Box::new(var_ast.clone()));
    op_ast.next.push(Box::new(num_ast.clone()));

    ir_ast.next.push(Box::new(op_ast.clone()));

    let mut ast_symbol_map = HashMap::<usize, ASTSymbol>::new();
    ast_symbol_map.insert(0, ASTSymbol {id: 0, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0)});
    ast_symbol_map.insert(1, ASTSymbol {id: 1, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(1)});

    println!("{}", ir_ast.to_string(&ast_symbol_map));

    sign_adjust(&mut ir_ast);
    let sign_adjust_ast = ir_ast.clone();

    println!("{}", ir_ast.to_string(&ast_symbol_map));

    op_ast.ast_type = ASTType::Operator(Operator::Sub);
    num_ast.value = 1;
    
    op_ast.next.clear();
    op_ast.next.push(Box::new(var_ast));
    op_ast.next.push(Box::new(num_ast));

    ir_ast.next.clear();
    ir_ast.next.push(Box::new(op_ast));

    assert_eq!(sign_adjust_ast, ir_ast);
    
    /*
    let mut ir_ast = AbstractSyntaxTree::new();
    ir_ast.ast_type = ASTType::Assign;

    let mut opcode_ast = AbstractSyntaxTree::new();
    opcode_ast.ast_type = ASTType::Operator(Operator::Add);

    let mut operand1_ast = AbstractSyntaxTree::new();
    operand1_ast.value = -1isize as usize;
    operand1_ast.ast_type = ASTType::Integer(true);

    let mut operand2_ast = AbstractSyntaxTree::new();
    operand2_ast.value = 1;
    operand2_ast.ast_type = ASTType::Integer(true);

    opcode_ast.next.push(Box::new(operand1_ast.clone()));
    opcode_ast.next.push(Box::new(operand2_ast.clone()));

    ir_ast.next.push(Box::new(opcode_ast));

    let mut after_adjust_ast = AbstractSyntaxTree::new();
    after_adjust_ast.ast_type = ASTType::Assign;

    let mut opcode_ast = AbstractSyntaxTree::new();
    opcode_ast.ast_type = ASTType::Operator(Operator::Sub);

    opcode_ast.next.push(Box::new(operand2_ast.clone()));
    opcode_ast.next.push(Box::new(operand2_ast.clone()));

    after_adjust_ast.next.push(Box::new(opcode_ast));

    sign_adjust(&mut ir_ast);
    assert_eq!(after_adjust_ast, ir_ast);
    */
}

#[ignore = ""]
#[test]
fn test_constant_folding() {
    let (mut ir_ast, ast_symbol_map) = test_variable_propagation();
    println!("{}", ir_ast.to_string(&ast_symbol_map));
    constant_folding(&mut ir_ast);
    println!("{}", ir_ast.to_string(&ast_symbol_map));
    panic!("");
    /*
    let mut before_constant_folding_ast = AbstractSyntaxTree::new();
    before_constant_folding_ast.ast_type = ASTType::Begin("test".to_string());

    let mut assign_ast = AbstractSyntaxTree::new();
    assign_ast.ast_type = ASTType::Assign;
    
    let mut opcode_ast = AbstractSyntaxTree::new();
    opcode_ast.ast_type = ASTType::Operator(Operator::Add);

    let mut operand1 = AbstractSyntaxTree::new();
    operand1.ast_type = ASTType::Integer(true);
    operand1.value = (-2isize) as usize;

    let mut operand2 = AbstractSyntaxTree::new();
    operand2.value = 3;
    operand2.ast_type = ASTType::Integer(true);

    opcode_ast.next.push(Box::new(operand1));
    opcode_ast.next.push(Box::new(operand2));

    assign_ast.next.push(Box::new(opcode_ast));
    before_constant_folding_ast.next.push(Box::new(assign_ast));

    let mut after_constant_folding_ast = AbstractSyntaxTree::new();
    after_constant_folding_ast.ast_type = ASTType::Begin("test".to_string());

    let mut assign_ast = AbstractSyntaxTree::new();
    assign_ast.ast_type = ASTType::Assign;

    let mut value_ast = AbstractSyntaxTree::new();
    value_ast.value = ((-2 as isize) + 3) as usize;
    value_ast.ast_type = ASTType::Integer(true);

    assign_ast.next.push(Box::new(value_ast));
    after_constant_folding_ast.next.push(Box::new(assign_ast));

    println!("before_constant_folding_ast: \n{:#?}\n", before_constant_folding_ast);

    constant_folding(&mut before_constant_folding_ast);
    println!("after_constant_folding_ast: \n{:#?}\n", after_constant_folding_ast);
    assert_eq!(before_constant_folding_ast, after_constant_folding_ast);
    */
}

#[ignore = ""]
//#[test]
fn test_variable_propagation() -> (AbstractSyntaxTree, HashMap<usize, ASTSymbol>) {
    let mut irs_ast = AbstractSyntaxTree::new();
    irs_ast.ast_type = ASTType::Begin(String::from("test"));
    
    let mut ir1_ast = AbstractSyntaxTree::new();
    ir1_ast.value = 1;
    ir1_ast.ast_type = ASTType::Assign;

    let mut ir2_ast = AbstractSyntaxTree::new();
    ir2_ast.value = 2;
    ir2_ast.ast_type = ASTType::Assign;

    let mut ir3_ast = AbstractSyntaxTree::new();
    ir3_ast.value = 3;
    ir3_ast.ast_type = ASTType::Assign;

    let mut ir4_ast = AbstractSyntaxTree::new();
    ir4_ast.ast_type = ASTType::Assign;
    ir4_ast.value = 4;

    let mut mul_ast = AbstractSyntaxTree::new();
    mul_ast.ast_type = ASTType::Operator(Operator::Mul);

    let mut div_ast = AbstractSyntaxTree::new();
    div_ast.ast_type = ASTType::Operator(Operator::Div);

    let mut sub_ast = AbstractSyntaxTree::new();
    sub_ast.ast_type = ASTType::Operator(Operator::Sub);

    let operand_val_0 = AbstractSyntaxTree {
        ast_type: ASTType::Variable(false),
        value: 0,
        next: Vec::new(),
    };

    let operand_5 = AbstractSyntaxTree {
        ast_type: ASTType::Integer(true),
        value: 5,
        next: Vec::new(),
    };

    let operand_7 = AbstractSyntaxTree {
        ast_type: ASTType::Integer(true),
        value: 7,
        next: Vec::new(),
    };

    let operand_i = AbstractSyntaxTree {
        ast_type: ASTType::Variable(false),
        value: 1,
        next: Vec::new(),
    };

    let operand_6 = AbstractSyntaxTree {
        ast_type: ASTType::Integer(true),
        value: 6,
        next: Vec::new(),
    };

    let operand_j = AbstractSyntaxTree {
        ast_type: ASTType::Variable(false),
        value: 2,
        next: Vec::new(),
    };

    let operand_4 = AbstractSyntaxTree {
        ast_type: ASTType::Integer(true),
        value: 4,
        next: Vec::new(),
    };

    mul_ast.next.push(Box::new(operand_5.clone()));
    mul_ast.next.push(Box::new(operand_7.clone()));

    div_ast.next.push(Box::new(operand_i.clone()));
    div_ast.next.push(Box::new(operand_6.clone()));

    sub_ast.next.push(Box::new(operand_j.clone()));
    sub_ast.next.push(Box::new(operand_4.clone()));

    ir1_ast.next.push(Box::new(mul_ast.clone()));
    ir2_ast.next.push(Box::new(div_ast.clone())); 
    ir3_ast.next.push(Box::new(sub_ast.clone()));
    ir4_ast.next.push(Box::new(sub_ast.clone()));

    irs_ast.next.push(Box::new(ir1_ast.clone()));
    //irs_ast.next.push(Box::new(ir4_ast.clone()));
    irs_ast.next.push(Box::new(ir2_ast.clone()));
    irs_ast.next.push(Box::new(ir3_ast.clone()));

    

    let before_variable_propagation_ast = irs_ast.clone();
    let mut ast_symbol_map = HashMap::<usize, ASTSymbol>::new();
    ast_symbol_map.insert(0, ASTSymbol {id: 0, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0)});
    ast_symbol_map.insert(1, ASTSymbol {id: 1, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(1)});
    ast_symbol_map.insert(2, ASTSymbol {id: 2, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(2)});
    ast_symbol_map.insert(3, ASTSymbol {id: 3, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(3)});
    ast_symbol_map.insert(4, ASTSymbol {id: 4, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(4)});

    //println!("before: {}", before_variable_propagation_ast.to_string(&ast_symbol_map));
    variable_propagation(&mut irs_ast);
    //println!("after: {}", irs_ast.to_string(&ast_symbol_map));

    let mut after_variable_propagation_ast = AbstractSyntaxTree::new();
    after_variable_propagation_ast.ast_type = ASTType::Begin(String::from("test"));

    return (*irs_ast.next.first().unwrap().clone(), ast_symbol_map);
    
    //assert_eq!(irs_ast, before_variable_propagation_ast);
}

*/

#[ignore = ""]
#[test]
fn test_condictions() {
    let mut ast_symbol_map = HashMap::<usize, ASTSymbol>::new();
    let mut if_ast = AbstractSyntaxTree::new();
    if_ast.ast_type = ASTType::If;
    
    let mut v1 = AbstractSyntaxTree::new();
    let mut v2 = AbstractSyntaxTree::new();
    let mut v3 = AbstractSyntaxTree::new();
    let mut v4 = AbstractSyntaxTree::new();
    let mut v5 = AbstractSyntaxTree::new();
    let mut v6 = AbstractSyntaxTree::new();

    ast_symbol_map.insert(1, ASTSymbol { id: 1, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(2, ASTSymbol { id: 2, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(3, ASTSymbol { id: 3, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(4, ASTSymbol { id: 4, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(5, ASTSymbol { id: 5, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(6, ASTSymbol { id: 6, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });

    v1.ast_type = ASTType::Variable(false);
    v1.value = 1;
    v2.ast_type = ASTType::Variable(false);
    v2.value = 2;
    v3.ast_type = ASTType::Variable(false);
    v3.value = 3;
    v4.ast_type = ASTType::Variable(false);
    v4.value = 4;
    v5.ast_type = ASTType::Variable(false);
    v5.value = 5;
    v6.ast_type = ASTType::Variable(false);
    v6.value = 6;


    let mut condictions = AbstractSyntaxTree::new();
    condictions.ast_type = ASTType::Condictions;

    let mut operand1 = AbstractSyntaxTree::new();
    operand1.ast_type = ASTType::Operator(Operator::And);

    let mut operand2 = AbstractSyntaxTree::new();
    operand2.ast_type = ASTType::Operator(Operator::Or);

    let mut condiction1 = AbstractSyntaxTree::new();
    condiction1.ast_type = ASTType::Condiction(Relation::L);

    let mut condiction2 = AbstractSyntaxTree::new();
    condiction2.ast_type = ASTType::Condiction(Relation::G);

    let mut condiction3 = AbstractSyntaxTree::new();
    condiction3.ast_type = ASTType::Condiction(Relation::EQ);

    condiction1.next.push(Box::new(v1));
    condiction1.next.push(Box::new(v2));

    condiction2.next.push(Box::new(v3));
    condiction2.next.push(Box::new(v4));

    condiction3.next.push(Box::new(v5));
    condiction3.next.push(Box::new(v6));
    
    operand2.next.push(Box::new(condiction2));
    operand2.next.push(Box::new(condiction3));

    operand1.next.push(Box::new(condiction1));
    operand1.next.push(Box::new(operand2));
    //operand1.next.push(Box::new(condiction2));

    condictions.next.push(Box::new(operand1));
    if_ast.next.push(Box::new(condictions));

    println!("{}", if_ast.to_string(&ast_symbol_map));
    panic!();
}

#[ignore = ""]
#[test]
fn test_condcition_to_condictions() {
    let mut ast_symbol_map = HashMap::<usize, ASTSymbol>::new();

    let mut if_ast1 = AbstractSyntaxTree::new();
    if_ast1.ast_type = ASTType::If;

    let mut if_ast2 = AbstractSyntaxTree::new();
    if_ast2.ast_type = ASTType::If;

    let mut condiction1 = AbstractSyntaxTree::new();
    condiction1.ast_type = ASTType::Condiction(Relation::EQ);

    let mut condiction2 = AbstractSyntaxTree::new();
    condiction2.ast_type = ASTType::Condiction(Relation::L);

    let mut true1 = AbstractSyntaxTree::new();
    true1.ast_type = ASTType::True;

    let mut v1 = AbstractSyntaxTree::new();
    v1.ast_type = ASTType::Variable(false);
    v1.value = 1;

    let mut v2 = AbstractSyntaxTree::new();
    v2.ast_type = ASTType::Variable(false);
    v2.value = 2;

    let mut v3 = AbstractSyntaxTree::new();
    v3.ast_type = ASTType::Variable(false);
    v3.value = 3;

    let mut v4 = AbstractSyntaxTree::new();
    v4.ast_type = ASTType::Variable(false);
    v4.value = 4;

    ast_symbol_map.insert(1, ASTSymbol { id: 1, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(2, ASTSymbol { id: 2, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(3, ASTSymbol { id: 3, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(4, ASTSymbol { id: 4, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });

    condiction1.next.push(Box::new(v1));
    condiction1.next.push(Box::new(v2));

    condiction2.next.push(Box::new(v3));
    condiction2.next.push(Box::new(v4));


    if_ast2.next.push(Box::new(condiction2));

    true1.next.push(Box::new(if_ast2));

    if_ast1.next.push(Box::new(condiction1));
    if_ast1.next.push(Box::new(true1));

    println!("before:\n{:#?}", if_ast1);
    println!("before:\n{}", if_ast1.to_string(&ast_symbol_map));
    optimization::optimization(&mut if_ast1, &ast_symbol_map);
    println!("after:\n{:#?}", if_ast1);
    println!("after:\n{}", if_ast1.to_string(&ast_symbol_map));

    panic!();
}


#[ignore = ""]
#[test]
fn test_branch_aggregation() {
    let mut ast_symbol_map = HashMap::<usize, ASTSymbol>::new();

    let mut if_ast1 = AbstractSyntaxTree::new();
    if_ast1.ast_type = ASTType::If;
    
    let mut if_ast2 = AbstractSyntaxTree::new();
    if_ast2.ast_type = ASTType::If;

    let mut if_ast3 = AbstractSyntaxTree::new();
    if_ast3.ast_type = ASTType::If;

    let mut condiction1 = AbstractSyntaxTree::new();
    condiction1.ast_type = ASTType::Condiction(Relation::L);

    let mut condiction2 = AbstractSyntaxTree::new();
    condiction2.ast_type = ASTType::Condiction(Relation::G);

    let mut condiction3 = AbstractSyntaxTree::new();
    condiction3.ast_type = ASTType::Condiction(Relation::EQ);

    let mut true_ast1 = AbstractSyntaxTree::new();
    true_ast1.ast_type = ASTType::True;

    let mut true_ast2 = AbstractSyntaxTree::new();
    true_ast2.ast_type = ASTType::True;

    let mut true_ast3 = AbstractSyntaxTree::new();
    true_ast3.ast_type = ASTType::True;

    let mut break_ast = AbstractSyntaxTree::new();
    break_ast.ast_type = ASTType::Break;

    let mut v1 = AbstractSyntaxTree::new();
    let mut v2 = AbstractSyntaxTree::new();
    let mut v3 = AbstractSyntaxTree::new();
    let mut v4 = AbstractSyntaxTree::new();
    let mut v5 = AbstractSyntaxTree::new();
    let mut v6 = AbstractSyntaxTree::new();

    ast_symbol_map.insert(1, ASTSymbol { id: 1, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(2, ASTSymbol { id: 2, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(3, ASTSymbol { id: 3, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(4, ASTSymbol { id: 4, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(5, ASTSymbol { id: 5, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(6, ASTSymbol { id: 6, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });

    v1.ast_type = ASTType::Variable(false);
    v1.value = 1;
    v2.ast_type = ASTType::Variable(false);
    v2.value = 2;
    v3.ast_type = ASTType::Variable(false);
    v3.value = 3;
    v4.ast_type = ASTType::Variable(false);
    v4.value = 4;
    v5.ast_type = ASTType::Variable(false);
    v5.value = 5;
    v6.ast_type = ASTType::Variable(false);
    v6.value = 6;
    
    condiction1.next.push(Box::new(v1));
    condiction1.next.push(Box::new(v2));

    condiction2.next.push(Box::new(v3));
    condiction2.next.push(Box::new(v4));

    condiction3.next.push(Box::new(v5));
    condiction3.next.push(Box::new(v6));

    if_ast1.next.push(Box::new(condiction1));
    if_ast2.next.push(Box::new(condiction2));
    if_ast3.next.push(Box::new(condiction3));

    true_ast3.next.push(Box::new(break_ast));
    if_ast3.next.push(Box::new(true_ast3));

    true_ast2.next.push(Box::new(if_ast3)); 
    if_ast2.next.push(Box::new(true_ast2));

    true_ast1.next.push(Box::new(if_ast2)); 
    if_ast1.next.push(Box::new(true_ast1));


    //println!("{:#?}", if_ast1);
    println!("before:\n{}", if_ast1.to_string(&ast_symbol_map));
    optimization::optimization(&mut if_ast1, &ast_symbol_map);    
    //optimization::optimization(&mut if_ast1, &ast_symbol_map);
    println!("after:\n{}", if_ast1.to_string(&ast_symbol_map));
    
    panic!();
}

#[test]
fn test_while() {    
}

#[ignore = ""]
#[test]
fn test_format_if() {
    let mut if_ast = AbstractSyntaxTree::new();
    if_ast.ast_type = ASTType::If;

    let mut true_ast = AbstractSyntaxTree::new();
    true_ast.ast_type = ASTType::True;

    let mut false_ast = AbstractSyntaxTree::new();
    false_ast.ast_type = ASTType::False;

    let mut condiction = AbstractSyntaxTree::new();
    condiction.ast_type = ASTType::Condiction(Relation::EQ);

    let mut break_ast = AbstractSyntaxTree::new();
    break_ast.ast_type = ASTType::Break;


    let mut v1 = AbstractSyntaxTree::new();
    v1.ast_type = ASTType::Variable(false);
    v1.value = 1;

    let mut v2 = AbstractSyntaxTree::new();
    v2.ast_type = ASTType::Variable(false);
    v2.value = 2;

    let mut ast_symbol_map = HashMap::<usize, ASTSymbol>::new();
    ast_symbol_map.insert(1, ASTSymbol { id: 1, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(2, ASTSymbol { id: 2, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });

    condiction.next.push(Box::new(v1));
    condiction.next.push(Box::new(v2));

    true_ast.next.push(Box::new(break_ast.clone()));
    false_ast.next.push(Box::new(break_ast));

    if_ast.next.push(Box::new(condiction));
    if_ast.next.push(Box::new(true_ast));
    if_ast.next.push(Box::new(false_ast));

    println!("before:\n{:#?}", if_ast);
    //optimization::format_condictions(&mut if_ast);
    println!("after:\n{:#?}", if_ast);

    panic!();
}

#[ignore = ""]
#[test]
fn test_foramt_while() {
    let mut begin_ast = AbstractSyntaxTree::new();

    let mut loop_ast = AbstractSyntaxTree::new();
    loop_ast.ast_type = ASTType::Loop;

    let mut if_ast = AbstractSyntaxTree::new();
    if_ast.ast_type = ASTType::If;

    let mut true_ast = AbstractSyntaxTree::new();
    true_ast.ast_type = ASTType::True;

    let mut false_ast = AbstractSyntaxTree::new();
    false_ast.ast_type = ASTType::False;

    let mut condiction_ast = AbstractSyntaxTree::new();
    condiction_ast.ast_type = ASTType::Condiction(Relation::EQ);

    let mut break_ast = AbstractSyntaxTree::new();
    break_ast.ast_type = ASTType::Break;

    let mut continue_ast = AbstractSyntaxTree::new();
    continue_ast.ast_type = ASTType::Continue;

    let mut assign_ast = AbstractSyntaxTree::new();
    assign_ast.ast_type = ASTType::Assign(false);
    assign_ast.value = 3;

    let mut v1 = AbstractSyntaxTree::new();
    v1.ast_type = ASTType::Variable(false);
    v1.value = 1;

    let mut v2 = AbstractSyntaxTree::new();
    v2.ast_type = ASTType::Variable(false);
    v2.value = 2;

    let mut v3 = AbstractSyntaxTree::new();
    v3.ast_type = ASTType::Variable(false);
    v3.value = 3;

    let mut if_ast1 = if_ast.clone();
    let mut true_ast1 = true_ast.clone();
    let mut false_ast1 = false_ast.clone();

    let mut ast_symbol_map = HashMap::<usize, ASTSymbol>::new();
    ast_symbol_map.insert(1, ASTSymbol { id: 1, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(2, ASTSymbol { id: 2, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });
    ast_symbol_map.insert(3, ASTSymbol { id: 3, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });

    condiction_ast.next.push(Box::new(v1));
    condiction_ast.next.push(Box::new(v3));

    true_ast.next.push(Box::new(continue_ast));
    false_ast.next.push(Box::new(break_ast));

    if_ast.next.push(Box::new(condiction_ast.clone()));
    if_ast.next.push(Box::new(true_ast));
    if_ast.next.push(Box::new(false_ast));

    if_ast1.next.push(Box::new(condiction_ast.clone()));
    if_ast1.next.push(Box::new(true_ast1));
    if_ast1.next.push(Box::new(false_ast1));


    assign_ast.next.push(Box::new(v2));
    
    //loop_ast.next.push(Box::new(assign_ast));
    loop_ast.next.push(Box::new(if_ast1));
    loop_ast.next.push(Box::new(if_ast));

    begin_ast.next.push(Box::new(loop_ast));

    println!("before:\n{}", begin_ast.to_string(&ast_symbol_map));
    optimization::optimization(&mut begin_ast, &ast_symbol_map);
    //optimization::format_condictions(&mut loop_ast);
    //optimization::format_while(&mut begin_ast);
    //optimization::format_while(&mut loop_ast);  
    //optimization::format_condictions(&mut loop_ast);
    //println!("{:#?}", loop_ast);
    println!("after:\n{}", begin_ast.to_string(&ast_symbol_map));

    panic!();
}

#[test]
fn test_constant_folding() {
    let mut before_constant_folding_ast = AbstractSyntaxTree::new();
    let mut after_constant_folding_ast = AbstractSyntaxTree::new();

    let mut after_constant_folding_ir = AbstractSyntaxTree::new();
    after_constant_folding_ir.ast_type = ASTType::Assign(false);

    let mut assign = AbstractSyntaxTree::new();
    assign.ast_type = ASTType::Assign(false); 
    let mut add = AbstractSyntaxTree::new();
    add.ast_type = ASTType::Operator(Operator::Add);

    let mut int_5 = AbstractSyntaxTree::new();
    int_5.ast_type = ASTType::Integer(true, ASTSymbolValueType::Int);
    int_5.value = 5;

    let mut int_7 = AbstractSyntaxTree::new();
    int_7.ast_type = ASTType::Integer(true, ASTSymbolValueType::Int);
    int_7.value = 7;

    add.next.push(Box::new(int_5));
    add.next.push(Box::new(int_7));

    assign.next.push(Box::new(add));

    before_constant_folding_ast.next.push(Box::new(assign));
    
    let mut int_12 = AbstractSyntaxTree::new();
    int_12.ast_type = ASTType::Integer(true, ASTSymbolValueType::Int);
    int_12.value = 12;

    after_constant_folding_ir.next.push(Box::new(int_12));
    after_constant_folding_ast.next.push(Box::new(after_constant_folding_ir));

    let mut ast_symbol_map = HashMap::<usize ,ASTSymbol>::new();
    ast_symbol_map.insert(0, ASTSymbol { id: 0, select_type: ASTSymbolValueType::Int, scope: Scope::Local, address: Address::GR(0) });


    println!("before:\n{}", before_constant_folding_ast.to_string(&ast_symbol_map));
    optimization(&mut before_constant_folding_ast, &ast_symbol_map);
    println!("after:\n{}", before_constant_folding_ast.to_string(&ast_symbol_map));
    println!("target:\n{}", after_constant_folding_ast.to_string(&ast_symbol_map));
    
    assert_eq!(before_constant_folding_ast, after_constant_folding_ast);
}

#[test]
fn test_operator_order() {
    let add = Operator::Add;
    let sub = Operator::Sub;
    let mul = Operator::Mul;
    let div = Operator::Div;

    assert_eq!(false, add < sub);
    assert_eq!(false, sub < add);
    assert_eq!(false, add > sub);
    assert_eq!(false, sub > add);

    assert_eq!(false, mul < div);
    assert_eq!(false, mul > div);
    assert_eq!(false, div < mul);
    assert_eq!(false, div > mul);

    assert_eq!(true, add < mul);
    assert_eq!(false, add > mul);
    assert_eq!(false, mul < add);
    assert_eq!(true, mul > add);

    assert_eq!(true, sub < mul);
    assert_eq!(false, add > div);
    assert_eq!(false, div < add);
    assert_eq!(true, mul > sub);

}

//#[ignore = ""]
#[test]
fn test_refine_assign_str_bracket() {
    let mut str1 = String::from("(1 + ((4 * 8) + (4 / 6)))");
    let mut str2 = String::from("(((1 + 9) * 5) + 7)");
    let mut str3 = String::from("(var1 + ((var2 * var3) + (var4 * var5)))");
    let mut str4 = String::from("(((var1 + var2) * var3) + var4)");
    let mut str5 = String::from("(var1 + ((4 * var2) + (4 * var3)))");
    let mut str6 = String::from("(((var1 + 9) * var2) + 7)");

    let str1_refined = String::from("1 + 4 * 8 + 4 / 6");
    let str2_refined = String::from("(1 + 9) * 5 + 7");
    let str3_refined = String::from("var1 + var2 * var3 + var4 / var5");
    let str4_refined = String::from("(var1 + var2) * var3 + var4");
    let str5_refined = String::from("var1 + 4 * var2 + 4 / var3");
    let str6_refined = String::from("(var1 + 9) * var2 + 7");

    refine_assign_str_bracket(&mut str1);
    refine_assign_str_bracket(&mut str2);
    refine_assign_str_bracket(&mut str3);
    refine_assign_str_bracket(&mut str4);
    refine_assign_str_bracket(&mut str5);
    refine_assign_str_bracket(&mut str6);

    assert_eq!(str1, str1_refined);
    assert_eq!(str2, str2_refined);
    assert_eq!(str3, str3_refined);
    assert_eq!(str4, str4_refined);
    assert_eq!(str5, str5_refined);
    assert_eq!(str6, str6_refined);

}


