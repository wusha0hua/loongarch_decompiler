use super::*;

pub fn slli_w(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
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

    if value == 0 {
        gr_states[index1].state = true;
        gr_states_parameter[index1].state = true;
        match &gr_states[index2].value {
            RegisterRecord::Number(number) => {
                gr_states[index1].value = RegisterRecord::Number(Number::from(number.value, true, Size::Signed32));
                gr_states_parameter[index1].value = gr_states[index1].value.clone();
            }
            RegisterRecord::Symbol(symbol) => {
                gr_states[index1].value = RegisterRecord::Symbol(DFISymbolRecord {
                    address: symbol.address.clone(),
                    sym_type: symbol.sym_type.clone(),
                    id: symbol.id,
                    size: Size::Signed32,
                    value: false,
                });

                gr_states_parameter[index1].value = gr_states[index1].value.clone();
            }
        }


    } else {
        match &gr_states[index2].value {
            RegisterRecord::Number(number2) => {}
            RegisterRecord::Symbol(symbol2) => {
                let sym = DFISymbolRecord {
                    address: Address::GR(index1),
                    sym_type: DFISymbolType::Temp,
                    id: symbol_table.tmp_counter.get(),
                    size: Size::Signed32,
                    value: false,
                };

                let ir = DataFlowIr {
                    address: insn.address,
                    opcode: DataFlowIrOpcode::Mul,
                    operand1: Some(DFIOperand::Symbol(sym.clone())),
                    operand2: Some(DFIOperand::Symbol(symbol2.clone())),
                    operand3: Some(DFIOperand::Number(Number::from(1 << value, true, Size::Signed32))),
                };

                irs.push(ir);
                nop = false;

                gr_states[index1].state = true;
                gr_states[index1].value = RegisterRecord::Symbol(sym);

                gr_states_parameter[index1].state = true;
                gr_states_parameter[index1].value = gr_states[index1].value.clone();
            }
        }
    }


    if nop {
        irs.push(DataFlowIr::nop(insn.address));
    }
}
