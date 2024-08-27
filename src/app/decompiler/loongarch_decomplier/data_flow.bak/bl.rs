use crate::loongarch_decomplier::*;

pub fn bl(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
    let mut i = 0;


    let operand = match insn.operand1 {
        Some(operand) => operand,
        None => panic!("error"),
    };

    let mut stack = match &gr_states[3].value {
        RegisterRecord::Symbol(symbol) => match &symbol.address {
            Address::Stack(stack) => *stack,
            _ => panic!("error"),
        }
        _ => panic!("error"),
    };

    let mut parameters = Vec::<RegisterRecord>::new();
    for i in 4..12 {
        if gr_states_parameter[i].state {

            parameters.push(gr_states_parameter[i].value.clone());
        } else {
            break;
        }
    } 
    /*
    if insn.address == 0x1200006fc {
        println!("{:#?}", parameters);
        panic!("print from bl.rs");
    }
    */
    /*
    if insn.address == 0x1200008f0 {
        for p in parameters.iter() {
            println!("{:?}", p);
        }
        panic!("print from bl.rs");
    }
    */
    
    let mut stacks = HashMap::<isize, DFISymbolRecord>::new();
    for symbol in symbol_parameter.iter() {
        if let Address::Stack(stack) = &symbol.address {
            stacks.insert(*stack, symbol.clone());
        }
    }
    /*
    let mut stack_param = HashMap::<isize, DFISymbolRecord>::new();
    for sym in symbol_parameter.iter() {
        if sym.sym_type == DFISymbolType::Param {
            if let Address::Stack(stack) = sym.address {
                let mut sym = sym.clone();
                sym.value = true;
                stack_param.insert(stack, sym);
            }
        }
    }
    if insn.address == 0x120000a08 {
        println!("{:#?}", stack_param);
        panic!("");
    }
    */

    //let mut stack_param = Vec::<DFISymbolRecord>::new();

    let mut gap = 0;
    loop {
        if let Some(sym) = stacks.get(&stack) {
            let mut sym = sym.clone();
            sym.value = true;
            parameters.push(RegisterRecord::Symbol(sym));
            //stack_param.push(stack);
            gap = 0;
        } else {
            gap += 1;
        } 

        stack += 1;
        if gap == 8 {
            break;
        }
    }



    /*
    for symbol in symbol_parameter.iter() {
        for stack_p in &stack_param {
            println!("{}", stack_p);
            if let Address::Stack(stack) = &symbol.address {
                if *stack == *stack_p {
                    parameters.push(RegisterRecord::Symbol(DFISymbolRecord {
                        address: symbol.address.clone(),
                        sym_type: symbol.sym_type.clone(),
                        id: symbol.id,
                        size: symbol.size.clone(),
                        value: true,
                    }));
                }
            }
        }
    }
    */


    let offset = insn.address as isize + (operand.value << 2) as isize;

    gr_states[4].state = true;
    let sym = DFISymbolRecord {
        address: Address::GR(4),
        sym_type: DFISymbolType::Return,
        id: symbol_table.return_counter.get(),
        size: Size::Signed64,
        value: true,
    };
    gr_states[4].value = RegisterRecord::Symbol(sym.clone());


    /*
    if insn.address == 0x120000a08 {
        println!("{:#?}", parameters);
        panic!("");
    }
    */

    /*
    if insn.address == 0x120000928 {
        println!("print from bl.rs: {:#?}", parameters);
        panic!("");
    }
    */

    let ir = DataFlowIr {
        address: insn.address,
        opcode: DataFlowIrOpcode::Call,
        operand1: Some(DFIOperand::Number(Number::from(offset, false, Size::Unsigned64))),
        operand2: Some(DFIOperand::Parameter(parameters)),
        operand3: Some(DFIOperand::Symbol(sym)),
    };

    irs.push(ir);
    symbol_parameter.clear();
    gr_states_parameter.clear();
    for i in 0..32 {
        gr_states_parameter.push(GRRecord::new());
    }
}
