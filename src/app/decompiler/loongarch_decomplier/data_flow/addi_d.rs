use super::*;

pub fn addi_d(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
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

    let num = Number::from(value, true, Size::Signed64);

    if !gr_states[index2].state {
        let parameter = DFISymbolRecord {
            address: Address::GR(index2),
            sym_type: DFISymbolType::Param,
            id: symbol_table.tmp_counter.get(),
            size: Size::Signed64,
            value: false,
        };
        symbol_table.symbols.insert(parameter.clone());
        symbol_parameter.insert(parameter.clone());

        gr_states[index2].state = true;
        gr_states[index2].value = RegisterRecord::Symbol(parameter);

        gr_states_parameter[index2].state = true;
        gr_states_parameter[index2].value = gr_states[index2].value.clone();
    }

    gr_states[index1].state = true;
    gr_states_parameter[index1].state = true;
    let sym: Option<RegisterRecord> = match &gr_states[index2].value {
        RegisterRecord::Number(number) => {
            gr_states[index1].value = RegisterRecord::Number(Number::from(value + number.value, true, Size::Signed64));
            gr_states_parameter[index1].value = gr_states[index1].value.clone();
            None
        }

        RegisterRecord::Symbol(symbol) => {
            match &symbol.address {
                Address::GR(gr) => {
                    let temp_sym = DFISymbolRecord {
                        address: Address::GR(index1),
                        sym_type: DFISymbolType::Temp,
                        id: symbol_table.tmp_counter.get(),
                        size: Size::Signed64,
                        value: false,
                    };


                    let ir = DataFlowIr {
                        address: insn.address,
                        opcode: DataFlowIrOpcode::Add,
                        operand1: Some(DFIOperand::Symbol(temp_sym.clone())),
                        operand2: Some(DFIOperand::Symbol(symbol.clone())),
                        operand3: Some(DFIOperand::Number(Number::from(value, true, Size::Signed64))),
                    };
                    irs.push(ir);
                    nop = false;

                    gr_states_parameter[index1].value = gr_states[index1].value.clone();
                    Some(RegisterRecord::Symbol(temp_sym))
                }

                Address::FR(fr) => {None}

                Address::Stack(stack) => {
                    let mut symbol = symbol.clone();
                    symbol.size = Size::Signed64;
                    if symbol.value {
                        let tmp_symbol = DFISymbolRecord{
                            address: Address::GR(index1),
                            sym_type: DFISymbolType::Temp,
                            id: symbol_table.tmp_counter.get(),
                            size: Size::Signed64,
                            value: false,
                        };

                        let ir = DataFlowIr {
                            address: insn.address,
                            opcode: DataFlowIrOpcode::Add,
                            operand1: Some(DFIOperand::Symbol(tmp_symbol.clone())),
                            operand2: Some(DFIOperand::Symbol(symbol.clone())),
                            operand3: Some(DFIOperand::Number(num)),
                        };

                        irs.push(ir);
                        nop = false;
                        Some(RegisterRecord::Symbol(tmp_symbol))
                        /*
                        gr_states[index1].value = RegisterRecord::Symbol(DFISymbolRecord {
                            address: Address::Stack(stack + value),
                            sym_type: symbol.sym_type.clone(),
                            id: symbol.id,
                            size: symbol.size.clone(),
                            value: symbol.value,
                        });
                        */
                    } else {
                        gr_states[index1].value = RegisterRecord::Symbol(DFISymbolRecord {
                            address: Address::Stack(stack + value),
                            sym_type: symbol.sym_type.clone(),
                            id: symbol.id,
                            size: symbol.size.clone(),
                            value: symbol.value,
                        });


                        gr_states_parameter[index1].value = gr_states[index1].value.clone();
                        None
                    }
                }

                Address::Memory(memory) => {
                    gr_states[index1].value = RegisterRecord::Symbol(DFISymbolRecord {
                        address: Address::Memory((*memory as i64 + value) as u64),
                        sym_type: symbol.sym_type.clone(),
                        id: symbol.id,
                        size: symbol.size.clone(),
                        value: false
                    });

                    gr_states_parameter[index1].value = gr_states[index1].value.clone();
                    None
                }
            }
        }    
    }; 

    if let Some(sym) = sym {
        gr_states[index1].state = true;
        gr_states[index1].value = sym;

        gr_states_parameter[index1].state = true;
        gr_states_parameter[index1].value = gr_states[index1].value.clone();
    }
    /*
    if insn.address == 0x1200006cc {
        panic!("{:?}", gr_states[index1]);
    }
    */
    //println!("{:?}", gr_states[index1].value); 
    if nop {
        irs.push(DataFlowIr::nop(insn.address));
    }
}
