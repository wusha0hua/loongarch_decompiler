use crate::loongarch_decomplier::*;
pub fn st_w(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
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

    let index1 = operand1.value;
    let index2 = operand2.value;
    let value = operand3.value as isize;

    if !gr_states[index2].state {
        panic!("")
    } 

    if !gr_states[index1].state {
        let parameter = DFISymbolRecord {
            address: Address::GR(index2),
            sym_type: DFISymbolType::Param,
            id: symbol_table.param_counter.get(),
            size: Size::Signed32,
            value: false,
        }; 
    
        gr_states[index1].state = true;
        gr_states[index1].value = RegisterRecord::Symbol(parameter.clone());

        gr_states[index1].state = true;
        gr_states[index1].value = gr_states[index1].value.clone();
    
        symbol_table.symbols.insert(parameter.clone());
        symbol_parameter.insert(parameter);
    }

    match &gr_states[index2].value {
        RegisterRecord::Symbol(symbol2) => {
            match &symbol2.address {
                Address::Stack(stack) => {
                    let symbol = DFISymbolRecord {
                        address: Address::Stack(stack + value),
                        sym_type: symbol2.sym_type.clone(),
                        id: symbol2.id,
                        size: symbol2.size.clone(),
                        value: symbol2.value,
                    }; 

                    if let None = symbol_table.symbols.get(&symbol) {
                        symbol_table.symbols.insert(symbol.clone());
                        symbol_parameter.insert(symbol.clone());
                    }

                    match &gr_states[index1].value {
                        RegisterRecord::Number(number) => {
                            let mut sym = symbol.clone();
                            sym.value = true;
                            sym.size = Size::Signed32;
                            let ir = DataFlowIr {
                                address: insn.address,
                                opcode: DataFlowIrOpcode::Store,
                                operand1: Some(DFIOperand::Number(Number::from(number.value, true, Size::Signed32))),
                                operand2: Some(DFIOperand::Symbol(sym)),
                                operand3: None,
                            };

                            irs.push(ir);
                            nop = false;
                        }

                        RegisterRecord::Symbol(symbol1) => {
                            let mut sym2 = symbol.clone();
                            let mut sym1 = symbol1.clone();

                            sym1.size = Size::Signed32;
                            sym1.value = false;

                            sym2.value = true;
                            sym2.size = Size::Signed32;
                            match &symbol1.address {
                                Address::Stack(stack) => {
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
                                Address::Memory(memory) => {}
                                _  => {
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
                            }
                        }
                    }
                }

                Address::Memory(memory) => {
                    panic!("from st_w.rs : Address::Memory(memory)");
                }

                _ => {
                    let sym_tmp = DFISymbolRecord {
                        address: Address::GR(usize::MAX),
                        sym_type: DFISymbolType::Temp,
                        id :symbol_table.tmp_counter.get(),
                        size: Size::Unsigned64,
                        value: symbol2.value,
                    };

                    let ir = DataFlowIr {
                        address: insn.address - 1,
                        opcode: DataFlowIrOpcode::Add,
                        operand1: Some(DFIOperand::Symbol(sym_tmp.clone())),
                        operand2: Some(DFIOperand::Symbol(symbol2.clone())),
                        operand3: Some(DFIOperand::Number(Number::from(value, true, Size::Signed64))),
                    };

                    irs.push(ir);
                    nop = false;

                    match &gr_states[index1].value {
                        RegisterRecord::Number(number1) => {
                            let mut number = number1.clone();
                            number.size = Size::Signed32;
                            number.signed = true;

                            let ir = DataFlowIr {
                                address: insn.address,
                                opcode: DataFlowIrOpcode::Store,
                                operand1: Some(DFIOperand::Number(number)),
                                operand2: Some(DFIOperand::Symbol(sym_tmp)),
                                operand3: None,
                            };

                            irs.push(ir);
                            nop = false;
                        }

                        RegisterRecord::Symbol(symbol1) => {
                            let mut symbol = symbol1.clone();
                            symbol.value = true;
                            symbol.size = Size::Signed32;

                            let mut sym_tmp = sym_tmp.clone();
                            sym_tmp.size = Size::Signed32;
                            sym_tmp.value = true;
                            
                            let ir = DataFlowIr {
                                address: insn.address,
                                opcode: DataFlowIrOpcode::Store,
                                operand1: Some(DFIOperand::Symbol(symbol)),
                                operand2: Some(DFIOperand::Symbol(sym_tmp)),
                                operand3: None,
                            }; 

                            irs.push(ir);
                            nop = false;

                        }
                    }

                    /*
                    println!("print from st_w.rs");
                    println!("--------------------------------------");
                    println!("{:x}: \n{:?}\n{:?}\n{:?}", insn.address, gr_states[index1], gr_states[index2], operand3);
                    println!("---------------------------------------");
                    panic!("")
                    */
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
