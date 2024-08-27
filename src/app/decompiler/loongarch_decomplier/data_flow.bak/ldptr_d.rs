use crate::loongarch_decomplier::*;
pub fn ldptr_d(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
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
            let symbol = DFISymbolRecord {
                address: Address::Memory((number.value + value) as usize),
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
