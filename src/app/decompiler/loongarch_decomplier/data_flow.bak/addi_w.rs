use crate::loongarch_decomplier::*;

pub fn addi_w(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
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

    /*
    if insn.address == 0x120000860 {
        println!("{:?}", gr_states[index1]);
        println!("{:?}", gr_states[index2]);
        panic!("");
    }
    */


    if !gr_states[index2].state {
        let parameter = DFISymbolRecord {
            address: Address::GR(index2),
            sym_type: DFISymbolType::Param,
            id: symbol_table.param_counter.get(),
            size: Size::Signed32,
            value: false,
        };
        symbol_table.symbols.insert(parameter.clone());
        symbol_parameter.insert(parameter.clone());

        gr_states[index2].state = true;
        gr_states[index2].value = RegisterRecord::Symbol(parameter.clone());

        gr_states[index2].state = true;
        gr_states[index2].value = RegisterRecord::Symbol(parameter);
    }

    gr_states[index1].state = true;
    gr_states_parameter[index1].state = true;
    let tmp_symbol: Option<DFISymbolRecord> = match &gr_states[index2].value {
        RegisterRecord::Number(number) => {
            gr_states[index1].value = RegisterRecord::Number(Number::from(value + number.value, true, Size::Signed32));
            gr_states_parameter[index1].value = gr_states[index1].value.clone();
            None
        }

        RegisterRecord::Symbol(symbol2) => {
            match &symbol2.address {
                Address::GR(gr) => {
                    let tmp_symbol = DFISymbolRecord {
                        address: Address::GR(index1),
                        sym_type: symbol2.sym_type.clone(),
                        id: symbol_table.tmp_counter.get(),
                        size: symbol2.size.clone(),
                        value: false,
                    };

                    //gr_states[index1].value = RegisterRecord::Symbol(tmp_symbol.clone());

                    //gr_states_parameter[index1].value = gr_states[index1].value.clone();
                    //gr_states_parameter[index1].state = true;

                    let symbol = symbol2.clone();
                    let ir = DataFlowIr {
                        address: insn.address,
                        opcode: DataFlowIrOpcode::Add,
                        operand1: Some(DFIOperand::Symbol(tmp_symbol.clone())),
                        operand2: Some(DFIOperand::Symbol(symbol)),
                        operand3: Some(DFIOperand::Number(Number::from(value, true, Size::Signed32))),
                    };

                    irs.push(ir);
                    
                    Some(tmp_symbol)
                }
                Address::FR(fr) => {None}

                Address::Stack(stack) => {
                    /*
                    gr_states[index1].value = RegisterRecord::Symbol(DFISymbolRecord {
                        address: Address::Stack(stack + value),
                        sym_type: symbol.sym_type.clone(),
                        id: symbol.id,
                        size: symbol.size.clone(),
                    });
                    */

                    let sym = DFISymbolRecord {
                        address: Address::GR(index1),
                        sym_type: DFISymbolType::Temp,
                        id: symbol_table.tmp_counter.get(),
                        size: Size::Signed32,
                        value: false,
                    }; 

                    //gr_states[index1].state = true;
                    //gr_states[index1].value = RegisterRecord::Symbol(sym.clone());

                    let ir = DataFlowIr{
                        address: insn.address,
                        opcode: DataFlowIrOpcode::Add,
                        operand1: Some(DFIOperand::Symbol(sym.clone())),
                        operand2: Some(DFIOperand::Symbol(symbol2.clone())),
                        operand3: Some(DFIOperand::Number(Number::from(value, true, Size::Signed32))),
                    };

                    irs.push(ir);
                    nop = false;
                    Some(sym)
                }

                Address::Memory(memory) => {
                    if symbol2.value {
                        let number = Number::from(value, true, Size::Signed32);
                        let temp = DFISymbolRecord {
                            address: Address::GR(index1),
                            sym_type: DFISymbolType::Temp,
                            id: symbol_table.tmp_counter.get(),
                            size: Size::Signed32,
                            value: false,
                        };
                        let ir = DataFlowIr {
                            address: insn.address,
                            opcode: DataFlowIrOpcode::Add,
                            operand1: Some(DFIOperand::Symbol(temp.clone())),
                            operand2: Some(DFIOperand::Symbol(symbol2.clone())),
                            operand3: Some(DFIOperand::Number(number)),
                        };
                        irs.push(ir);
                        nop = false;
            
                        gr_states[index1].value = RegisterRecord::Symbol(temp);
                        gr_states_parameter[index1] = gr_states[index1].clone();
                    } else {
                        gr_states[index1].value = RegisterRecord::Symbol(DFISymbolRecord {
                            address: Address::Memory((*memory as isize + value) as usize),
                            sym_type: symbol2.sym_type.clone(),
                            id: symbol2.id,
                            size: symbol2.size.clone(),
                            value: false,
                        });
                    }
                    gr_states_parameter[index1].value = gr_states[index1].value.clone();
                    gr_states_parameter[index1].state = true;
                    None
                }
            }
        }    
    }; 

    if let Some(sym) = tmp_symbol {
        gr_states[index1].state = true;
        gr_states[index1].value = RegisterRecord::Symbol(sym);

        gr_states_parameter[index1].state = true;
        gr_states_parameter[index1].value = gr_states[index1].value.clone();

    }
    //println!("{:?}", gr_states[index1].value); 
    if nop {
        irs.push(DataFlowIr::nop(insn.address));
    }
}
