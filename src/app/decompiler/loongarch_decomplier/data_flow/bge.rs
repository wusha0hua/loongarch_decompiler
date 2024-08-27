use super::*;

pub fn bge(insn: AssemblyInstruction, irs: &mut Vec<DataFlowIr>, gr_states: &mut Vec<GRRecord>, symbol_table: &mut DFISymbolRecordTable, gr_states_parameter: &mut Vec<GRRecord>, symbol_parameter: &mut HashSet<DFISymbolRecord>) {
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

    let address = insn.address as i64 + (operand3.value << 2) as i64;
    let index1 = operand1.value as usize;
    let index2 = operand2.value as usize;

    let mut ir = DataFlowIr {
        address: insn.address,
        opcode: DataFlowIrOpcode::Jcc(Relation::GE),
        operand1:None,
        operand2: None,
        operand3: Some(DFIOperand::Number(Number::from(address, false, Size::Unsigned64))),
    };

    match &gr_states[index1].value {
        RegisterRecord::Number(number) => {
            let mut number = number.clone();
            number.size = set_signed(number.size);
            number.signed = true;
            ir.operand1 = Some(DFIOperand::Number(number));
        } 
        RegisterRecord::Symbol(symbol) => {
            let mut symbol = symbol.clone();
            symbol.size = set_signed(symbol.size);
            ir.operand1 = Some(DFIOperand::Symbol(symbol));
        }
    }

    match &gr_states[index2].value {
        RegisterRecord::Number(number) => {
            let mut number = number.clone();
            number.size = set_signed(number.size);
            number.signed = true;
            ir.operand2 = Some(DFIOperand::Number(number));
        }
        RegisterRecord::Symbol(symbol) => {
            let mut symbol = symbol.clone();
            symbol.size = set_signed(symbol.size);
            ir.operand2 = Some(DFIOperand::Symbol(symbol));
        }
    }

    irs.push(ir);
}

fn set_signed(size: Size) -> Size {
    match size {
        Size::Unsigned8 => Size::Signed8,
        Size::Unsigned16 => Size::Signed16,
        Size::Unsigned32 => Size::Signed32,
        Size::Unsigned64 => Size::Signed64,
        _ => size,
    }
}
