use super::*;

pub fn stptr_w(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
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
    let value = (operand3.value as i64) << 2;

    if !gr_states[index2].state {
        panic!("")
    } 


    match &gr_states[index2].value {
        RegisterRecord::Symbol(symbol2) => {
            match &symbol2.address {
                Address::Stack(stack) => {
                    let symbol = DFISymbolRecord {
                        address: Address::Stack(stack + value),
                        sym_type: symbol2.sym_type.clone(),
                        id: symbol2.id,
                        size: Size::Signed32,
                        value: true,
                    }; 

                    if let None = symbol_table.symbols.get(&symbol) {
                        symbol_table.symbols.insert(symbol.clone());
                        symbol_parameter.insert(symbol.clone());
                    }

                    if !gr_states[index1].state {
                        let parameter = DFISymbolRecord {
                            address: Address::GR(index2),
                            sym_type: DFISymbolType::Param,
                            id: symbol_table.tmp_counter.get(),
                            size: Size::Signed32,
                            value: false,
                        }; 

                        gr_states[index1].state = true;
                        gr_states[index1].value = RegisterRecord::Symbol(parameter.clone());

                        gr_states_parameter[index1].state = true;
                        gr_states_parameter[index1].value = gr_states[index1].value.clone();

                        symbol_table.symbols.insert(parameter.clone());
                        symbol_parameter.insert(parameter);
                    }

                    match &gr_states[index1].value {
                        RegisterRecord::Number(number) => {
                            let mut num = number.clone();
                            num.signed = true;
                            num.size = Size::Signed32;

                            let ir = DataFlowIr {
                                address: insn.address,
                                opcode: DataFlowIrOpcode::Store,
                                operand1: Some(DFIOperand::Number(num)),
                                operand2: Some(DFIOperand::Symbol(symbol)),
                                operand3: None,
                            };

                            irs.push(ir);
                            nop = false;
                        }

                        RegisterRecord::Symbol(symbol1) => {
                            match &symbol1.address {
                                Address::Stack(stack) => {
                                    let mut sym1 = symbol.clone();
                                    sym1.size = Size::Signed32;
                                    sym1.value = symbol1.value;

                                    let ir = DataFlowIr {
                                        address: insn.address,
                                        opcode: DataFlowIrOpcode::Store,
                                        operand1: Some(DFIOperand::Symbol(sym1)),
                                        operand2: Some(DFIOperand::Symbol(symbol)),
                                        operand3: None,
                                    };

                                    irs.push(ir);
                                    nop = false;
                                }

                                Address::Memory(memory) => {

                                }

                                Address::GR(gr) => {
                                    let mut sym1 = symbol1.clone();
                                    let ir = DataFlowIr {
                                        address: insn.address,
                                        opcode: DataFlowIrOpcode::Store,
                                        operand1: Some(DFIOperand::Symbol(sym1)),
                                        operand2: Some(DFIOperand::Symbol(symbol)),
                                        operand3: None,
                                    };

                                    irs.push(ir);
                                    nop = false;
                                }

                                Address::FR(fr) => {

                                }

                            }
                        }
                    }

                }

                Address::Memory(memory) => {
                    if symbol2.value {
                        if value == 0 {
                            match &gr_states[index1].value {
                                RegisterRecord::Symbol(symbol) => {}
                                RegisterRecord::Number(number) => {
                                    let ir = DataFlowIr {
                                        address: insn.address,
                                        opcode: DataFlowIrOpcode::Store,
                                        operand1: Some(DFIOperand::Number(number.clone())),
                                        operand2: Some(DFIOperand::Symbol(symbol2.clone())),
                                        operand3: None,
                                    };

                                    irs.push(ir);
                                    nop = false;
                                }
                            }
                        } else {

                        }
                    } 
                    /*
                    let mut new_symbol = symbol.clone();
                    new_symbol.address = Address::Memory((*memory as isize + value) as usize);
                    if let None = symbol_table.symbols.get(&new_symbol) {
                        new_symbol.id = symbol_table.global_counter.get();
                        symbol_table.symbols.insert(new_symbol.clone());
                    } else {
                        match &gr_states[index1].value {
                            RegisterRecord::Symbol(symbol) => {}
                            RegisterRecord::Number(number) => {
                                let ir = DataFlowIr {
                                    address: insn.address,
                                    opcode: DataFlowIrOpcode::Store,
                                    operand1: Some(DFIOperand::Number(number.clone())),
                                    operand2: Some(DFIOperand::Symbol(new_symbol.clone())),
                                    operand3: None,
                                };

                                irs.push(ir);
                            }
                        }
                    } 
                */
                }

                _ => {
                    match &gr_states[index1].value {
                        RegisterRecord::Number(number) => {
                            let mut num = number.clone();
                            num.size = Size::Signed32;
                            num.signed = true;

                            let mut sym = symbol2.clone();
                            sym.size = Size::Signed32;
                            sym.value = true;

                            if value == 0 {
                                let ir = DataFlowIr {
                                    address: insn.address,
                                    opcode: DataFlowIrOpcode::Store,
                                    operand1: Some(DFIOperand::Number(num)),
                                    operand2: Some(DFIOperand::Symbol(sym)),
                                    operand3: None,
                                };

                                irs.push(ir);
                                nop = false;
                            }

                        }

                        RegisterRecord::Symbol(symbol1) => {
                            let mut sym1 = symbol1.clone();
                            sym1.size = Size::Signed32;
                            //sym1.value = true;

                            let mut sym2 = symbol2.clone();
                            sym2.size = Size::Signed32;
                            sym2.value = true;

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

        RegisterRecord::Number(number) => {
            /*
            println!("{:x}", insn.address);
            panic!("print from stptr_w.rs")
            */
            let address = Address::Memory(number.value as u64);
            let mut id = usize::MAX;
            if !symbol_table.symbols.iter().any(|symbol| {
                if symbol.address == address {
                    id = symbol.id;
                }
                symbol.address == address
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

            match &gr_states[index1].value {
                RegisterRecord::Number(number1) => {
                    let ir = DataFlowIr {
                        address: insn.address,
                        opcode: DataFlowIrOpcode::Store,
                        operand1: Some(DFIOperand::Number(number1.clone())),
                        operand2: Some(DFIOperand::Symbol(global_sym)),
                        operand3: None,
                    };

                    irs.push(ir);
                    nop = false;
                }
                RegisterRecord::Symbol(symbol1) => {
                    let mut sym = symbol1.clone();
                    sym.size = Size::Signed32;
                    let ir = DataFlowIr {
                        address: insn.address,
                        opcode: DataFlowIrOpcode::Store,
                        operand1: Some(DFIOperand::Symbol(sym)),
                        operand2: Some(DFIOperand::Symbol(global_sym)),
                        operand3: None,
                    };

                    irs.push(ir);
                    nop = false;
                }
            }
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
