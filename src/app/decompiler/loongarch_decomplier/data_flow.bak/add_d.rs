use crate::loongarch_decomplier::*;

pub fn add_d(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
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
    let index3 = operand3.value;

    /*
    if insn.address == 0x120000a98 {
        println!("{:?}", gr_states[index1]);
        println!("{:?}", gr_states[index2]);
        println!("{:?}", gr_states[index3]);
        panic!("print from add_d.rs");
    }
    */


    if !gr_states[index2].state {
        let parameter = DFISymbolRecord {
            address: Address::GR(index2),
            sym_type: DFISymbolType::Param,
            id: symbol_table.param_counter.get(),
            size: Size::Signed64,
            value: true,
        };
        symbol_table.symbols.insert(parameter.clone());
        symbol_parameter.insert(parameter.clone());

        gr_states[index2].state = true;

        gr_states_parameter[index2].state = true;
        gr_states_parameter[index2].value = RegisterRecord::Symbol(parameter);
    }


    if !gr_states[index3].state {
        let parameter = DFISymbolRecord {
            address: Address::GR(index3),
            sym_type: DFISymbolType::Param,
            id: symbol_table.param_counter.get(),
            size: Size::Signed64,
            value: false,
        };
        symbol_table.symbols.insert(parameter.clone());
        symbol_parameter.insert(parameter.clone());

        gr_states[index3].state = true;
        gr_states[index3].value = RegisterRecord::Symbol(parameter.clone());
        gr_states_parameter[index3].state = true;
        gr_states_parameter[index3].value = RegisterRecord::Symbol(parameter);
    }

    let sym1: Option<DFISymbolRecord> = match &gr_states[index2].value {
        RegisterRecord::Number(number2) => {
            match &gr_states[index3].value {
                RegisterRecord::Number(number3) => {
                    let number1 = RegisterRecord::Number(Number::from(number2.value + number3.value, number2.signed, number2.size.clone())); 
                    gr_states[index1].state = true;
                    gr_states[index1].value = number1.clone();
                    
                    gr_states_parameter[index1].state = true;
                    //gr_states_parameter[index1].value = number1;
                    None
                }

                RegisterRecord::Symbol(symbol3) => {None}
            }
        } 

        RegisterRecord::Symbol(symbol2) => {
            match &gr_states[index3].value {
                RegisterRecord::Number(number3) => {None}

                RegisterRecord::Symbol(symbol3) => {
                    let tmp_symbol = DFISymbolRecord{
                        address: Address::GR(index1),
                        sym_type: DFISymbolType::Temp,
                        id: symbol_table.tmp_counter.get(),
                        size: Size::Signed64,
                        value: false,
                    };

                    symbol_table.symbols.insert(tmp_symbol.clone());
                    //symbol_parameter.insert(tmp_symbol.clone());

                    let ir = DataFlowIr {
                        address: insn.address,
                        opcode: DataFlowIrOpcode::Add,
                        operand1: Some(DFIOperand::Symbol(tmp_symbol.clone())),
                        operand2: Some(DFIOperand::Symbol(symbol2.clone())),
                        operand3: Some(DFIOperand::Symbol(symbol3.clone())),
                    };
                    irs.push(ir);
                    nop = false;
                    
                    Some(tmp_symbol)
                }
            }
        }
    };

    if let Some(symbol) = sym1 {
        gr_states[index1].state = true;
        gr_states[index1].value = RegisterRecord::Symbol(symbol.clone());

        gr_states_parameter[index1].state = true;
        gr_states_parameter[index1].value = RegisterRecord::Symbol(symbol);
    }

    if nop {
        irs.push(DataFlowIr::nop(insn.address));
    }

    if insn.address == 0x1200006cc {
        panic!("{:#?}", gr_states[12]);
    }

}
