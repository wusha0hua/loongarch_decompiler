use super::*;

pub fn pcaddu12i(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
    let mut nop = true;
    let operand1 = match insn.operand1 {
        Some(operand) => operand,
        None => panic!("error"),
    };

    let operand2 = match insn.operand2 {
        Some(operand) => operand,
        None => panic!("error"),
    };

    let index1 = operand1.value as usize;
    let value = (operand2.value as i64) << 12;
    let address = insn.address as i64;

    gr_states[index1].state = true;
    gr_states[index1].value = RegisterRecord::Number(Number::from(address + value, false, Size::Unsigned64));

    gr_states_parameter[index1].state = true;
    gr_states_parameter[index1].value = gr_states[index1].value.clone();

    if nop {
        irs.push(DataFlowIr::nop(insn.address));
    }
}
