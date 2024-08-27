use crate::loongarch_decomplier::*;
pub fn ldptr_w(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
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
    let value = (operand3.value as isize) << 2;

    
    if !gr_states[index2].state {
        panic!("")
    } 


    match &gr_states[index2].value {
        RegisterRecord::Number(number) => {
            /*
            let symbol = DFISymbolRecord {
                address: Address::Memory((number.value + value) as usize),
                sym_type: DFISymbolType::Global,
                id: symbol_table.global_counter.get(),
                size: Size::Signed32,
                value: true,
            };         
            */

            /*
            if let None = &symbol_table.symbols.get(&symbol) {
                symbol_table.symbols.insert(symbol.clone());
                symbol_parameter.insert(symbol.clone());
            }
            */

            let address = Address::Memory((number.value + value) as usize);
            let mut id = usize::MAX;
            let mut flag = true;
            if !symbol_table.symbols.iter().any(|sym| {
                if sym.address == address {
                    id = sym.id;
                    flag = false;
                }
                sym.address == address
            }) {
                id = symbol_table.global_counter.get();
            }

            let global_sym = DFISymbolRecord {
                address,
                sym_type: DFISymbolType::Global,
                id,
                size: Size::Signed32,
                value: true,
            };

            if flag {
                symbol_table.symbols.insert(global_sym.clone());
            }

            gr_states[index1].state = true;
            gr_states[index1].value = RegisterRecord::Symbol(global_sym.clone());

            gr_states_parameter[index1].state = true;
            gr_states_parameter[index1].value = gr_states[index1].value.clone();


        }

        RegisterRecord::Symbol(symbol2) => {
            match &symbol2.address {
                Address::Stack(stack) => {
                    let sym = DFISymbolRecord {
                        address: Address::Stack(stack + value),
                        sym_type: symbol2.sym_type.clone(),
                        id: 0,
                        size: symbol2.size.clone(),
                        value: true,
                    };


                    //if let None = symbol_table.symbols.get(&sym) {
                    if !symbol_table.symbols.iter().any(|x| x.address == sym.address) && (*stack + value) >= 0 {
                        let mut param_sym = sym.clone();
                        param_sym.sym_type = DFISymbolType::Param;
                        param_sym.id = ((*stack + value) as usize / 8) + 8;//symbol_table.param_counter.get();
                        symbol_table.symbols.insert(param_sym.clone());
                        symbol_parameter.insert(param_sym.clone());
                        gr_states[index1].state = true;
                        gr_states[index1].value = RegisterRecord::Symbol(param_sym);
                        gr_states_parameter[index1].state = true;
                        gr_states_parameter[index1].value = gr_states[index1].value.clone();
                        /*
                        if insn.address == 0x1200009dc {
                            println!("print from ldptr_w.rs: param: \n{:?}", gr_states[index1]);
                            panic!("");
                        }
                        */
                   } else {
                        let mut temp_sym = DFISymbolRecord {
                            address: Address::GR(0),
                            sym_type: DFISymbolType::Temp,
                            id: symbol_table.tmp_counter.get(),
                            size: Size::Signed32,
                            value: false,
                        };

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
                        /*
                        if insn.address == 0x1200009dc {
                            println!("print from ldptr_w.rs: local\n{:?}", gr_states[index1]);
                            panic!("");
                        }
                        */

                    }
                }

                Address::Memory(memory) => {}

                _ => {
                    if !symbol2.value {
                        if value == 0 {
                            let mut sym = symbol2.clone();
                            sym.value = true;
                            sym.size = Size::Signed32;
                            sym.sym_type = DFISymbolType::Temp;

                            let temp_sym = DFISymbolRecord {
                                address: Address::GR(index1),
                                sym_type: DFISymbolType::Temp,
                                id: symbol_table.tmp_counter.get(),
                                size: Size::Signed32,
                                value: false,
                            };

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
                            gr_states_parameter[index1] = gr_states[index1].clone();
                            nop = false;
                        } else {

                        }
                    } else {
                        if value == 0 {
                            nop = false;
                            
                            let mut sym = symbol2.clone();
                            sym.value = false;                     
                            sym.size = Size::Signed32;
                            sym.sym_type = DFISymbolType::Temp;
                            sym.id = symbol_table.tmp_counter.get();


                            let ir = DataFlowIr {
                                address: insn.address,
                                opcode: DataFlowIrOpcode::Load,
                                operand1: Some(DFIOperand::Symbol(sym.clone())),
                                operand2: Some(DFIOperand::Symbol(symbol2.clone())),
                                operand3: None,
                            };
                            gr_states[index1].state = true;
                            gr_states[index1].value = RegisterRecord::Symbol(sym);
                            gr_states_parameter[index1] = gr_states[index1].clone();
                            nop = false;
                            irs.push(ir);
                        }
                    }

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
