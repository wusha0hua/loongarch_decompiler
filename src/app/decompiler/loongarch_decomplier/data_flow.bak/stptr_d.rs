use crate::loongarch_decomplier::*;
pub fn stptr_d(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
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

    if index1 == 22 {
        return;
    }


    match &gr_states[index2].value {
        RegisterRecord::Symbol(symbol) => {
            match &symbol.address {
                Address::Stack(stack) => {
                    let symbol = DFISymbolRecord {
                        address: Address::Stack(stack + value),
                        sym_type: symbol.sym_type.clone(),
                        id: symbol.id,
                        size: symbol.size.clone(),
                        value: false,
                    }; 

                    if let None = symbol_table.symbols.get(&symbol) {
                        symbol_table.symbols.insert(symbol.clone());
                    }
                    symbol_parameter.insert(symbol.clone());

                    if !gr_states[index1].state {
                        let parameter = DFISymbolRecord {
                            address: Address::GR(index2),
                            sym_type: DFISymbolType::Param,
                            id: symbol_table.param_counter.get(),
                            size: Size::Signed64,
                            value: false,
                        }; 

                        gr_states[index1].state = true;
                        gr_states[index1].value = RegisterRecord::Symbol(parameter.clone());

                        gr_states_parameter[index1].state = true;
                        gr_states_parameter[index1].value = gr_states[index1].value.clone();

                        symbol_table.symbols.insert(parameter.clone());
                        symbol_parameter.insert(parameter.clone());

                        let ir = DataFlowIr {
                            address: insn.address,
                            opcode: DataFlowIrOpcode::Store,
                            operand1: Some(DFIOperand::Symbol(parameter)),
                            operand2: Some(DFIOperand::Symbol(symbol.clone())),
                            operand3: None,
                        };

                        irs.push(ir);
                        nop = false;
                    } 

                    match &gr_states[index1].value {
                        RegisterRecord::Number(number) => {
                            let ir = DataFlowIr {
                                address: insn.address,
                                opcode: DataFlowIrOpcode::Store,
                                operand1: Some(DFIOperand::Number(number.clone())),
                                operand2: Some(DFIOperand::Symbol(symbol.clone())),
                                operand3: None,
                            }; 
                            irs.push(ir);
                            nop = false;
                        } 

                        RegisterRecord::Symbol(symbol1) => {
                            let mut symbol = symbol.clone();
                            symbol.value = true;
                            let ir = DataFlowIr {
                                address: insn.address,
                                opcode: DataFlowIrOpcode::Store,
                                operand1: Some(DFIOperand::Symbol(symbol1.clone())),
                                operand2: Some(DFIOperand::Symbol(symbol)),
                                operand3: None,
                            };

                            /*
                            if insn.address == 0x120000bb0 {
                                println!("print from stptr.rs: \n{:#?}", symbol_parameter);
                                panic!("");
                            }
                            */
                            irs.push(ir);
                            nop = false;
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
    let mut symbol = DFISymbolRecord {
        address: Address::Memory(0),
        sym_type: DFISymbolType::Temp,
        id: 0,
        size: Size::Signed8,
        value: None,
    };

    if operand2.value == 3 {
        let stack_address = gr_states[3].value + ((operand3.value as isize) << 2);
        symbol = DFISymbolRecord {
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
    } else {

    }

    if operand1.value == 1 {
        
    } else if operand1.value == 22 {
        
    }
*/
    if nop {
        irs.push(DataFlowIr::nop(insn.address));
    }
}

