use super::*;

pub fn b(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
    let operand = match insn.operand1 {
        Some(operand) => operand,
        None => panic!("error"),
    };

    let offset = insn.address as i64 + (operand.value << 2) as i64;

    let ir = DataFlowIr {
        address: insn.address,
        opcode: DataFlowIrOpcode::Jmp,
        operand1: Some(DFIOperand::Number(Number::from(offset, false, Size::Unsigned64))),
        operand2: None,
        operand3: None,
    };



    irs.push(ir);
}
