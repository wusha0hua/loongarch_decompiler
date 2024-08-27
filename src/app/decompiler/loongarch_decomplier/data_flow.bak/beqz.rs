use crate::loongarch_decomplier::*;

pub fn beqz(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
    let operand1 = match insn.operand1 {
        Some(operand) => operand,
        None => panic!("error"),
    };

    let operand2 = match insn.operand2 {
        Some(operand) => operand,
        None => panic!("error"),
    };


    let address = insn.address as isize + (operand2.value << 2) as isize;
    let index1 = operand1.value;

    let mut ir = DataFlowIr {
        address: insn.address,
        opcode: DataFlowIrOpcode::Jcc(Relation::EQ),
        operand1:None,
        operand2: Some(DFIOperand::Number(Number::from(0, false, Size::Unsigned64))),
        operand3: Some(DFIOperand::Number(Number::from(address, false, Size::Unsigned64))),
    };

    match &gr_states[index1].value {
        RegisterRecord::Number(number) => {
            let mut number = number.clone();
            ir.operand1 = Some(DFIOperand::Number(number));
        } 
        RegisterRecord::Symbol(symbol) => {
            let mut symbol = symbol.clone();
            ir.operand1 = Some(DFIOperand::Symbol(symbol));
        }
    }


    irs.push(ir);
}
