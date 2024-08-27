use crate::loongarch_decomplier::*;

pub fn andi(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
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

    let index1 = operand1.value;
    let index2 = operand2.value;
    let value = operand3.value;

    if index1 == 0 {
        irs.push(DataFlowIr::nop(insn.address));
        return;
    } 

    if !gr_states[index1].state {
        let parameter = DFISymbolRecord {
            address: Address::GR(index1),
            sym_type: DFISymbolType::Param,
            id: symbol_table.param_counter.get(),
            size: Size::Signed64,
            value: false,
        };
        symbol_table.symbols.insert(parameter.clone());
        symbol_parameter.insert(parameter.clone());

        gr_states[index1].state = true;
        gr_states[index1].value = RegisterRecord::Symbol(parameter);

        gr_states_parameter[index1].state = true;
        gr_states_parameter[index1].value = gr_states[index1].value.clone();
    }

    if !gr_states[index2].state {
        let parameter = DFISymbolRecord {
            address: Address::GR(index2),
            sym_type: DFISymbolType::Param,
            id: symbol_table.param_counter.get(),
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


    match &gr_states[index2].value {
        RegisterRecord::Number(number2) => {
            let number1 = RegisterRecord::Number(Number::from(((number2.value as usize) & value) as isize, number2.signed, number2.size.clone())); 
            gr_states[index1].state = true;
            gr_states[index1].value = number1;

            gr_states_parameter[index1].state = true;
            gr_states_parameter[index1].value = gr_states[index1].value.clone();
        } 
        
        RegisterRecord::Symbol(symbol2) => {

        }
    }

    if nop {
        irs.push(DataFlowIr::nop(insn.address));
    }
}
