use super::*;
pub fn st_d(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
    let mut nop = true;

    let operand1 = match insn.operand1 {
        Some(operand) => operand,
        None => panic!("error"),
    };

    let operand2 = match insn.operand2 {
        Some(operand) => operand,
        None => panic!("error"),
    };

    let operand3 = match insn.operand3 {
        Some(operand) => operand,
        None => panic!("error"),
    };

    let index1 = operand1.value as usize;
    let index2 = operand2.value as usize;
    let value = operand3.value as i64;


    if !gr_states[index2].state {
        panic!("")
    } 

    if index1 == 22 || index1 == 1{
        return;
    }

    if !gr_states[index1].state {
        let param_sym = DFISymbolRecord {
            address: Address::GR(index1),
            sym_type: DFISymbolType::Param,
            id: symbol_table.tmp_counter.get(),
            size: Size::Signed64,
            value: false,
        };

        gr_states[index1].state = true;
        gr_states[index1].value = RegisterRecord::Symbol(param_sym.clone());
        
        gr_states_parameter[index1] = gr_states[index1].clone();

        symbol_table.symbols.insert(param_sym.clone());
        symbol_parameter.insert(param_sym);
    }


    gr_states_parameter[index2] = gr_states[index2].clone();
    match &gr_states[index2].value {
        RegisterRecord::Symbol(symbol2) => {
            match &symbol2.address {
                Address::Stack(stack) => {
                    let mut symbol = DFISymbolRecord {
                        address: Address::Stack(stack + value),
                        sym_type: symbol2.sym_type.clone(),
                        id: symbol2.id,
                        size: symbol2.size.clone(),
                        value: true,
                    }; 


                    if let None = symbol_table.symbols.get(&symbol) {
                        symbol_table.symbols.insert(symbol.clone());
                    }
                    symbol_parameter.insert(symbol.clone());

                    if !gr_states[index1].state && *stack + value >= 0 {
                        let parameter = DFISymbolRecord {
                            address: Address::GR(index2),
                            sym_type: DFISymbolType::Param,
                            id: symbol_table.tmp_counter.get(),
                            size: Size::Signed64,
                            value: false,
                        }; 

                        gr_states[index1].state = true;
                        gr_states[index1].value = RegisterRecord::Symbol(parameter.clone());

                        gr_states_parameter[index1].state = true;
                        gr_states_parameter[index1].value = gr_states[index1].value.clone();

                        symbol_table.symbols.insert(parameter);
                    }
                    gr_states_parameter[index1] = gr_states[index1].clone();


                    match &gr_states[index1].value {
                        RegisterRecord::Number(number) => {
                            let mut sym =symbol.clone();
                            sym.value = true;
                            sym.size = Size::Signed64;
                            let ir = DataFlowIr {
                                address: insn.address,
                                opcode: DataFlowIrOpcode::Store,
                                operand1: Some(DFIOperand::Number(Number::from(number.value, true, Size::Signed64))),
                                operand2: Some(DFIOperand::Symbol(sym)),
                                operand3: None,
                            };

                            irs.push(ir);
                            nop = false;
                        }

                        RegisterRecord::Symbol(symbol1) => {
                            match &symbol1.address {
                                Address::Stack(stack) => {
                                    let mut sym2 = symbol.clone();
                                    let mut sym1 = symbol1.clone();

                                    //sym1.size = sym2.size.clone();
                                    //sym2.value = true;
                                    //sym2.size = Size::Signed64;
                                    

                                    let ir = DataFlowIr {
                                        address: insn.address,
                                        opcode: DataFlowIrOpcode::Store,
                                        operand1: Some(DFIOperand::Symbol(sym1)),
                                        operand2: Some(DFIOperand::Symbol(sym2)),
                                        operand3: None,
                                    };

                                    irs.push(ir);
                                    nop = false;
                                }
                                Address::Memory(memory) => {
                                    let mut sym2 = symbol.clone();
                                    let mut sym1 = symbol1.clone();

                                    sym1.value = true;
                                    sym1.size = Size::Unsigned64;
                                    sym2.value = true;
                                    sym2.size = Size::Signed64;

                                    let ir = DataFlowIr {
                                        address: insn.address,
                                        opcode: DataFlowIrOpcode::Store,
                                        operand1: Some(DFIOperand::Symbol(sym1)),
                                        operand2: Some(DFIOperand::Symbol(sym2)),
                                        operand3: None,
                                    };

                                    irs.push(ir);
                                    nop = false;
                                }
                                _  => {
                                    let mut sym = symbol.clone();
                                    sym.value = true;
                                    sym.size = Size::Signed64;
                                    let ir = DataFlowIr {
                                        address: insn.address,
                                        opcode: DataFlowIrOpcode::Store,
                                        operand1: Some(DFIOperand::Symbol(DFISymbolRecord {
                                            address: symbol1.address.clone(),
                                            sym_type: symbol1.sym_type.clone(),
                                            id: symbol1.id,
                                            size: Size::Signed64,
                                            value: symbol1.value,
                                        })),
                                        operand2: Some(DFIOperand::Symbol(sym)),
                                        operand3: None,
                                    };
                                    irs.push(ir);
                                    nop = false;
                                }
                            }
                        }

                    }
                }

                _ => {
                    panic!("")
                }
            }
        }

        RegisterRecord::Number(number) => {
            panic!("")
        }
    }

    /*
    if operand2.value == 3 {
        let stack_address = gr_states[3].value + operand3.value as isize;
        let symbol = DFISymbolRecord {
            address: Address::Stack(stack_address),
            sym_type: DFISymbolType::Local,
            id: symbol_table.loc_counter.get(),
            size: Size::Signed64,
            value: None,
        };

        let mut index = usize::MAX;
        for i in 0..symbol_table.symbols.len() {
            if symbol_table.symbols[i] == symbol {
                index = i;
                break;
            }
        }
        
        if index == usize::MAX {
            symbol_table.symbols.push(symbol);
        }
    }

    if operand1.value == 1 {
        
    }
    */
    if nop {
        irs.push(DataFlowIr::nop(insn.address));
    }
}
