use super::*;

pub fn or(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
    let mut nop = true;
    let operand1 = match &insn.operand1 {
        Some(operand) => operand,
        None => panic!("error"),
    };

    let operand2 = match &insn.operand2 {
        Some(operand) => operand,
        None => panic!("error"),
    };

    let operand3 = match &insn.operand3 {
        Some(operand) => operand,
        None => panic!("error"),
    };

    let index1 = operand1.value as usize;
    let index2 = operand2.value as usize;
    let index3 = operand3.value as usize;


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

    if !gr_states[index3].state {
        let parameter = DFISymbolRecord {
            address: Address::GR(index3),
            sym_type: DFISymbolType::Param,
            id: symbol_table.tmp_counter.get(),
            size: Size::Signed64,
            value: false,
        };
        symbol_table.symbols.insert(parameter.clone());
        symbol_parameter.insert(parameter.clone());

        gr_states[index3].state = true;
        gr_states[index3].value = RegisterRecord::Symbol(parameter);

        gr_states_parameter[index3].state = true;
        gr_states_parameter[index3].value = gr_states[index3].value.clone();
    }

    if index3 == 0 {
        /*
        if insn.address == 0x12000079c {
            println!("{:?}", gr_states[index2]); 
            panic!("print from or.rs");
        }
        */
        if gr_states[index2].state {
            gr_states[index1].state = true;
            gr_states[index1].value = gr_states[index2].value.clone();

            gr_states_parameter[index1].state = true;
            gr_states_parameter[index1].value = gr_states[index2].value.clone();
        } else {
            let param_sym = DFISymbolRecord {
                address: Address::GR(index2),
                sym_type: DFISymbolType::Param,
                id: symbol_table.tmp_counter.get(),
                size: Size::Unsigned64,
                value: false,
            };

            gr_states[index2].state = true;
            gr_states[index2].value = RegisterRecord::Symbol(param_sym.clone());
            
            gr_states[index1].state = true;
            gr_states[index1].value = gr_states[index2].value.clone();

            gr_states_parameter[index1] = gr_states[index1].clone();
            gr_states_parameter[index2] = gr_states[index2].clone();
        }
        irs.push(DataFlowIr::nop(insn.address));
        return;
    }

    match &gr_states[index2].value {
        RegisterRecord::Number(number2) => {
            match &gr_states[index3].value {
                RegisterRecord::Number(number3) => {
                    let number1 = RegisterRecord::Number(Number::from(number2.value | number3.value, number2.signed, number2.size.clone())); 
                    gr_states[index1].state = true;
                    gr_states[index1].value = number1;

                    gr_states_parameter[index1].state = true;
                    gr_states_parameter[index1].value = gr_states[index1].value.clone();
                }

                RegisterRecord::Symbol(symbol3) => {

                }
            }
        } 

        RegisterRecord::Symbol(symbol2) => {

        }
    }

    if nop {
        irs.push(DataFlowIr::nop(insn.address));
    }

}
