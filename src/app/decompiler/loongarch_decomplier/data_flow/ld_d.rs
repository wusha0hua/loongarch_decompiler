use super::*;
pub fn ld_d(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
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
    /*
    if insn.address == 0x120000794 {
        println!("{:?}", gr_states[index2]);
        panic!("print from ld_d.rs");
    }
    */

    match &gr_states[index2].value {
        RegisterRecord::Number(number) => {
            let symbol = DFISymbolRecord {
                address: Address::Memory((number.value + value) as u64),
                sym_type: DFISymbolType::Global,
                id: symbol_table.global_counter.get(),
                size: Size::Signed64,
                value: true,
            };         

            if let None = &symbol_table.symbols.get(&symbol) {
                symbol_table.symbols.insert(symbol.clone());
                symbol_parameter.insert(symbol.clone());
            }

            gr_states[index1].state = true;
            gr_states[index1].value = RegisterRecord::Symbol(symbol.clone());

            gr_states_parameter[index1].state = true;
            gr_states_parameter[index1].value = gr_states[index1].value.clone();
        }

        RegisterRecord::Symbol(symbol) => {
            match &symbol.address {
                Address::Stack(stack) => {
                    /*if !symbol.value*/ {
                        let mut sym = DFISymbolRecord {
                            address: Address::Stack(stack + value),
                            sym_type: DFISymbolType::Local,
                            id: 0,
                            size: Size::Signed64,
                            value: true,
                        };
                        /*
                        if let None = symbol_table.symbols.get(&sym) {
                            symbol_table.symbols.insert(sym.clone());
                            symbol_parameter.insert(sym.clone());
                        } else {
                            if insn.address == 0x1200006d4 {
                                panic!("{:?}", symbol_table.symbols.get(&sym));
                            }
                        }
                        */
                        let mut sym = match symbol_table.symbols.get(&sym) {
                            Some(symbol) => {
                                if insn.address == 0x1200006d4 {
                                }
                                symbol.clone()
                            }
                            None => sym,
                        };

                        gr_states[index1].state = true;
                        gr_states[index1].value = RegisterRecord::Symbol(sym);

                        gr_states_parameter[index1].state = true;
                        gr_states_parameter[index1].value = gr_states[index1].value.clone();
                    } /*else {
                        /*panic!();*/
                    }*/
                    /*
                    if !symbol.value {
                        let sym = DFISymbolRecord {
                            address: Address::Stack(stack + value),
                            sym_type: DFISymbolType::Local,
                            id: 0,
                            size: Size::Signed64,
                            value: true,
                        };

                        if let None = symbol_table.symbols.get(&sym) {
                            symbol_table.symbols.insert(sym.clone());
                            symbol_parameter.insert(sym.clone());
                        }

                        gr_states[index1].state = true;
                        gr_states[index1].value = RegisterRecord::Symbol(sym);

                        gr_states_parameter[index1].state = true;
                        gr_states_parameter[index1].value = gr_states[index1].value.clone();
                    } else {
                    */
                    /*
                        let temp_sym = DFISymbolRecord {
                            address: Address::GR(index1),
                            sym_type: DFISymbolType::Temp,
                            id: symbol_table.tmp_counter.get(),
                            size: Size::Signed64,
                            value: false,
                        }; 
                        
                        if let None = symbol_table.symbols.get(&temp_sym) {
                            symbol_table.symbols.insert(temp_sym.clone());
                            symbol_parameter.insert(temp_sym.clone());
                        }

                        let mut sym = symbol.clone();
                        sym.value = true;
                        sym.size = Size::Signed64;
                        sym.address = Address::Stack(stack + value);

                        let ir = DataFlowIr {
                            address: insn.address,
                            opcode: DataFlowIrOpcode::Load,
                            operand1: Some(DFIOperand::Symbol(temp_sym.clone())),
                            operand2: Some(DFIOperand::Symbol(sym)),
                            operand3: None,
                        };
                        
                        irs.push(ir);

                        gr_states[index1].state = true;
                        gr_states[index1].value = RegisterRecord::Symbol(temp_sym);

                        gr_states_parameter[index1].state = true;
                        gr_states_parameter[index1].value = gr_states[index1].value.clone();

                        nop = false;
                    //}
                    */
                }

                Address::Memory(memory) => {

                }

                _ => {
                    if !symbol.value {
                        let mut temp = DFISymbolRecord {
                            address: Address::GR(index1),
                            sym_type: DFISymbolType::Temp,
                            id: symbol_table.tmp_counter.get(),
                            size: Size::Unsigned64,
                            value: false,
                        };
                        let ir = DataFlowIr {
                            address: insn.address,
                            opcode: DataFlowIrOpcode::Add,
                            operand1: Some(DFIOperand::Symbol(temp.clone())),
                            operand2: Some(DFIOperand::Symbol(symbol.clone())),
                            operand3: Some(DFIOperand::Number(Number::from(value, true, Size::Signed64))),
                        };

                        irs.push(ir);
                        nop = false;

                        temp.value = true;
                        temp.size = Size::Signed64;
                        gr_states[index1].state = true;
                        gr_states[index1].value = RegisterRecord::Symbol(temp);

                    } else {
                        panic!("print from ld_d.rs");
                    }
                    /*
                    if insn.address == 0x120000794 {
                        println!("{:?}", gr_states[index2]);
                        panic!("print from ld_d.rs");
                    }         
                    */
                }
            }
        }
    }
    if nop {
        irs.push(DataFlowIr::nop(insn.address));
    }
}
        /*
        RegisterRecord::Symbol(symbol) => {
            match &symbol.address {
                Address::Stack(stack) => {
                    let symbol = DFISymbolRecord {
                        address: Address::Stack(stack + value),
                        sym_type: symbol.sym_type.clone(),
                        id: symbol.id,
                        size: symbol.size.clone(),
                    }; 

                    if let None = symbol_table.symbols.get(&symbol) {
                        symbol_table.symbols.insert(symbol.clone());
                    }

                    if !gr_states[index1].state {
                        let parameter = DFISymbolRecord {
                            address: Address::GR(index2),
                            sym_type: DFISymbolType::Param,
                            id: symbol_table.param_counter.get(),
                            size: Size::Signed64,
                        }; 

                        gr_states[index1].state = true;
                        gr_states[index1].value = RegisterRecord::Symbol(parameter.clone());
                    }
                    */
